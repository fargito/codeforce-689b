use std::io;

fn main() -> io::Result<()> {
    let mut first_line = String::new();
    let mut second_line = String::new();
    io::stdin().read_line(&mut first_line)?;
    io::stdin().read_line(&mut second_line)?;

    let res = handle(first_line, second_line);

    println!("{res}");

    Ok(())
}

fn handle(first_line: String, second_line: String) -> String {
    "toto".to_string()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_1() {}

    #[test]
    fn test_2() {}

    #[test]
    fn test_3() {}
}
