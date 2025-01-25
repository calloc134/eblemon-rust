pub fn sanitize_to_filename(s: &str) -> String {
    let blacklist = ['/', '\\', '?', '%', '*', ':', '|', '"', '<', '>', '.', ' '];
    s.chars()
        .map(|c| if blacklist.contains(&c) { '_' } else { c })
        .collect()
}
