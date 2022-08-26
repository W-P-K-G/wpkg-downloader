use std::{io::{self, Cursor}, fs::File};
use std::path::Path;

pub async fn download_string(url: &str) -> anyhow::Result<String>{
    Ok(reqwest::get(url).await?.text().await?)
}
pub async fn download_from_url(url: &str, path: &str) -> anyhow::Result<()>{
    let resp = reqwest::get(url).await?;
    let mut out = File::create(path)?;
    let mut content =  Cursor::new(resp.bytes().await?);
    io::copy(&mut content, &mut out)?;
    Ok(())
}
fn get_dir() -> anyhow::Result<String> {
        let app_dirs = AppDirs::new(Some("WPKG"), true).context("Error")?;
        let config_dir = app_dirs.config_dir.display().to_string();
        if !Path::new(&config_dir).exists() {
            fs::create_dir(&config_dir)?;
        }
        Ok(config_dir)
}