// Second try.  Just doing the automated suggestion.

pub fn reply(message: &str) -> &str {
    match message.trim() {
        x if x.split_whitespace().next().is_none() => "Fine. Be that way!",
        x if x.ends_with("?") => {
            if x.to_uppercase() == x && x.to_lowercase() != x { "Calm down, I know what I'm doing!" }
            else { "Sure." }
        },
        x if x.to_uppercase() == x && x.to_lowercase() != x => "Whoa, chill out!",
        _ => "Whatever."
    }
}