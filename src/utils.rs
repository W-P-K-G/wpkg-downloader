use std::{io::{self, Cursor}, fs::File};

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