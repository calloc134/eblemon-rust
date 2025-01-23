use parse_metadata::extract_metadata;
use proconio::input;
mod parse_metadata;

fn main() {
    // まずURLを取得
    proconio::input! {
        url: String,
    }

    // アクセス
    let response = reqwest::blocking::get(&url).unwrap();
    let html = response.text().unwrap();

    // メタデータを取得
    let metadata = extract_metadata(&html).unwrap();
    println!("Title: {}", metadata.title);
    println!("Total pages: {}", metadata.total_pages);
}
