use std::{default, fs};
use colored::*;

const PATH: &str = "/proc/meminfo";

#[derive(Default)]
struct MemoryInfo {
    pub mem_total: u64,
    pub mem_free: u64,
    pub mem_avail: u64,
}

impl MemoryInfo {
    fn new() -> Self {
        Self::default()
    }

    fn get_info(&mut self) {
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

    fn print_info(&self) {
        println!("{} {} \n{} {}\n{} {}",
        "Total Memory".red().bold(), self.mem_total,
        "Free Memory".red().bold(), self.mem_free,
        "Avaiable Memory".red().bold(), self.mem_avail);
    }
}

fn main() {
    let mut memory = MemoryInfo::new();
    memory.get_info();
    memory.print_info();
    
}
