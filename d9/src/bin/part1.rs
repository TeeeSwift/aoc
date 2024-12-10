use d9::memory::Memory;
use d9::progress_bar;
use d9::{convert_to_memory, parse_input};

pub fn main() {
    let pb = progress_bar::create_pb();

    let string: String = parse_input("input");

    let mut mem: Memory = convert_to_memory(string);

    mem.compress_by_cell(&pb);

    let sum = mem.check_sum();

    pb.finish_with_message("Done!");
    println!("{}", sum);
}
