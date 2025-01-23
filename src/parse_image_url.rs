use anyhow::{anyhow, Context, Result};
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

fn main() -> Result<()> {
    let html = r#"<?xml version="1.0" encoding="UTF-8"?><ajax-response><component id="id18b" ><![CDATA[<div id="id18b">
        <span name="_scale" style="display:none;" id="id18c">1</span>
        <span name="_pageNum" style="display:none;" id="id18d">1</span>
        <span name="_pageImageURL" style="display:none;" id="id18e">/kikan_info/GetImg.jpg?id=119323131220803976491832</span>
        <span name="_pageImageScale" style="display:none;" id="id18f">200</span>
        <span name="_pageServiceMsg" style="display:none;" id="id190"></span>
        <span name="_isDrawOnePage" style="display:none;" id="id191">true</span>
        <span name="_isLeftToRightPage" style="display:none;" id="id192">true</span>
        <script type="text/javascript">
/*<![CDATA[*/

          readParams();
          drawPageImage();
        
/*]]]]><![CDATA[>*/
</script>
      </div>]]></component><priority-evaluate><![CDATA[(function(){setCurrentScale();})();]]></priority-evaluate><evaluate><![CDATA[(function(){startLoadingIndicator();})();]]></evaluate></ajax-response>"#;

    match get_page_image_url(html) {
        Ok(url) => println!("_pageImageURL: {}", url),
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}
