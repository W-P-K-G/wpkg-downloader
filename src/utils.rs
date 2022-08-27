use std::{io::{self, Cursor}, fs::{File, self}, process::Command, os::windows::process::CommandExt};
use std::path::Path;
use anyhow::Context;
use platform_dirs::AppDirs;

const CREATE_NO_WINDOW: u32 = 0x08000000;

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
pub fn get_dir() -> anyhow::Result<String> {
        let app_dirs = AppDirs::new(Some("WPKG"), true).context("Error")?;
        let config_dir = app_dirs.config_dir.display().to_string();
        if !Path::new(&config_dir).exists() {
            fs::create_dir(&config_dir)?;
        }
        Ok(config_dir)
}
pub fn run_process(exe: &str, args: Vec<&str>, wait: bool) -> anyhow::Result<()> {
    let mut full_command: Vec<&str> = vec![];

    #[cfg(target_os = "windows")]
    {
        full_command.push("cmd.exe");
        full_command.push("/c");
        if !wait {
            full_command.push("start");
        }
    }

    full_command.push(exe);
    for arg in args {
        full_command.push(arg);
    }

    let mut command = Command::new(full_command[0].clone());
    command.args(full_command[1..full_command.len()].to_vec());

    #[cfg(target_os = "windows")]
    command.creation_flags(CREATE_NO_WINDOW);

    if wait {
        command.output()?;
    } else {
        command.spawn()?;
    }

    Ok(())
}