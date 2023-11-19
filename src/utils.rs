pub fn is_alpha(value: char) -> bool {
    value.is_alphabetic() || value == '_'
}

pub fn is_alphanumeric(value: char) -> bool {
    is_alpha(value) || value.is_numeric()
}