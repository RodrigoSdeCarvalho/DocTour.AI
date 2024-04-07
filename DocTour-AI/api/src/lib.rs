pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use system::Logger;
    use system::benchmark;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_system() {
        let result = benchmark!(add(2, 2));
        Logger::error("Mock Error", false);
        assert_eq!(result, 4);
    }
}
