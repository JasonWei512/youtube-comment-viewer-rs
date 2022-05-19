use anyhow::{anyhow, Result};
use clap::Parser;
use const_format::formatcp;
use reqwest::Url;

use crate::{config, constants};

const SHORT_ABOUT: &str = "A command line tool to display comments of a YouTube video.";

static LONG_ABOUT: &str = formatcp!("
{}

{}", SHORT_ABOUT, constants::ABOUT_API_KEY);

#[derive(Parser, Debug)]
#[clap(author, version, about = SHORT_ABOUT, long_about = LONG_ABOUT)]
struct CliArgs {
    /// The YouTube video ID or url to display comments for
    video_id_or_url: String,

    /// Display comments in YouTube's default order (default)
    #[clap(short, long)]
    default: bool,

    /// Display the newest comment first
    #[clap(short, long)]
    new: bool,

    /// Display the oldest comment first
    #[clap(short, long)]
    old: bool,

    /// Display the comment with the most likes first
    #[clap(short, long)]
    like: bool,

    /// Display the newest 10 comments only
    #[clap(short, long)]
    peek: bool,

    /// The YouTube API key to use instead of the default one
    #[clap(long)]
    api_key: Option<String>,
}

pub enum Order {
    Default,
    New,
    Old,
    Like,
    Peek,
}

pub struct Args {
    pub video_id: String,
    pub order: Order,
    pub api_key: String,
}

pub fn parse() -> Result<Args> {
    let cli_args = CliArgs::parse();

    let order = match (
        cli_args.default,
        cli_args.new,
        cli_args.old,
        cli_args.like,
        cli_args.peek,
    ) {
        (false, false, false, false, false) => Order::Default,
        (true, false, false, false, false) => Order::Default,
        (false, true, false, false, false) => Order::New,
        (false, false, true, false, false) => Order::Old,
        (false, false, false, true, false) => Order::Like,
        (false, false, false, false, true) => Order::Peek,
        _ => {
            return Err(anyhow!("Only one of the \"--default\", \"--new\", \"--old\", \"--like\", \"--peek\" option can be specified."));
        }
    };

    let api_key = match cli_args.api_key {
        Some(key) => key,
        None => match std::env::var("YOUTUBE_API_KEY") {
            Ok(key) => key,
            Err(_) => config::DEFAULT_YOUTUBE_API_KEY.to_string(),
        },
    };

    let video_id = parse_video_id(&cli_args.video_id_or_url);

    Ok(Args {
        video_id,
        order,
        api_key,
    })
}

fn parse_video_id(video_id_or_url: &str) -> String {
    let video_id_or_url_with_protocol =
        if video_id_or_url.starts_with("http://") || video_id_or_url.starts_with("https://") {
            video_id_or_url.to_owned()
        } else {
            "https://".to_owned() + video_id_or_url
        };

    match Url::parse(&video_id_or_url_with_protocol) {
        Ok(url) => {
            match url.host_str() {
                Some("www.youtube.com") | Some("youtube.com") => {
                    // https://www.youtube.com/watch?v={video_id}
                    if url.path() == "/watch" {
                        if let Some(video_id) =
                            url.query_pairs().find(|(k, _)| k == "v").map(|(_, v)| v)
                        {
                            return video_id.into_owned();
                        }
                    }
                }
                Some("youtu.be") => {
                    // https://youtu.be/{video_id}
                    return url.path().chars().skip(1).collect::<String>();
                }
                _ => {}
            }
        }
        Err(_) => {}
    }
    return video_id_or_url.to_owned();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_video_id() {
        let url_and_id_pairs = [
            ("https://www.youtube.com/watch?v=9Gj47G2e1Jc", "9Gj47G2e1Jc"),
            ("http://www.youtube.com/watch?v=9Gj47G2e1Jc", "9Gj47G2e1Jc"),
            ("www.youtube.com/watch?v=9Gj47G2e1Jc", "9Gj47G2e1Jc"),

            ("https://youtube.com/watch?v=9Gj47G2e1Jc", "9Gj47G2e1Jc"),
            ("http://youtube.com/watch?v=9Gj47G2e1Jc", "9Gj47G2e1Jc"),
            ("youtube.com/watch?v=9Gj47G2e1Jc", "9Gj47G2e1Jc"),

            ("https://youtu.be/9Gj47G2e1Jc", "9Gj47G2e1Jc"),
            ("http://youtu.be/9Gj47G2e1Jc", "9Gj47G2e1Jc"),
            ("youtu.be/9Gj47G2e1Jc", "9Gj47G2e1Jc"),

            ("9Gj47G2e1Jc", "9Gj47G2e1Jc"),
        ];

        for (url, id) in url_and_id_pairs {
            assert_eq!(parse_video_id(url), id);
        }
    }
}
