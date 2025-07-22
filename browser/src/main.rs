mod parser;

fn main() {
    let test_string = "123\r\n456\r\n789";
    let lines = test_string.lines().collect::<Vec<&str>>();
    let mut line_iter = lines.iter();
    let one = line_iter.next().unwrap();
    let rest = line_iter.collect::<Vec<&&str>>();
    println!("First line: {}", one);
    println!("Remaining lines: {:?}", rest);
}
