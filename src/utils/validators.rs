use regex::Regex;

pub fn validate_dni(dni: &str) -> bool {
    dni.len() == 8 && dni.chars().all(|c| c.is_ascii_digit())
}

pub fn validate_ruc(ruc: &str) -> bool {
    ruc.len() == 11 && ruc.chars().all(|c| c.is_ascii_digit())
}

pub fn validate_email(email: &str) -> bool {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    email_regex.is_match(email)
}

pub fn validate_phone(phone: &str) -> bool {
    let phone_regex = Regex::new(r"^\+?[1-9]\d{1,14}$").unwrap();
    phone_regex.is_match(phone)
}

pub fn validate_password_strength(password: &str) -> Result<(), String> {
    if password.len() < 8 {
        return Err("La contraseña debe tener al menos 8 caracteres".to_string());
    }

    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());

    if !has_uppercase || !has_lowercase || !has_digit {
        return Err("La contraseña debe contener mayúsculas, minúsculas y números".to_string());
    }

    Ok(())
}
