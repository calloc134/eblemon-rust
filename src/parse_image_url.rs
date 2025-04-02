use anyhow::Result;
use scraper::{Html, Selector};
use thiserror::Error;

#[derive(Error, Debug)]
enum ParseError {
    #[error("'_pageImageURL' element not found")]
    PageImageURLNotFound,
    #[error("'_pageImageURL' element does not have text content")]
    PageImageURLTextContentNotFound,
}

pub fn extract_page_image_url(html: &str) -> Result<String> {
    let document = Html::parse_fragment(html);
    let selector = Selector::parse(r#"span[name="_pageImageURL"]"#).unwrap();
    let page_image_url_element = document
        .select(&selector)
        .next()
        .ok_or(ParseError::PageImageURLNotFound)?;
    let text_content: String = page_image_url_element.text().collect();
    if text_content.is_empty() {
        return Err(ParseError::PageImageURLTextContentNotFound.into());
    }
    Ok(text_content)
}
