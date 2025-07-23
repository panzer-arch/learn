use std::{
    collections::HashMap,
    net::{SocketAddr, ToSocketAddrs},
    sync::Arc,
};
use anyhow::{Error, Result, anyhow, bail};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use rustls::pki_types::ServerName;
use tokio_rustls::{rustls, TlsConnector};

pub struct URL {
    pub url: String,
    pub scheme: String,
    pub hostname: String,
    pub path: String,
    pub port: u16,
}

impl URL {
    pub async fn request(&self) -> Result<String> {
        let host = format!("{}:{}", self.hostname, self.port);
        let request = format!("GET {} HTTP/1.0\r\nHost: {}\r\n\r\n", self.path, host);
        let mut response = String::new();
        if self.scheme == "https" {
            let mut root_cert_store = rustls::RootCertStore::empty();
            root_cert_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
            let config = rustls::ClientConfig::builder()
              .with_root_certificates(root_cert_store)
              .with_no_client_auth();
            let stream = TcpStream::connect(host.clone()).await?;
            let connector = TlsConnector::from(Arc::new(config));
            let domain = ServerName::try_from(self.hostname.clone())?;
            let mut stream = connector.connect(domain, stream).await?;
            stream.write_all(request.as_bytes()).await?;
            stream.get_ref().0.readable().await?;
            stream.read_to_string(&mut response).await?;
        } else {
            let mut stream = TcpStream::connect(host.clone()).await?;
            stream.write_all(request.as_bytes()).await?;
            stream.readable().await?;
            stream.read_to_string(&mut response).await?;
        }
        let lines = response.lines().collect::<Vec<&str>>();
        let mut line_iter = lines.iter();
        let status_line = line_iter.next();
        if let Some(status) = status_line {
            let status_parts: Vec<&str> = status.split_whitespace().collect();
            if status_parts.len() < 2 {
                bail!("Invalid response status line: {}", status);
            }
        } else {
            bail!("No response from server");
        }
        let mut response_headers = HashMap::new();
        loop {
            match line_iter.next() {
                Some(line) => {
                    if line.is_empty() {
                        break; // End of headers
                    }
                    let split_line = line.splitn(2, ":").collect::<Vec<&str>>();
                    println!("Header: {}", split_line.join(":"));
                    let key = split_line.get(0).ok_or(anyhow!("Invalid header format"))?;
                    let value = split_line.get(1).ok_or(anyhow!("Invalid header format"))?;
                    response_headers.insert(key.to_lowercase(), value.trim());
                }
                None => break,
            };
        }
        let content = line_iter.map(|s| s.to_string()).collect::<Vec<String>>();
        Ok(content.join("\r\n"))
    }
}

impl TryFrom<&str> for URL {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        if value.is_empty() {
            bail!("URL cannot be empty");
        } else {
            let split_arr = value.split("://").collect::<Vec<&str>>();
            let scheme = *split_arr.get(0).ok_or(anyhow!("Invalid URL format"))?;
            let url = split_arr[1..].join("://");
            if scheme != "http" && scheme != "https" {
                bail!("URL must start with 'http://' or 'https://'");
            }
            let url = if !url.contains("/") {
                format!("{}/", url)
            } else {
                url.to_string()
            };
            let url_split = url.split("/").collect::<Vec<&str>>();
            let mut host = *url_split.get(0).ok_or(anyhow!("Host cannot be empty"))?;
            let path = format!("/{}", url_split[1..].join("/"));
            let host_split = host.splitn(2, ':').collect::<Vec<&str>>();
            let mut port = if scheme == "https" {
                443
            } else {
                80
            };
            if host_split.len() == 2 {
                host = *host_split.get(0).ok_or(anyhow!("Host cannot be empty"))?;
                port = host_split.get(1).ok_or(anyhow!("port error"))?.parse::<u16>()?;
            }
            Ok(URL {
                url: url.to_string(),
                scheme: scheme.to_string(),
                hostname: host.to_string(),
                path,
                port,
            })
        }
    }
}

pub fn show(body: String) {
    let mut in_tag = false;
    for c in body.chars() {
        if c == '<' {
            in_tag = true;
            print!("<");
        } else if c == '>' {
            in_tag = false;
            print!(">");
        } else {
            print!("{}", c);
        }
    }
}

pub async fn load(url: &str) -> Result<URL> {
    let url = URL::try_from(url)?;
    let body = url.request().await?;
    show(body);
    Ok(url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_url() {
        let url_str = "http://example.com/path/to/resource";
        let url = URL::try_from(url_str).unwrap();
        assert_eq!(url.url, "example.com/path/to/resource");
        assert_eq!(url.scheme, "http");
        assert_eq!(url.host, "example.com");
        assert_eq!(url.path, "/path/to/resource");
    }

    #[test]
    fn test_invalid_url_empty() {
        let result = URL::try_from("");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_url_no_scheme() {
        let result = URL::try_from("example.com/path/to/resource");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_url_wrong_scheme() {
        let result = URL::try_from("ftp://example.com/path/to/resource");
        assert!(result.is_err());
    }
}
