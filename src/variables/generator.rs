/// Type represents a generator of variable names for the purpose of renaming
pub struct VarGen {
    current: Vec<char>,
}

impl VarGen {
    /// Initiate the generator with 'a'
    pub fn new() -> Self {
        Self { current: vec!['a'] }
    }
    /// Calculate the next character
    fn next_char(c: char) -> char {
        match c {
            'a'..='y' => (c as u8 + 1) as char,
            'z' => 'A',
            'A'..='Y' => (c as u8 + 1) as char,
            'Z' => 'a',
            _ => panic!("Unexpected character"),
        }
    }
}

impl Iterator for VarGen {
    type Item = String;

    /// Generate the names
    /// Starts with single characters a...z
    /// And then moves into longer strings like aa...az etc
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current.iter().collect::<String>();
        for i in (0..self.current.len()).rev() {
            if self.current[i] != 'z' {
                self.current[i] = Self::next_char(self.current[i]);
                return Some(result);
            } else {
                self.current[i] = 'a';
            }
        }
        self.current.insert(0, 'a');
        Some(result)
    }
}
