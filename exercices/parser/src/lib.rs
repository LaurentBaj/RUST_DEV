fn parser(str: String) -> Result<i32, String> {
    match str.parse() {
        Ok(n) => Ok(n),
        Err(_) => Err(String::from("Error"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let str = String::from("42");
        let res = parser(str);
        assert_eq!(res, Ok(42));
    }

    #[test]
    fn catch_exception() {
        let str = String::from("abc");
        let res = parser(str);
        assert_eq!(res, Err(String::from("Error")));
    }
}
