use anyhow::{anyhow, Context, Result};
use scraper::{Html, Selector};
use thiserror::Error;

// メタデータ構造体
#[derive(Debug, PartialEq)]
pub struct Metadata {
    pub title: String,
    pub total_pages: u32,
}

// エラー定義
#[derive(Error, Debug)]
enum ParseError {
    #[error("Title not found")]
    TitleNotFound,
    #[error("Total pages not found")]
    TotalPagesNotFound,
}

// HTMLを解析してメタデータを取得する関数
pub fn extract_metadata(html: &str) -> Result<Metadata> {
    let document = Html::parse_document(html);

    // タイトルを取得
    let title_selector =
        Selector::parse("h1").map_err(|_| anyhow!("Invalid selector for title"))?;
    let title_element = document
        .select(&title_selector)
        .next()
        .ok_or(ParseError::TitleNotFound)?;
    let title = title_element.inner_html();

    // 全ページ数を取得
    let total_pages_selector = Selector::parse("span.allpageno")
        .map_err(|_| anyhow!("Invalid selector for total pages"))?;
    let total_pages_element = document
        .select(&total_pages_selector)
        .next()
        .ok_or(ParseError::TotalPagesNotFound)?;
    let total_pages_str = total_pages_element.inner_html();
    let total_pages = total_pages_str
        .parse::<u32>()
        .context("Failed to parse total pages as u32")?;

    Ok(Metadata { title, total_pages })
}
