pub fn generate_anchor_page_url(normal_url: &str, page_number: u32) -> String {
    let url = format!(
        "{}-1.IBehaviorListener.0-tocListPanel-tocList-{}-anchor",
        normal_url, page_number
    );
    url
}
