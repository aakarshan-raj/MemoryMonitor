

mod mem;

fn main() {
    let mut memory = mem::MemoryInfo::new();
    memory.get_info();
 
    memory.print_info();

}
