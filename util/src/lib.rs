#[cfg(test)]
mod tests {
    use crate::year2022;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_main() {
        let res = year2022::may::day22::main();
    }
}

pub mod year2022;
