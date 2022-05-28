// #![windows_subsystem = "windows"]
#[cfg(target_os = "windows")]
mod win;

#[cfg(target_os = "windows")]
use crate::{win::Window};

use std::{thread, time::Duration};

fn main() {
    loop {
        let window = Window::foreground_window().unwrap();
        
        println!("{}", window.title().unwrap());
        println!("{}", window.process_id().unwrap());
        println!("{}", window.process_name().unwrap());

        thread::sleep(Duration::from_secs(1));
    }
}
