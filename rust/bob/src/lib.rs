enum Reply {
    Question,
    Yell,
    YellQuestion,
    Empty,
    Default,
}

pub fn reply(message: &str) -> &str {
    let message = message.trim_end();

    match reply_type(message) {
        Reply::Question => "Sure.",
        Reply::Yell => "Whoa, chill out!",
        Reply::YellQuestion => "Calm down, I know what I'm doing!",
        Reply::Empty => "Fine. Be that way!",
        Reply::Default => "Whatever.",
    }
}

fn reply_type(message: &str) -> Reply {
    let alpha = message
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<String>();
    let is_all_upper = !alpha.is_empty() && alpha.chars().all(|c| c.is_uppercase());

    if message.ends_with('?') {
        if is_all_upper {
            return Reply::YellQuestion;
        } else {
            return Reply::Question;
        }
    }

    if is_all_upper {
        return Reply::Yell;
    } else if message.is_empty() || message.chars().all(|c| c.is_whitespace()) {
        return Reply::Empty;
    }

    Reply::Default
}
