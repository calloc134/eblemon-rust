use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub base_ebook_host: String,
    pub download_base_dir: String,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string("eblemon.toml")?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}
