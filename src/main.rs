use std::fs;


const PATH:&str = "/proc/meminfo";

fn main() {
    let memory_info = fs::read_to_string(PATH).unwrap();
    for lines in memory_info.lines(){
        let mut split = lines.split_whitespace();
        let (key,value) = (split.next().unwrap(),split.next().unwrap());
        println!("key is {} value is {}",key,value);

    }
}
