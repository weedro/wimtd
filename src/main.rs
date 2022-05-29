mod db;
mod err;

#[cfg(target_os = "windows")]
mod win;

use db::Storage;
use err::Error;

#[cfg(target_os = "windows")]
use crate::{win::Window};

use std::{thread, time::Duration};

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

fn main() {
    env_logger::init();

    let storage = Storage::init("test.db").expect("Failed to initialize database");

    loop {
        if let Err(err) = update(&storage) {
            log::error!("{:?}", err);
        }

        thread::sleep(Duration::from_secs(1));
    }
}
