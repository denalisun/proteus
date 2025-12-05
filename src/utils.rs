use std::{fs::{File, OpenOptions}, io::Write};

use windows::Win32::System::Console::AllocConsole;

pub struct Logger {
    out: File
}

impl Logger {
    pub fn new() -> Logger {
        unsafe { AllocConsole().ok(); }
        Logger { 
            out: OpenOptions::new().write(true).open("CONOUT$").unwrap()
        }
    }

    pub fn print(&mut self, text: &str) {
        writeln!(self.out, "[PROTEUS] {}", text).ok();
    }

    #[allow(dead_code)] // temporary
    pub fn error(&mut self, text: &str) {
        writeln!(self.out, "[PROTEUS_ERR] {}", text).ok();
    }
}