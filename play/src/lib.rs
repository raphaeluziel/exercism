pub fn reply(message: &str) -> &str {
    let trimmed_message = message.trim();
    if trimmed_message.chars().any(|c| !matches!(c, 'A'..='z')) {
        "Whatever."
    }
    else if trimmed_message.split_whitespace().next() == None {
        "Fine. Be that way!"
    }
    else if trimmed_message.ends_with("?") && message.to_uppercase() == message {
        "Calm down, I know what I'm doing!"
    }
    else if trimmed_message.ends_with("?") { 
        "Sure." 
    }
    else if trimmed_message.to_uppercase() == message { 
        "Whoa, chill out!" 
    }
    else { 
        "Whatever." 
    }
}
