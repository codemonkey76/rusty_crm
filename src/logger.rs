use std::fs::{self, File};
use std::io::Error;
use simplelog::*;

pub fn setup_logger() -> Result<(), Error> {
    let log_path = "log/log.log";
    fs::create_dir_all("log")?;
    let log_file = File::create(log_path)?;
    WriteLogger::init(LevelFilter::Info, Config::default(), log_file).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    Ok(())
}

