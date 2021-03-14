pub fn validate_isbn(isbn: &str) -> bool {
    if isbn.len() != 10 && isbn.len() != 13 {
        return false;
    }

    isbn.chars().all(|c| c.is_digit(10))
}
