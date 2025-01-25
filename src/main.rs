use indicatif::ProgressBar;
use parse_metadata::extract_metadata;
mod parse_image_url;
mod parse_metadata;
use dialoguer::Input;
use log::{error, info};
use ureq::agent;

const BASE_EBOOK_HOST: &str = "https://elib.maruzen.co.jp";

fn main() {
    // ロガーの初期化
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    // まずURLを取得
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

    // 次のページにアクセスするためのURLを作成
    let next_page_url = format!("{}-1.IBehaviorListener.0-browseForm-nextPageSubmit", url);

    // プログレスバーの初期化
    let bar = ProgressBar::new(metadata.total_pages as u64);

    for i in 1..=metadata.total_pages {
        // 次のページにアクセス
        let response = client
            .post(&next_page_url)
            .set("X-Requested-With", "XMLHttpRequest")
            .set("Wicket-Ajax", "true")
            .set("Wicket-Ajax-BaseURL", &BASE_EBOOK_HOST)
            // パラメータを設定
            .send_form(&[
                ("id100_hf_0", ""),
                ("changeScale", "1"),
                ("pageNumEditor", &i.to_string()),
                ("nextPageSubmit", "1"),
            ])
            // .call()
            .unwrap_or_else(|e| {
                error!("Failed to access the anchor page URL: {:?}", e);
                panic!("Failed to access the anchor page URL");
            });

        let html = response.into_string().unwrap();

        let image_relative_url = parse_image_url::get_page_image_url(&html).unwrap_or_else(|e| {
            error!("Failed to parse the page image URL: {:?}", e);
            // debug!("HTML: {}", html);
            println!("HTML: {}", html);
            panic!("Failed to parse the page image URL");
        });

        info!("Page image URL: {}", image_relative_url);

        let image_url = format!("{}{}", BASE_EBOOK_HOST, image_relative_url);

        // ファイルのダウンロード
        let response = client.get(&image_url).call().unwrap_or_else(|e| {
            error!("Failed to download the image file: {:?}", e);
            panic!("Failed to download the image file");
        });
        let mut output_image = std::fs::File::create(format!("testimage/page{}.jpg", i))
            .unwrap_or_else(|e| {
                error!("Failed to create the image file: {:?}", e);
                panic!("Failed to create the image file");
            });

        std::io::copy(&mut response.into_reader(), &mut output_image).unwrap_or_else(|e| {
            error!("Failed to write the image file: {:?}", e);
            panic!("Failed to write the image file");
        });

        bar.inc(1);
    }

    info!("All image files have been downloaded");
    bar.finish_with_message("Finished");
}
