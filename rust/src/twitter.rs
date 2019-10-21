extern crate egg_mode;
extern crate getopts;
extern crate serde;
extern crate serde_json;
extern crate tokio;
extern crate url;
extern crate uuid;

use self::egg_mode::tweet::DraftTweet;
use self::egg_mode::Token;
use self::tokio::runtime::current_thread::block_on_all;
use self::url::Url;

use categories::Category;
use models::Post;

use std::env;
use std::error::Error;
use std::rc::Rc;

pub fn token_from_env() -> Result<egg_mode::Token, Box<dyn Error>> {
    let token = egg_mode::Token::Access {
        consumer: egg_mode::KeyPair::new(
            env::var("TWITTER_CONSUMER_KEY")?,
            env::var("TWITTER_CONSUMER_SECRET")?,
        ),
        access: egg_mode::KeyPair::new(
            env::var("TWITTER_ACCESS_KEY")?,
            env::var("TWITTER_ACCESS_KEY")?,
        ),
    };

    Ok(token)
}

pub fn register(
    consumer_key: String,
    consumer_secret: String,
) -> Result<egg_mode::Token, Box<dyn Error>> {
    let con_token = egg_mode::KeyPair::new(consumer_key, consumer_secret);

    let request_token = block_on_all(egg_mode::request_token(&con_token, "oob"))?;

    println!("Go to the following URL, sign in, and enter the PIN:");
    println!("{}", egg_mode::authorize_url(&request_token));

    let mut pin = String::new();
    std::io::stdin().read_line(&mut pin)?;
    println!("");

    let (token, _user_id, _screen_name) =
        block_on_all(egg_mode::access_token(con_token, &request_token, pin))?;

    Ok(token)
}

pub fn tweet_post(
    token: &Token,
    post: &Post,
    categories: &[Rc<Category>],
) -> Result<(), Box<dyn Error>> {
    if let Some(tweet_url) = &post.twitter_url {
        let tweet_id = tweet_id_from_url(&tweet_url)
            .ok_or_else(|| format_err!("{} is not a valid tweet URL", tweet_url))?;
        info!("🔁 Tweet {}", tweet_url);
        let work = egg_mode::tweet::retweet(tweet_id, token);
        block_on_all(work)?;
    } else {
        let status_text = tweet_text_from_post(post, categories);
        info!("Tweet {}", status_text);
        let tweet = DraftTweet::new(status_text);

        let work = tweet.send(token);
        block_on_all(work)?;
    };

    Ok(())
}

fn tweet_text_from_post(post: &Post, categories: &[Rc<Category>]) -> String {
    let hashtags = categories
        .iter()
        .map(|category| category.hashtag.as_str())
        .collect::<Vec<&str>>()
        .join(" ");

    format!(
        "{title} by {author}: {url} {tags}",
        title = post.title,
        author = post.author,
        url = post.url,
        tags = hashtags
    )
}

// https://twitter.com/llogiq/status/1012438300781576192
fn tweet_id_from_url(url: &str) -> Option<u64> {
    let url: Url = url.parse().ok()?;
    if url.domain() != Some("twitter.com") {
        return None;
    }

    let segments = url.path_segments().map(|iter| iter.collect::<Vec<_>>())?;
    match segments.as_slice() {
        [_, "status", id] => id.parse().ok(),
        _ => None,
    }
}

#[test]
fn test_tweet_id_from_valid_url() {
    assert_eq!(
        tweet_id_from_url(
            &"https://twitter.com/llogiq/status/1012438300781576192"
                .parse()
                .unwrap()
        ),
        Some(1012438300781576192)
    );
}

#[test]
fn test_tweet_id_from_invalid_url() {
    assert_eq!(
        tweet_id_from_url(
            &"https://not_twitter.com/llogiq/status/1012438300781576192"
                .parse()
                .unwrap()
        ),
        None
    );
}

#[test]
fn test_tweet_id_from_non_status_url() {
    assert_eq!(
        tweet_id_from_url(&"https://twitter.com/rustlang/".parse().unwrap()),
        None
    );
}

#[test]
fn test_tweet_id_from_almost_valid_url() {
    assert_eq!(
        tweet_id_from_url(
            &"https://mobile.twitter.com/shaneOsbourne/status/1012451814338424832/photo/2"
                .parse()
                .unwrap()
        ),
        None
    );
}