use crate::{versions::Versions, utils::{self, download_from_url}, globals::CURRENT_VERSION};

pub async fn update(path: &str) -> anyhow::Result<()> {
    // Parase Json
    let ver: Vec<Versions> = Versions::parse(
        &utils::download_string("https://raw.githubusercontent.com/W-P-K-G/JSONFiles/master/Versions.json").await?)?;

    // Get Nevest version
    let nevest_ver = ver[ver.len()-1].clone();

    // Update
    if CURRENT_VERSION != nevest_ver.version {
        download_from_url(&nevest_ver.link, path).await?;
    } else {
        println!("Nevest version!, skipping update");
    }
    Ok(())
}