use std::cmp::Ordering;

pub fn reply(message: &str) -> &str {
    let message = message.trim_end();
    if message.len() == 0 {
        return "Fine. Be that way!";
    }
    let question = message.ends_with('?');
    let caps = message.chars().any(|c| c.is_ascii_alphabetic())
        && message.to_ascii_uppercase().cmp(&message.to_string()) == Ordering::Equal;

    match (question, caps) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Sure.",
        (false, true) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
