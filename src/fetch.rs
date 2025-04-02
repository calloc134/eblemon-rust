use crate::parse_image_url;
use crate::parse_metadata;

pub fn get_html_from_url(client: &ureq::Agent, url: &str) -> (String, String) {
    let response = client.get(url).call().unwrap_or_else(|e| {
        log::error!("Failed to access the URL: {:?}", e);
        panic!("Failed to access the URL")
    });
    let new_url = response.get_url().to_string();
    let html = response.into_string().unwrap();
    (new_url, html)
}

pub fn get_html_from_post_form(
    client: &ureq::Agent,
    url: &str,
    form_params: &[(&str, &str)],
    base_host: &str,
) -> String {
    let response = client
        .post(url)
        .set("X-Requested-With", "XMLHttpRequest")
        .set("Wicket-Ajax", "true")
        .set("Wicket-Ajax-BaseURL", base_host)
        .send_form(form_params)
        .unwrap_or_else(|e| {
            log::error!("Failed to access the anchor page URL: {:?}", e);
            panic!("Failed to access the anchor page URL")
        });
    response.into_string().unwrap()
}

pub fn get_metadata_from_url(
    client: &ureq::Agent,
    url: &str,
) -> Result<(String, parse_metadata::Metadata), Box<dyn std::error::Error>> {
    let (new_url, html) = get_html_from_url(client, url);
    let metadata = parse_metadata::parse_metadata_from_html(&html).map_err(|e| {
        log::error!("Failed to extract metadata: {:?}", e);
        e
    })?;
    Ok((new_url, metadata))
}

pub fn post_and_download_image(
    client: &ureq::Agent,
    url: &str,
    form_params: &[(&str, &str)],
    base_host: &str,
    download_dir: &str,
    page_number: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let html = get_html_from_post_form(client, url, form_params, base_host);
    let image_relative_url = parse_image_url::extract_page_image_url(&html).map_err(|e| {
        log::error!(
            "Failed to parse the page image URL for page {}: {:?}",
            page_number,
            e
        );
        e
    })?;
    let image_url = format!("{}{}", base_host, image_relative_url);
    let response = client.get(&image_url).call().map_err(|e| {
        log::error!(
            "Failed to download the image file for page {}: {:?}",
            page_number,
            e
        );
        e
    })?;
    let file_path = format!("{}/{}.jpg", download_dir, page_number);
    let mut output_image = std::fs::File::create(&file_path).map_err(|e| {
        log::error!(
            "Failed to create the image file for page {}: {:?}",
            page_number,
            e
        );
        e
    })?;
    std::io::copy(&mut response.into_reader(), &mut output_image).map_err(|e| {
        log::error!(
            "Failed to write the image file for page {}: {:?}",
            page_number,
            e
        );
        e
    })?;
    Ok(())
}

pub fn download_image_for_page(
    client: &ureq::Agent,
    url: &str,
    base_host: &str,
    download_dir: &str,
    page_number: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let page_str = page_number.to_string();
    let form_params = [
        ("id100_hf_0", ""),
        ("changeScale", "1"),
        ("pageNumEditor", page_str.as_str()),
        ("nextPageSubmit", "1"),
    ];
    post_and_download_image(
        client,
        url,
        &form_params,
        base_host,
        download_dir,
        page_number,
    )
}

pub fn skip_initial_page(
    client: &ureq::Agent,
    next_page_url: &str,
    base_host: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let _ = get_html_from_post_form(
        client,
        next_page_url,
        &[
            ("id100_hf_0", ""),
            ("changeScale", "1"),
            ("pageNumEditor", "1"),
            ("nextPageSubmit", "1"),
        ],
        base_host,
    );
    Ok(())
}
