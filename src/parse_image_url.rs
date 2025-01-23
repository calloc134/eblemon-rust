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

pub fn get_page_image_url(html: &str) -> Result<String> {
    let document = Html::parse_fragment(html);
    let selector = Selector::parse(r#"span[name="_pageImageURL"]"#).unwrap();

    let mut page_image_url_element = None;
    for element in document.select(&selector) {
        page_image_url_element = Some(element);
        break;
    }

    let page_image_url_element = page_image_url_element.ok_or(ParseError::PageImageURLNotFound)?;

    let mut text_content = String::new();
    for text in page_image_url_element.text() {
        text_content.push_str(text);
    }

    if text_content.is_empty() {
        return Err(ParseError::PageImageURLTextContentNotFound.into());
    }

    Ok(text_content)
}
