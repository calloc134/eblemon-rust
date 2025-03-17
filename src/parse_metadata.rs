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

// New helper function to extract and trim text from the first matching element.
fn extract_trimmed_text_by_selector(
    document: &Html,
    selector_str: &str,
    err: ParseError,
) -> Result<String> {
    let selector = Selector::parse(selector_str)
        .map_err(|_| anyhow!("Invalid selector for {}", selector_str))?;
    let element = document.select(&selector).next().ok_or(err)?;
    Ok(element.inner_html().trim().to_string())
}

// HTMLを解析してメタデータを取得する関数
pub fn parse_metadata_from_html(html: &str) -> Result<Metadata> {
    let document = Html::parse_document(html);

    // タイトルを取得
    let title = extract_trimmed_text_by_selector(&document, "h1", ParseError::TitleNotFound)?;

    // 全ページ数を取得
    let total_pages_str = extract_trimmed_text_by_selector(
        &document,
        "span.allpageno",
        ParseError::TotalPagesNotFound,
    )?;
    let total_pages = total_pages_str
        .parse::<u32>()
        .context("Failed to parse total pages as u32")?;

    Ok(Metadata { title, total_pages })
}
