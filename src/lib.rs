use san::SAN;

pub mod san;

pub struct PGNReader<'a> {
    data: &'a str,
}

impl<'a> PGNReader<'a> {
    pub fn new(data: &'a str) -> Self {
        Self { data: data.trim() } // start at one to ignore the first "["
    }

    fn get_char(&self, index: usize) -> char {
        self.data[index..index + 1].chars().next().unwrap()
    }

    pub fn read(self, visitor: &mut impl Visitor) {
        for piece in self.data.split("\n\n") {
            visitor.start_game();

            if piece.starts_with('[') {
                for mut header in piece
                    .lines()
                    .map(|header| header.trim_start_matches('[').trim_end_matches(']').split(' ')) {
                    let start = header.next().unwrap(); // TODO: Error handling here
                    let end = header.next().unwrap().trim_matches('"');
                    visitor.header(Header(start, end));
                }
            } else if piece.starts_with('1') {
                for item in piece.split(|char: char| (char == '\n' || char == ' ')) {
                    if
                        item.starts_with(
                            |char: char|
                                char.is_numeric() ||
                                char == '*' ||
                                char == '{' ||
                                char == '}' ||
                                char == '[' ||
                                char == '-' ||
                                char == '#'
                        )
                    {
                        continue;
                    }

                    visitor.san(SAN::new(item));
                }

                visitor.end_game();
            }
        }

        // loop {
        //     visitor.start_game();

        //     loop {
        //         let starting_index = self.index;

        //         while self.get_char(self.index) != ' ' {
        //             self.index += 1;
        //         }

        //         let key = &self.data[starting_index..self.index];

        //         self.index += 2; // Ignore the """ and " "

        //         let starting_index = self.index;

        //         while self.get_char(self.index) != '\n' {
        //             self.index += 1;
        //         }

        //         let value = &self.data[starting_index..self.index - 2];

        //         self.index += 1; // Skip the new line char

        //         visitor.header(Header(key, value));

        //         if self.get_char(self.index) != '[' {
        //             break;
        //         } else {
        //             self.index += 1;
        //         }
        //     }

        //     self.index += 1;

        //     let end_index = match
        //         self.data[self.index..].split("\n").position(|line| line.is_empty())
        //     {
        //         Some(n) => n + self.index,
        //         None => {
        //             break;
        //         }
        //     };

        //     dbg!(end_index);

        //     for item in self.data[self.index..end_index].split(
        //         |char: char| (char == '\n' || char == ' ')
        //     ) {
        //         if item.starts_with(|char: char| (char.is_numeric() || char == '*' || char == '{')) {
        //             continue;
        //         }

        //         visitor.san(SAN::new(item));
        //     }

        //     self.index = end_index;
        // }
    }
}

#[derive(Debug)]
pub struct Header<'a>(&'a str, &'a str);

pub trait Visitor {
    fn start_game(&mut self);
    fn end_game(&mut self);
    fn header(&mut self, header: Header);
    fn san(&mut self, san: SAN);
}