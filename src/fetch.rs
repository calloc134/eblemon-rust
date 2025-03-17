pub fn fetch_html(client: &ureq::Agent, url: &str) -> (String, String) {
    let response = client.get(url).call().unwrap_or_else(|e| {
        log::error!("Failed to access the URL: {:?}", e);
        panic!("Failed to access the URL")
    });
    let new_url = response.get_url().to_string();
    let html = response.into_string().unwrap();
    (new_url, html)
}

pub fn fetch_post_html(
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
