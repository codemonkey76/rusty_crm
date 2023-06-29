use std::fs::{self, File};
use std::io::Error;
use simplelog::*;
use directories::ProjectDirs;

pub fn setup_logger() -> Result<(), Error> {
    let proj_dirs = ProjectDirs::from("au", "popplestones", "RustyCrm").expect("Failed to get project directory");
    let log_path = proj_dirs.config_dir().join("log.log");
    fs::create_dir_all("log")?;
    let log_file = File::create(log_path)?;
    WriteLogger::init(LevelFilter::Info, Config::default(), log_file).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    Ok(())
}

