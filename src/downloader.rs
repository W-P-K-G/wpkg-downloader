use crate::{versions::Versions, utils::{self, download_from_url}};

pub async fn download_wpkg(path: &str) -> anyhow::Result<()> {
    // Parase Json
    let ver: Vec<Versions> = Versions::parse(
        &utils::download_string("https://raw.githubusercontent.com/W-P-K-G/JSONFiles/master/Versions.json").await?)?;

    // Get Nevest version
    let nevest_ver = ver[ver.len()-1].clone();

    // Update
    download_from_url(&nevest_ver.link, path).await?;
    Ok(())
}