mod addition;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(addition::calculate_two_numbers(2, 2), 4);
    }
    #[test]
    fn it_wont_work() {
        let result = 2 + 5;
        assert_eq!(result, 7);
    }
}
