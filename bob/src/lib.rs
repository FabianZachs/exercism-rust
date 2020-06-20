pub fn reply(message: &str) -> &str {
    match message.trim() {
        m if m.is_empty() => "Fine. Be that way!",
        m if all_capital(m) && m.ends_with('?') => "Calm down, I know what I\'m doing!",
        m if m.ends_with('?') => "Sure.",
        m if all_capital(m) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}

fn all_capital(message: &str) -> bool {
    let are_letters = message.chars().any(|x| x.is_alphabetic());
    message.to_uppercase() == message && are_letters
}
