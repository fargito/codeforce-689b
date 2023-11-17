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
    use crate::handle;

    #[test]
    fn test_1() {
        let res = handle("3".to_string(), "2 2 3".to_string());

        assert_eq!(res, "0 1 2");
    }

    #[test]
    fn test_2() {
        let res = handle("5".to_string(), "1 2 3 4 5".to_string());

        assert_eq!(res, "0 1 2 3 4");
    }

    #[test]
    fn test_3() {
        let res = handle("7".to_string(), "4 4 4 4 7 7 7".to_string());

        assert_eq!(res, "0 1 2 1 2 3 3");
    }
}
