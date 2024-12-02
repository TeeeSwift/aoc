pub struct Report {
    pub values: Vec<i32>,
}

impl Report {
    pub fn new(vec: Vec<i32>) -> Self {
        Self { values: vec }
    }

    pub fn is_valid(&self) -> bool {
        let v = &self.values;

        let increasing: bool = (v[1] - v[0]).is_positive();
        let mut valid: bool = true;

        for index in 1..v.len() {
            let difference = v[index] - v[index - 1];

            if difference.is_positive() != increasing
                || difference.abs() < 1
                || difference.abs() > 3
            {
                valid = false;
                break;
            }
        }

        valid
    }

    pub fn try_adjustment(&self) -> bool {
        for i in 0..self.values.len() {
            let mut alternative: Vec<i32> = self.values.clone();
            alternative.remove(i);
            let report = Report::new(alternative);
            if report.is_valid() {
                return true;
            }
        }

        false
    }
}
