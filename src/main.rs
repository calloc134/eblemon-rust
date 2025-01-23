use generate_anchor_page_url::generate_anchor_page_url;
use parse_metadata::extract_metadata;
use proconio::input;
mod generate_anchor_page_url;
mod parse_image_url;
mod parse_metadata;
use ureq::agent;

fn main() {
    // まずURLを取得
    input! {
        url: String,
    }

    // セッションを作成
    let client = agent();

    // アクセス
    let response = client.get(&url).call();
    if (response.is_err()) {
        panic!("Failed to access the URL");
    }

    let response = response.unwrap();
    let url = response.get_url().to_string();
    let html = response.into_string().unwrap();

    // メタデータを取得
    let metadata = extract_metadata(&html).unwrap();
    println!("Title: {}", metadata.title);
    println!("Total pages: {}", metadata.total_pages);

    for i in 1..=metadata.total_pages {
        // アンカーページのURLを生成
        let anchor_page_url = generate_anchor_page_url(&url, i);
        println!("Anchor page URL: {}", anchor_page_url);

        // アンカーページにアクセス
        let response = client.get(&anchor_page_url).call();
        if (response.is_err()) {
            panic!("Failed to access the anchor page URL");
        }

        let response = response.unwrap();
        let html = response.into_string().unwrap();
        print!("HTML: {}", html);

        // アンカーページのレスポンスの解析
        let page_image_url = parse_image_url::get_page_image_url(&html).unwrap();

        println!("Page image URL: {}", page_image_url);

        // ファイルのダウンロード
        // とりあえずダミーの処理
    }

    println!("Done");
}
