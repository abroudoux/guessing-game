#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_choose_difficulty() {
        assert_eq!(choose_difficulty("easy"), Difficulty::Easy);
        assert_eq!(choose_difficulty("medium"), Difficulty::Medium);
        assert_eq!(choose_difficulty("hard"), Difficulty::Hard);
        assert_eq!(choose_difficulty("invalid"), Difficulty::Easy);
    }
}
