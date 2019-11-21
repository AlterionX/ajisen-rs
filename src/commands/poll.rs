use regex::Regex;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};
use serenity::utils::MessageBuilder;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PollRequest<'a> {
    question: &'a str,
    choices: Option<Vec<&'a str>>,
}

// TODO: Implement better error report by specifying where things failed.
impl<'a> PollRequest<'a> {
    const STR_PATTERN: &'static str = r#"^(?s:(?:"(?P<text>(?:[^\\"]|(?:\\.))*)")\s*)"#;
    const POLL_HEAD: &'static str = r"^(?s:\s*~poll\s*)";
    const OPTS_HEAD: &'static str = r"^(?s:\s*(?i:opt(?:ion)?s)\s*)";

    fn without_head(msg: &'a str) -> Option<&'a str> {
        let poll_head_regex = Regex::new(Self::POLL_HEAD).unwrap();
        poll_head_regex
            .find(msg)
            .map(|matched| &msg[matched.end()..])
    }

    fn extract_question(msg: &'a str) -> Option<(&'a str, &'a str)> {
        let str_regex = Regex::new(Self::STR_PATTERN).unwrap();
        str_regex
            .captures(msg)
            .and_then(|capture| {
                if let (Some(full), Some(text)) = (capture.get(0), capture.name("text")) {
                    Some((full, text))
                } else {
                    None
                }
            })
            .map(|(full, text)| (
                    &msg[text.start()..text.end()],
                    &msg[full.end()..],
            ))
    }

    fn without_choices_head_or_none(msg: &'a str) -> Option<&'a str> {
        let opts_head_regex = Regex::new(Self::OPTS_HEAD).unwrap();

        opts_head_regex
            .find(msg)
            .map(|matched| &msg[matched.end()..])
    }

    fn extract_choices(msg: &'a str) -> Option<(Vec<&'a str>, &'a str)> {
        let str_regex = Regex::new(Self::STR_PATTERN).unwrap();
        let mut remaining = msg;
        let matches_plus_one_remaining = std::iter::from_fn(|| {
            let capture = str_regex.captures(remaining)?; // If no matches, we're done.
            if let (Some(full), Some(text)) = (capture.get(0), capture.name("text")) {
                let text = &remaining[text.start()..text.end()];
                remaining = &remaining[full.end()..];
                Some(text)
            } else {
                None
            }
        });
        Some((matches_plus_one_remaining.collect(), remaining))
    }

    fn extract(msg: &'a str) -> Option<Self> {
        let remaining = Self::without_head(msg)?;
        let (question, remaining) = Self::extract_question(remaining)?;
        let (choices, remaining) = if let Some(remaining) = Self::without_choices_head_or_none(remaining) {
            let (choices, remaining) = Self::extract_choices(remaining)?;
            (Some(choices), remaining)
        } else {
            (None, remaining)
        };
        if remaining.len() == 0 {
            Some(Self {
                question,
                choices,
            })
        } else {
            None
        }
    }
}

const REGIONAL_INDICATORS: &'static str = "🇦🇧🇨🇩🇪🇫🇬🇭🇮🇯🇰🇱🇲🇳🇴🇵🇶🇷🇸🇹🇺🇻🇼🇽🇾🇿";
const YES_NO_INDICATORS: &'static str = "👍👎";

#[command]
#[description("Poll")]
#[usage("
    Enter your question in quotes with escape characters if need be. Follow this with options, also with
    escape characters. A more technical example is show here:

    `~poll \"Question\" [opt[ion]s (\"An option\")+]`

    If no options are provided, the question is assumed to be a yes or no question.
")]
#[example("~poll \"How's the weather today?\" \"Good.\" \"Ok\" \"Bad\"")]
#[help_available]
pub fn poll(ctx: &mut Context, msg: &Message) -> CommandResult {
    let question = PollRequest::extract(msg.content.as_str()).ok_or("Cannot parse message as a poll request!")?;
    if question.choices.as_ref().map_or(0, |v| v.len()) > 26 {
        Err("We only support polls with up to 26 choices! Sorry.".to_string())?;
    }

    let mut response = MessageBuilder::new();
    response
        .push("@here, ")
        .mention(&msg.author)
        .push_line(" has started a poll!")
        .push_bold_line(question.question)
        .push_line("Here are the choices:");
    let indicators: String = if let Some(choices) = question.choices {
        for (idx, choice) in choices.iter().enumerate() {
            response
                .push(":regional_indicator_")
                .push((b'a' + idx as u8) as char)
                .push(": :")
                .push_bold_line(choice);
        }
        response.push("Please react with your response!");
        REGIONAL_INDICATORS
            .chars()
            .enumerate()
            .filter(|(i, _)| *i < choices.len())
            .map(|(_, c)| c)
            .collect()
    } else {
        response
            .push_bold_safe("Please select yes or no.")
            .build();
        YES_NO_INDICATORS.chars().collect()
    };
    let response = response.build();

    let message = msg.channel_id.say(&ctx, &response)?;
    for indicator in indicators.chars() {
        message.react(&ctx, indicator)?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_fail() {
        const SAMPLE: &'static str = "";
        assert_eq!(PollRequest::extract(SAMPLE), None);
    }

    #[test]
    fn weird_fail() {
        const SAMPLE: &'static str = "~poll \"Hell\"o\"";
        assert_eq!(PollRequest::extract(SAMPLE), None);
    }

    #[test]
    fn simple() {
        const SAMPLE: &'static str = "~poll \"Hello\"";
        assert_eq!(PollRequest::extract(SAMPLE), Some(PollRequest {
            question: "Hello",
            choices: None,
        }));
    }

    #[test]
    fn options() {
        const SAMPLE: &'static str = "~poll \"Hello\" options";
        assert_eq!(PollRequest::extract(SAMPLE), Some(PollRequest {
            question: "Hello",
            choices: Some(vec![])
        }));
    }

    #[test]
    fn opts() {
        const SAMPLE: &'static str = "~poll \"Hello\" opts";
        assert_eq!(PollRequest::extract(SAMPLE), Some(PollRequest {
            question: "Hello",
            choices: Some(vec![])
        }));
    }

    #[test]
    fn actual_options() {
        const SAMPLE: &'static str = "~poll \"Hello\" opts \"Data\" \"More \\\" Data\"";
        assert_eq!(PollRequest::extract(SAMPLE), Some(PollRequest {
            question: "Hello",
            choices: Some(vec!["Data", "More \\\" Data"])
        }));
    }
}
