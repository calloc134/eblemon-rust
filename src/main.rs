use generate_anchor_page_url::generate_anchor_page_url;
use parse_metadata::extract_metadata;
mod generate_anchor_page_url;
mod parse_image_url;
mod parse_metadata;
use dialoguer::Input;
use log::{debug, error, info};
use ureq::agent;

fn main() {
    // ロガーの初期化
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    // まずURLを取得
    // info!("Please input the URL of the target page");
    // input! {
    //     url: String,
    // }
    let url = Input::<String>::new()
        .with_prompt("Please input the URL of the target page")
        .interact()
        .unwrap_or_else(|e| {
            error!("Failed to get the URL: {:?}", e);
            panic!("Failed to get the URL");
        });

    // セッションを作成
    let client = agent();
    info!("Successfully created a session. Start accessing the URL");

    // アクセス
    let response = client.get(&url).call().unwrap_or_else(|e| {
        error!("Failed to access the URL: {:?}", e);
        panic!("Failed to access the URL");
    });

    info!("Successfully accessed. Start extracting metadata");

    let url = response.get_url().to_string();
    let html = response.into_string().unwrap();

    // メタデータを取得
    let metadata = extract_metadata(&html).unwrap();
    info!(
        "Title: {}, Total pages: {}",
        metadata.title, metadata.total_pages
    );

    info!("Start downloading the image files...");
    for i in 1..=metadata.total_pages {
        // アンカーページのURLを生成
        let anchor_page_url = generate_anchor_page_url(&url, i);
        debug!("Anchor page URL: {}", anchor_page_url);

        // アンカーページにアクセス
        let response = client
            .get(&anchor_page_url)
            .set("X-Requested-With", "XMLHttpRequest")
            .set("Wicket-Ajax", "true")
            .set("Wicket-Ajax-BaseURL", &url)
            .call()
            .unwrap_or_else(|e| {
                error!("Failed to access the anchor page URL: {:?}", e);
                panic!("Failed to access the anchor page URL");
            });

        let html = response.into_string().unwrap();

        let page_image_url = parse_image_url::get_page_image_url(&html).unwrap_or_else(|e| {
            error!("Failed to parse the page image URL: {:?}", e);
            debug!("HTML: {}", html);
            panic!("Failed to parse the page image URL");
        });

        info!("Page image URL: {}", page_image_url);

        // すこし待機
        std::thread::sleep(std::time::Duration::from_millis(500));

        // ファイルのダウンロード
        let response = client.get(&page_image_url).call().unwrap_or_else(|e| {
            error!("Failed to download the image file: {:?}", e);
            panic!("Failed to download the image file");
        });
        let mut output_image =
            std::fs::File::create(format!("page{}.jpg", i)).unwrap_or_else(|e| {
                error!("Failed to create the image file: {:?}", e);
                panic!("Failed to create the image file");
            });

        std::io::copy(&mut response.into_reader(), &mut output_image).unwrap_or_else(|e| {
            error!("Failed to write the image file: {:?}", e);
            panic!("Failed to write the image file");
        });
    }

    info!("All image files have been downloaded");
}
