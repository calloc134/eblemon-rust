use generate_anchor_page_url::generate_anchor_page_url;
use parse_metadata::extract_metadata;
use proconio::input;
mod generate_anchor_page_url;
mod parse_image_url;
mod parse_metadata;
use ureq::agent;

fn main() {
    // まずURLを取得
    println!("Please input the URL of the target page");
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
    // let cookies = client.cookie_store();
    println!("URL: {}", url);
    // let cookies_hoge = cookies.iter_any().collect::<Vec<_>>();
    // println!("Cookies: {:?}", cookies_hoge);

    // メタデータを取得
    let metadata = extract_metadata(&html).unwrap();
    println!("Title: {}", metadata.title);
    println!("Total pages: {}", metadata.total_pages);

    for i in 1..=metadata.total_pages {
        // アンカーページのURLを生成
        let anchor_page_url = generate_anchor_page_url(&url, i);
        println!("Anchor page URL: {}", anchor_page_url);

        // headers = {
        //     'X-Requested-With': 'XMLHttpRequest',
        //     'Wicket-Ajax': 'true',
        //     'Wicket-Ajax-BaseURL': '{}'.format(base_url)
        // }

        println!("Accessing the anchor page URL");

        // アンカーページにアクセス
        let response = client
            .get(&anchor_page_url)
            .set("X-Requested-With", "XMLHttpRequest")
            .set("Wicket-Ajax", "true")
            .set("Wicket-Ajax-BaseURL", &url)
            .call();

        if (response.is_err()) {
            panic!("Failed to access the anchor page URL");
        }

        let response = response.unwrap();
        let html = response.into_string().unwrap();
        // print!("HTML: {}", html);

        // アンカーページのレスポンスの解析
        // let page_image_url = parse_image_url::get_page_image_url(&html).unwrap_or_else(|| {
        //     println!("Failed to parse the page image URL");
        //     println!("HTML: {}", html);
        //     panic!("Failed to parse the page image URL");
        // });

        let page_image_url = match parse_image_url::get_page_image_url(&html) {
            Ok(url) => url,
            Err(e) => {
                println!("Failed to parse the page image URL: {:?}", e);
                println!("HTML: {}", html);
                panic!("Failed to parse the page image URL");
            }
        };

        println!("Page image URL: {}", page_image_url);

        // すこし待機
        std::thread::sleep(std::time::Duration::from_secs(0.5));

        // ファイルのダウンロード
        let response = client.get(&page_image_url).call();
        if (response.is_err()) {
            panic!("Failed to download the image file");
        }

        let response = response.unwrap();
    }

    println!("Done");
}
