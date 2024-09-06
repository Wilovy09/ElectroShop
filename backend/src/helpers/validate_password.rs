use regex::Regex;

pub fn is_valid_password(password: &str) -> bool {
    let length_re = Regex::new(r".{8,}").unwrap(); // Al menos 8 caracteres
    let uppercase_re = Regex::new(r"[A-Z]").unwrap(); // Al menos una mayúscula
    let special_char_re = Regex::new(r"[!@#$%^&*(),.?:{}|<>]").unwrap(); // Al menos un carácter especial

    length_re.is_match(password)
        && uppercase_re.is_match(password)
        && special_char_re.is_match(password)
}
