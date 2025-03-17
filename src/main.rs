mod constants;
mod fetch;
mod next_page_url;
mod parse_image_url;
mod parse_metadata;
mod sanitize_to_filename;
use constants::{BASE_EBOOK_HOST, DOWNLOAD_BASE_DIR};
use dialoguer::Input;
use indicatif::ProgressBar;
use log::{error, info};
use ureq::agent;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ロガーの初期化
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    // まずURLを取得
    let url = Input::<String>::new()
        .with_prompt("Please input the URL of the target page")
        .interact()
        .map_err(|e| {
            error!("Failed to get the URL: {:?}", e);
            e
        })?;

    // セッションを作成
    let client = agent();
    info!("Successfully created a session. Start accessing the URL");

    // GETリクエスト: データフェッチ関数に切り出し
    let (url, metadata) = fetch::fetch_metadata(&client, &url)?;

    info!(
        "Title: {}, Total pages: {}",
        metadata.title, metadata.total_pages
    );

    info!("Start downloading the image files...");

    // ダウンロード用のディレクトリを作成
    let download_dir = format!(
        "{}/{}",
        DOWNLOAD_BASE_DIR,
        sanitize_to_filename::sanitize_to_filename(&metadata.title)
    );

    // ディレクトリが存在しない場合は作成
    if !std::path::Path::new(&download_dir).exists() {
        std::fs::create_dir(&download_dir).map_err(|e| {
            error!("Failed to create the directory: {:?}", e);
            e
        })?;
    }

    // 次のページにアクセスするためのURLを作成
    let next_page_url = next_page_url::create_next_page_url(&url);

    // プログレスバーの初期化
    let bar = ProgressBar::new((metadata.total_pages - 1).into());

    // 先頭の白ページを飛ばすために1ページ目をスキップ
    fetch::skip_first_page(&client, &next_page_url, BASE_EBOOK_HOST)?;

    // -2でアクセスすると何故かうまく行く。ここは根拠がない
    for i in 0..(metadata.total_pages - 2) {
        fetch::fetch_and_download_image(
            &client,
            &next_page_url,
            &[
                ("id100_hf_0", ""),
                ("changeScale", "1"),
                ("pageNumEditor", i.to_string().as_str()),
                ("nextPageSubmit", "1"),
            ],
            BASE_EBOOK_HOST,
            &download_dir,
            i + 1,
        )?;
        bar.inc(1);
    }

    info!("All image files have been downloaded");
    bar.finish_with_message("Finished");
    Ok(())
}
