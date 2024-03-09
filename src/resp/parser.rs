use super::data_type::DataType;

#[derive(Debug)]
pub struct Parser<'a> {
    data: &'a [u8],
    position: usize,
    read_position: usize,
    b: u8,
}

type R<T> = Result<T, String>;

impl<'a> Parser<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        let mut p = Self {
            data,
            read_position: 0,
            position: 0,
            b: 0,
        };

        p.next_byte();
        p
    }

    fn next_byte(&mut self) {
        self.b = self.peek_byte();
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_byte(&self) -> u8 {
        self.data[self.read_position]
    }

    pub fn parse(&mut self) -> R<DataType> {
        Ok(match self.b {
            b'+' => self.parse_simple_strings()?,
            b'$' => self.parse_bulk_strings()?,
            b'*' => self.parse_arrays()?,
            other => todo!("{other}"),
        })
    }

    fn parse_simple_strings(&mut self) -> R<DataType> {
        self.next_byte();
        let mut buf = String::new();

        while !self.is_cur_crlf() {
            buf.push(self.b.to_ascii_lowercase() as char);
            self.next_byte();
        }

        self.next_byte();
        self.next_byte();

        Ok(DataType::SimpleString(buf))
    }

    fn parse_arrays(&mut self) -> R<DataType> {
        self.next_byte();

        let count = self.read_number();
        let mut data_types = Vec::with_capacity(count.into());

        for _ in 0..count {
            data_types.push(self.parse()?);
        }

        Ok(DataType::Array(data_types))
    }

    fn parse_bulk_strings(&mut self) -> R<DataType> {
        self.next_byte();

        let count = self.read_number();
        let mut s = String::with_capacity(count.into());

        for _ in 0..count {
            s.push(self.b.to_ascii_lowercase() as char);
            self.next_byte();
        }

        self.next_byte();
        self.next_byte();

        Ok(DataType::BulkString(s))
    }

    fn read_number(&mut self) -> usize {
        let mut s = String::new();
        while self.b.is_ascii_digit() {
            s.push(self.b as char);
            self.next_byte();
        }

        self.next_byte();
        self.next_byte();

        s.parse::<usize>().unwrap()
    }

    fn is_cur_crlf(&self) -> bool {
        self.b == b'\r' && self.peek_byte() == b'\n'
    }
}
