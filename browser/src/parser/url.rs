use std::net::SocketAddr;

use anyhow::{Error, Result, anyhow, bail};
use tokio::{io::AsyncWriteExt, net::TcpSocket};

pub struct URL {
    pub url: String,
    pub scheme: String,
    pub host: String,
    pub path: String,
}

impl URL {
    pub async fn request(&self) -> Result<()> {
        let addr = SocketAddr::new(self.host.parse()?, 80);
        let socket = TcpSocket::new_v4()?;
        let mut stream = socket.connect(addr).await?;
        let request = format!("GET {} HTTP/1.0\r\nHost: {}\r\n\r\n", self.path, self.host);
        stream.write_all(request.as_bytes()).await?;
        Ok(())
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
            let host = *url_split.get(0).ok_or(anyhow!("Host cannot be empty"))?;
            let path = format!("/{}", url_split[1..].join("/"));
            Ok(URL {
                url: url.to_string(),
                scheme: scheme.to_string(),
                host: host.to_string(),
                path,
            })
        }
    }
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
