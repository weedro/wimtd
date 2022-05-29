mod db;
mod err;
mod config;

#[cfg(target_os = "windows")]
mod win;

use config::Config;
use db::Storage;
use err::Error;
use reqwest::{StatusCode, header::{AUTHORIZATION, CONTENT_TYPE}};

use crate::db::Record;
#[cfg(target_os = "windows")]
use crate::{win::Window};

use std::{thread, time::Duration, path::Path, env};

const SYNC_INTERVAL: u32 = 300;

fn update(storage: &Storage) -> Result<(), Error> {
    let window = Window::foreground_window()?;

    let title = window.title()?;
    let process_name = window.process_name()?;

    match storage.last_record() {
        Ok(Some(rec)) if rec.title == title && rec.process_name == process_name => {
            storage.update_record(rec.start_time)?;
        },
        _ => {
            log::info!("Found new window: '{}'", title);
            storage.insert_record(title, process_name)?;
        }
    }

    Ok(())
}

fn sync(storage: &Storage, config: &Config) -> Result<(), Error> {
    let offset = storage.last_synced_record()?;

    let mut records: Vec<Record> = storage.get_records(offset)?;
    log::info!("Synchronizing {} records...", records.len());

    records.iter_mut().for_each(|r| {
        r.start_time = r.start_time.replace(" ", "T");
        r.process_name = Path::new(&r.process_name).file_name().unwrap().to_os_string().into_string().unwrap();
    });

    let body = serde_json::to_string(&records)?;

    let client = reqwest::blocking::Client::new();

    let response = client
        .post(&config.sync.url)
        .header(AUTHORIZATION, format!("Bearer {}", &config.sync.access_token))
        .header(CONTENT_TYPE, "application/json")
        .body(body)
        .send()?;

    let status = response.status();
    if status != StatusCode::OK {
        return Err(Error::Sync(response));
    }

    log::info!("Synchronized successfully");
    storage.add_sync_record(offset + records.len())?;
    Ok(())
}

fn main() {
    let config = config::read_config("config.toml");

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", &config.log_level);
    }
    env_logger::init();
    
    let storage = Storage::init(&config.db_path).expect("Failed to initialize database");

    let mut tick = 0;
    loop {
        if let Err(err) = update(&storage) {
            log::error!("{:?}", err);
        }

        if tick >= SYNC_INTERVAL && config.sync.enabled {
            tick = 0;
            if let Err(err) = sync(&storage, &config) {
                log::error!("{:?}", err);
            }
        }

        tick += 1;
        thread::sleep(Duration::from_secs(1));
    }
}
