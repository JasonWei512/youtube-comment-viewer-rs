mod args;
mod config;
mod constants;
mod models;

use anyhow::{anyhow, Result};
use args::{Args, Order};
use async_stream::try_stream;
use chrono::{DateTime, Local};
use futures_util::{pin_mut, Stream, StreamExt};
use models::{
    comment::{Comment, CommentSnippet},
    comment_thread::CommentThread,
    envelop::Envelop,
    video::Video,
};
use once_cell::sync::Lazy;
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;
use std::pin::Pin;

static REQWEST_CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

#[tokio::main]
async fn main() -> Result<()> {
    let Args {
        video_id,
        order,
        api_key,
    } = args::parse()?;

    print_video_info(&video_id, &api_key).await?;

    let comment_stream = get_comment_stream(&video_id, &order, &api_key);
    pin_mut!(comment_stream);

    match order {
        Order::Default | Order::New | Order::Peek => {
            while let Some(comment) = comment_stream.next().await {
                print_comment_with_replies(&comment?, &order, &api_key).await?;
            }
        }
        _ => {
            let mut all_comments = collect_stream(comment_stream).await?;

            match order {
                Order::Old => {
                    all_comments.sort_by_key(|c| c.snippet.top_level_comment.snippet.published_at)
                }
                Order::Like => {
                    all_comments.sort_by_key(|c| {
                        (
                            c.snippet.top_level_comment.snippet.like_count,
                            c.snippet.top_level_comment.snippet.published_at,
                        )
                    });
                    all_comments.reverse();
                }
                _ => unreachable!(),
            };

            for comment in all_comments {
                print_comment_with_replies(&comment, &order, &api_key).await?;
            }
        }
    }

    Ok(())
}

async fn print_video_info(video_id: &str, api_key: &str) -> Result<()> {
    println!("Url         https://www.youtube.com/watch?v={}", video_id);
    println!();

    let request = REQWEST_CLIENT
        .get("https://youtube.googleapis.com/youtube/v3/videos")
        .query(&[
            ("id", video_id),
            ("part", "snippet,statistics"),
            ("key", api_key),
        ]);

    let response = get_response::<Video>(request).await?;

    if let Some(video) = response.items.unwrap().get(0) {
        println!("Title       {}", video.snippet.title);
        println!("Channel     {}", video.snippet.channel_title);
        println!("Time        {}", &prettify_datetime(&video.snippet.published_at));
        println!("Description {}", video.snippet.description.replace("\n", "\n            "));
        println!();
        println!("Views       {}", video.statistics.view_count);
        println!("Likes       {}", video.statistics.like_count);
        println!("Comments    {}", video.statistics.comment_count);
        println!();

        return Ok(());
    } else {
        return Err(anyhow!("Video not found."));
    }
}

fn get_comment_stream<'a>(
    video_id: &'a str,
    order: &'a Order,
    api_key: &'a str,
) -> impl Stream<Item = Result<CommentThread>> + 'a {
    try_stream! {
        let mut page_token: Option<String> = None;

        loop {
            let mut request = REQWEST_CLIENT
                .get("https://youtube.googleapis.com/youtube/v3/commentThreads")
                .query(&[
                    ("videoId", video_id),
                    ("maxResults", if let Order::Peek = order { "10" } else { "100" }),
                    ("order", if let Order::Default = order { "relevance" } else { "time" }),
                    ("part", "snippet,replies"),
                    ("textFormat", "plainText"),
                    ("key", api_key)
                ]);

            if let Some(token) = page_token {
                request = request.query(&[("pageToken", &token)]);
            }

            let response = get_response::<CommentThread>(request).await?;

            for comment_thread in response.items {
                for comment in comment_thread {
                    yield comment;
                }
            }

            if let Order::Peek = order {
                break;
            }

            match response.next_page_token {
                Some(next_page_token) => { page_token = Some(next_page_token); },
                None => { break },
            };
        }
    }
}

