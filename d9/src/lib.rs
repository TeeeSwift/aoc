use std::{
    fs,
    io::{BufRead, BufReader},
};

pub mod file;
pub mod memory;
pub mod progress_bar;

use memory::Memory;

pub fn parse_input(filename: &str) -> String {
    let f = fs::File::open(format!("src/{}", filename)).unwrap();
    let reader = BufReader::new(f);

    reader.lines().next().unwrap().unwrap()
}

pub fn convert_to_memory(s: String) -> Memory {
    let mut files: Vec<file::File> = vec![];
    let mut vec: Vec<Option<usize>> = vec![];

    let mut index: usize = 0;
    let mut file_id: usize = 0;
    let mut bit_type: BitType = BitType::File;

    for char in s.chars() {
        let digit = char.to_string().parse::<usize>().unwrap();

        match bit_type {
            BitType::File => {
                // append to string
                let _v = vec![Some(file_id); digit];
                vec.extend(_v);
                files.push(file::File {
                    id: file_id,
                    len: digit,
                    index,
                });

                // increment file id
                file_id += 1;

                // increment character count
                index += digit;

                // toggle bit type
                bit_type = BitType::FreeMemory;
            }
            BitType::FreeMemory => {
                // append to string
                let _v = vec![None::<usize>; digit];
                vec.extend(_v);

                // increment character count
                index += digit;

                // toggle bit type
                bit_type = BitType::File
            }
        }
    }

    Memory::new(vec, files)
}

pub enum BitType {
    File,
    FreeMemory,
}
