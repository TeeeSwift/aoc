#[derive(Debug)]
pub struct Board {
    pub board: Vec<String>,
    pub word: String,
    width: usize,
    height: usize,
}

impl Board {
    pub fn new(v: Vec<String>, word: String) -> Self {
        return Board {
            word,
            width: v[0].len(),
            height: v.len(),
            board: v,
        };
    }

    pub fn join_range(&self, range: Vec<(i32, i32)>) -> String {
        range
            .iter()
            .map(|(y, x)| {
                if *y < 0 || *x < 0 || *y >= self.height as i32 || *x >= self.width as i32 {
                    return '_';
                } else {
                    return self.board[*y as usize].chars().nth(*x as usize).unwrap();
                }
            })
            .collect()
    }
}
