use indicatif::ProgressBar;

use crate::file::File;
pub struct Memory {
    pub files: Vec<File>,
    pub cells: Vec<Option<usize>>,
    max_empty: Option<usize>,
}

impl Memory {
    pub fn new(cells: Vec<Option<usize>>, files: Vec<File>) -> Self {
        Self {
            files,
            cells,
            max_empty: None,
        }
    }

    pub fn next_empty(&self, len: usize) -> Option<usize> {
        let mut consecutive_empty_cells: usize = 0;
        let mut starting_index: Option<usize> = None;

        for i in 0..self.cells.len() {
            match self.cells[i] {
                None => {
                    if starting_index.is_none() {
                        starting_index = Some(i)
                    }
                    consecutive_empty_cells += 1;
                }
                Some(_) => {
                    consecutive_empty_cells = 0;
                    starting_index = None;
                    continue;
                }
            }

            if consecutive_empty_cells == len {
                break;
            }
        }

        starting_index
    }

    pub fn to_string(&self) -> String {
        self.cells
            .iter()
            .map(|x| match x {
                Some(y) => format!("{}", y),
                None => ".".to_string(),
            })
            .collect::<String>()
    }

    pub fn compress_by_cell(&mut self, pb: &ProgressBar) {
        // get indexes of empty cells
        let mut empty_cells: Vec<usize> = vec![];
        // get index and value of full cells
        let mut file_cells: Vec<(usize, usize)> = vec![];

        for (index, cell) in self.cells.iter().enumerate() {
            match cell {
                Some(x) => file_cells.insert(0, (index, *x)),
                None => empty_cells.push(index),
            }
        }

        for (index, value) in file_cells.iter() {
            if empty_cells.len() == 0 {
                break;
            };

            let destination_cell = empty_cells.remove(0);
            if *index <= destination_cell {
                break;
            };

            self.cells[destination_cell] = Some(*value);
            self.cells[*index] = None;

            let percent: f64 = (destination_cell as f64 / file_cells.len() as f64) * 100 as f64;
            pb.set_position(percent as u64);
        }
    }

    pub fn compress_by_file(&mut self, pb: &ProgressBar) {
        // for each file
        let mut i = self.files.len() - 1;

        while i > 0 {
            let file = self.files[i];

            let len = file.len;

            // if file length is longer than the max empty we have knowledge of, just skip it
            if match self.max_empty {
                None => false,
                Some(x) => x < len,
            } {
                i -= 1;
                continue;
            }

            // find next instance of that many slots
            let destination_index = self.next_empty(len);

            match destination_index {
                Some(x) if x >= file.index => {
                    self.max_empty = Some(len - 1);
                    i -= 1;
                }
                Some(x) => {
                    for i in 0..file.len {
                        self.cells[x + i] = Some(file.id);
                        self.cells[file.index + i] = None;
                    }
                    i -= 1;
                }
                None => {
                    i -= 1;
                    continue;
                }
            }

            let percent: f64 =
                ((self.files.len() - i) as f64 / self.files.len() as f64) * 100 as f64;
            pb.set_position(percent as u64);
        }
    }

    pub fn check_sum(&self) -> usize {
        let mut total: usize = 0;

        for (index, value) in self.cells.iter().enumerate().filter(|(_, v)| v.is_some()) {
            total += index * value.unwrap();
        }

        return total;
    }
}
