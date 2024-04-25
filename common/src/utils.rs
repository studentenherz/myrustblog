use regex::Regex;

pub fn is_valid_email(email: &str) -> bool {
    let email_regex = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
    email_regex.is_match(email)
}

pub fn normalize_email(email: &str) -> String {
    email.trim().to_lowercase()
}

pub fn is_valid_username(username: &str) -> bool {
    let username_regex = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
    username.len() >= 3 && username.len() <= 100 && username_regex.is_match(username)
}

pub fn is_valid_password(password: &str) -> bool {
    let allowed_chars_regex =
        Regex::new(r#"^[a-zA-Z0-9!@#$%^&*()_+\-=\[\]{};':\"\\|,.<>\/?~ ]+$"#).unwrap();

    allowed_chars_regex.is_match(password) && password.len() >= 3 && password.len() <= 100
}

pub fn title_to_slug(title: &str) -> String {
    title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
