mod constants;
mod fetch;
mod next_page_url;
mod parse_image_url;
mod parse_metadata;
mod sanitize_to_filename;
use dialoguer::Input;
use indicatif::ProgressBar;
use log::{error, info};
use ureq::agent;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ロガーの初期化
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // 設定を読み込む
    let config = constants::load_config()?;

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

    // メタデータ取得
    let (url, metadata) = fetch::get_metadata_from_url(&client, &url)?;

    info!(
        "Title: {}, Total pages: {}",
        metadata.title, metadata.total_pages
    );

    info!("Start downloading the image files...");

    // ダウンロード用のディレクトリを作成
    let download_dir = format!(
        "{}/{}",
        config.download_base_dir,
        sanitize_to_filename::sanitize_to_filename(&metadata.title)
    );
    std::fs::create_dir_all(&download_dir).map_err(|e| {
        error!("Failed to create the directory: {:?}", e);
        e
    })?;

    // 次のページにアクセスするためのURLを作成
    let next_page_url = next_page_url::create_next_page_url(&url);

    // 先頭の白ページを飛ばすために1ページ目をスキップ
    fetch::skip_initial_page(&client, &next_page_url, &config.base_ebook_host)?;

    // 取得の開始
    // ページ数の計算
    let pages_to_download = metadata.total_pages.saturating_sub(2);
    // プログレスバーの初期化
    let bar = ProgressBar::new(pages_to_download.into());
    for i in 0..pages_to_download {
        fetch::download_image_for_page(
            &client,
            &next_page_url,
            &config.base_ebook_host,
            &download_dir,
            i + 1,
        )?;
        bar.inc(1);
    }

    info!("All image files have been downloaded");
    bar.finish_with_message("Finished");
    Ok(())
}
