use std::{fs, io::{stdout, Write}, thread, time::Duration};
use colored::*;

const PATH: &str = "/proc/meminfo";

#[derive(Default)]
pub struct MemoryInfo {
    pub mem_total: u64,
    pub mem_free: u64,
    pub mem_avail: u64,
}

impl MemoryInfo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_info(&mut self) {
        let content = fs::read_to_string(PATH).unwrap();
        for lines in content.lines() {
            let mut split = lines.split_whitespace();
            let (key, value) = (split.next().unwrap(), split.next().unwrap());
            match key {
                "MemTotal:"=>{self.mem_total=value.parse().unwrap()},
                "MemFree:"=>{self.mem_free=value.parse().unwrap()},
                "MemAvailable:"=>{self.mem_avail=value.parse().unwrap()}
                _=>{}
            }
        }
    }

    pub fn print_info(&mut self) {
        let mut stdout = stdout();
        loop{
        print!("\r{} {} {} {} {} {}",
        "Total Memory".red().bold(), self.mem_total,
        "Free Memory".red().bold(), self.mem_free,
        "Avaiable Memory".red().bold(), self.mem_avail);
        stdout.flush().unwrap();
        self.get_info();
        thread::sleep(Duration::from_millis(20));
        }
    }
}
