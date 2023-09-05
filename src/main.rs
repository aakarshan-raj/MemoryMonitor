use std::fs;


const PATH:&str = "/proc/memoinfo";

fn main() {
    let memory_info = fs::read_to_string(PATH).unwrap();
    println!("{}",memory_info);
}