fn get_reply_stream<'a>(
    comment_thread_id: &'a str,
    api_key: &'a str,
) -> impl Stream<Item = Result<Comment>> + 'a {
    try_stream! {
        let mut page_token: Option<String> = None;

        loop {
            let mut request = REQWEST_CLIENT
                .get("https://youtube.googleapis.com/youtube/v3/comments")
                .query(&[
                    ("part", "snippet,id"),
                    ("maxResults", "100"),
                    ("parentId", comment_thread_id),
                    ("textFormat", "plainText"),
                    ("key", api_key)
                ]);

            if let Some(token) = page_token {
                request = request.query(&[("pageToken", &token)]);
            }

            let response = get_response::<Comment>(request).await?;

            for comment in response.items {
                for reply in comment {
                    yield reply;
                }
            }

            match response.next_page_token {
                Some(next_page_token) => { page_token = Some(next_page_token); },
                None => { break },
            };
        }
    }
}

async fn print_comment_with_replies(
    comment_thread: &CommentThread,
    order: &Order,
    api_key: &str,
) -> Result<()> {
    print_single_comment_or_reply(&comment_thread.snippet.top_level_comment.snippet, false);

    if let Some(replies) = &comment_thread.replies {
        let sort = |cs: &mut Vec<Comment>| {
            match order {
                Order::Default | Order::Like => {
                    cs.sort_by_key(|c| (c.snippet.like_count, c.snippet.published_at));
                    cs.reverse();
                }
                Order::New | Order::Peek => {
                    cs.sort_by_key(|c| c.snippet.published_at);
                    cs.reverse()
                }
                Order::Old => cs.sort_by_key(|c| c.snippet.published_at),
            };
        };

        if replies.comments.len() as u32 == comment_thread.snippet.total_reply_count {
            let mut all_replies = replies.comments.clone();
            sort(&mut all_replies);

            for reply in &all_replies {
                print_reply(&reply);
            }
        } else {
            let reply_stream = get_reply_stream(&comment_thread.id, api_key);
            pin_mut!(reply_stream);

            match order {
                Order::New | Order::Peek => {
                    while let Some(reply) = reply_stream.next().await {
                        print_reply(&reply?);
                    }
                }
                _ => {
                    let mut all_replies = collect_stream(reply_stream).await?;
                    sort(&mut all_replies);

                    for reply in all_replies {
                        print_reply(&reply);
                    }
                }
            }
        }
    }

    println!();

    Ok(())
}

fn print_single_comment_or_reply(comment: &CommentSnippet, tabbed: bool) {
    let tab = || if tabbed { "            " } else { "" };

    println!("{}Author      {}", tab(), comment.author_display_name);
    println!("{}Time        {}", tab(), &prettify_datetime(&comment.published_at));
    println!("{}Likes       {}{}", tab(), comment.like_count, if comment.like_count > 0 { " 👍" } else { "" });
    println!("{}Content     {}", tab(), comment.text_display.replace("\n", &format!("\n            {}", tab())));
}

fn print_reply(reply: &Comment) {
    println!("            ------------");
    print_single_comment_or_reply(&reply.snippet, true);
}

async fn get_response<T: DeserializeOwned>(request: RequestBuilder) -> Result<Envelop<T>> {
    let response = request.send().await?.json::<Envelop<T>>().await?;

    if let Some(mut error) = response.error {
        if error.code == 400 {
            // Invalid YouTube API key
            error.message.push_str("\n\n");
            error.message.push_str(constants::ABOUT_API_KEY);
        }

        return Err(anyhow!(error));
    } else {
        return Ok(response);
    }
}

fn prettify_datetime(datetime: &DateTime<Local>) -> String {
    format!("{}", datetime.format("%Y/%m/%d %H:%M:%S"))
}

async fn collect_stream<T>(mut stream: Pin<&mut impl Stream<Item = Result<T>>>) -> Result<Vec<T>> {
    let mut comments = Vec::new();

    while let Some(comment) = stream.next().await {
        comments.push(comment?);
    }

    Ok(comments)
}
