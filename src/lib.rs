use san::SAN;

pub mod san;

pub struct PGNReader<'a> {
    data: &'a str,
    index: usize,
}

impl<'a> PGNReader<'a> {
    pub fn new(data: &'a str) -> Self {
        Self { data: data.trim(), index: 1 } // start at one to ignore the first "["
    }

    fn get_char(&self, index: usize) -> char {
        self.data[index..index + 1].chars().next().unwrap()
    }

    pub fn read(mut self, visitor: &mut impl Visitor) {
        visitor.start_game();

        loop {
            let starting_index = self.index;

            while self.get_char(self.index) != ' ' {
                self.index += 1;
            }

            let key = &self.data[starting_index..self.index];

            self.index += 2; // Ignore the """ and " "

            let starting_index = self.index;

            while self.get_char(self.index) != '\n' {
                self.index += 1;
            }

            let value = &self.data[starting_index..self.index - 2];

            self.index += 1; // Skip the new line char

            visitor.header(Header(key, value));

            if self.get_char(self.index) != '[' {
                break;
            } else {
                self.index += 1;
            }
        }

        self.index += 1;

        for item in self.data[self.index..].split(|char: char| (char == '\n' || char == ' ')) {
            if item.starts_with(|char: char| (char.is_numeric() || char == '*' || char == '{')) {
                continue;
            }

            visitor.san(SAN::new(item));
        }
    }
}

#[derive(Debug)]
pub struct Header<'a>(&'a str, &'a str);

pub trait Visitor {
    fn start_game(&mut self);
    fn header(&mut self, header: Header);
    fn san(&mut self, san: SAN);
}