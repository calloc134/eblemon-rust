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

    // Create the directory (and parents) in one call.
    std::fs::create_dir_all(&download_dir).map_err(|e| {
        error!("Failed to create the directory: {:?}", e);
        e
    })?;

    // 次のページにアクセスするためのURLを作成
    let next_page_url = next_page_url::create_next_page_url(&url);

    // 先頭の白ページを飛ばすために1ページ目をスキップ
    fetch::skip_first_page(&client, &next_page_url, BASE_EBOOK_HOST)?;

    // 取得の開始
    // Calculate the number of pages to download.
    let pages_to_download = metadata.total_pages.saturating_sub(2);
    // プログレスバーの初期化
    let bar = ProgressBar::new(pages_to_download.into());
    for i in 0..pages_to_download {
        fetch::download_page_image(
            &client,
            &next_page_url,
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
