use std::any;

use crate::{versions::Versions, utils::{self, download_from_url}};

pub async fn update(path: &str) -> anyhow::Result<()> {
    let ver: Vec<Versions> = Versions::parse(
        &utils::download_string("https://raw.githubusercontent.com/W-P-K-G/JSONFiles/master/Versions.json").await?)?;
    println!("{}, {}", ver[0].version, ver[0].link);
    download_from_url(&ver[ver.len()-1].link, path).await?;
    Ok(())
}