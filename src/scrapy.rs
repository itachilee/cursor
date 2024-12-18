use crate::redis::save_to_redis;
use color_eyre::{eyre::Ok, Result};
use regex::Regex;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use url::Url;

const BASE_URL: &str = "https://www.bilinovel.com";
const START_URL: &str = "https://www.bilinovel.com/novel/2978/157901.html";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Novel {
    pub title: String,
    pub content: String,
    pub prev: String,
    pub next: String,
}

async fn reqwest_html(url: &str) -> String {
    let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36";
    println!("start requesting : {}", url);
    let response = reqwest::Client::builder()
        .user_agent(user_agent)
        .build()
        .unwrap()
        .get(url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("end requesting : {}", url);
    response
}

pub async fn scrapy_html(url: &str) -> Novel {
    let html = reqwest_html(url).await;
    let (title, content) = parse_html(&html);
    let (prev, next) = regex_script(&html);

    let novel = Novel {
        title,
        content,
        prev,
        next: next.to_string(),
    };
    novel
}

/// 解析html 返回标题和内容
fn parse_html(html: &str) -> (String, String) {
    let html = Html::parse_document(html);
    let title_selector = Selector::parse("title").unwrap();
    let title = html.select(&title_selector).next().unwrap();
    // println!("title: {}", title.text().collect::<Vec<_>>().join(""));

    let content_selector = Selector::parse("div#acontent").unwrap();
    let content = html.select(&content_selector).next().unwrap();
    // println!("content: {}", content.text().collect::<Vec<_>>().join(""));
    (
        title.text().collect::<Vec<_>>().join(""),
        content.text().collect::<Vec<_>>().join(""),
    )
}

fn regex_script(text: &str) -> (String, String) {
    let re = Regex::new(r"<script(?:\s+[^>]*)*>([\s\S]*?)</script>").unwrap();

    for cap in re.captures_iter(text) {
        // println!("找到script内容（包含属性）: {}", &cap[1]);

        let (prev, next) = extract_page_links(&cap[1]);

        if prev.is_some() && next.is_some() {
            return (prev.unwrap(), next.unwrap());
        }
    }
    ("".to_string(), "".to_string())
}

fn extract_page_links(text: &str) -> (Option<String>, Option<String>) {
    let prev_re = Regex::new(r#"prevpage\s*=\s*"([^"]+)""#).unwrap();
    let next_re = Regex::new(r#"nextpage\s*=\s*"([^"]+)""#).unwrap();

    let prev_link = prev_re.captures(text).map(|cap| cap[1].to_string());

    let next_link = next_re.captures(text).map(|cap| cap[1].to_string());

    (prev_link, next_link)
}

fn join_url(base_url: &str, url: &str) -> String {
    let base_url = Url::parse(base_url).unwrap();
    let full_url = base_url.join(url).unwrap();
    full_url.to_string()
}
