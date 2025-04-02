use anyhow::{anyhow, Context, Result};
use scraper::{Html, Selector};
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub struct Metadata {
    pub title: String,
    pub total_pages: u32,
}

#[derive(Error, Debug)]
enum ParseError {
    #[error("Title not found")]
    TitleNotFound,
    #[error("Total pages not found")]
    TotalPagesNotFound,
}

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

pub fn parse_metadata_from_html(html: &str) -> Result<Metadata> {
    let document = Html::parse_document(html);

    let title = extract_trimmed_text_by_selector(&document, "h1", ParseError::TitleNotFound)?;

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
