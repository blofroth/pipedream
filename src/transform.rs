use std::io::{BufReader, Read, BufRead};
use std::io;
use std::cmp;

pub trait LinesTransform {
    fn transform(&mut self, line: &String) -> TfResult;
}

pub enum TfResult {
    Yield(String),
    Skip,
    Stop
}

pub struct LinesTransformer<T: LinesTransform, I: Read> {
    reader: BufReader<I>,
    finished: bool,
    curr_line: String,
    num_read: usize,
    transform: T
}

impl<T: LinesTransform, I: Read> LinesTransformer<T, I> {
    pub fn new(input: I, transform: T) -> LinesTransformer<T, I> {
        LinesTransformer {
            reader: BufReader::new(input),
            finished: false,
            curr_line: String::new(),
            num_read: 0,
            transform: transform
        }
    }

    fn refill(&mut self) -> io::Result<()> {
        self.curr_line.clear();
        match self.reader.read_line(&mut self.curr_line) {
            Ok(_) => {       
                match self.transform.transform(&self.curr_line) {
                    TfResult::Yield(line) => {
                        self.num_read = 0;
                        self.curr_line = line;
                    }
                    TfResult::Skip => {
                        // leave num read and curr_line as is
                    },
                    TfResult::Stop => {
                        self.finished = true;
                    }
                }
                Ok(())
            },
            Err(e) => Err(e)
        }
    }
}

impl <T: LinesTransform, I: Read> Read for LinesTransformer<T, I> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        while !self.finished && self.num_read == self.curr_line.len() {
            self.refill()?;
        }

        if self.finished {
            return Ok(0);
        }

        let str_bytes = self.curr_line.as_bytes();
        let remaining_bytes = &str_bytes[self.num_read..];
        let num_to_write = cmp::min(buf.len(), remaining_bytes.len());
        let (to_write, _) = remaining_bytes.split_at(num_to_write);
        buf[0..num_to_write].copy_from_slice(&to_write[0..num_to_write]);
        self.num_read += num_to_write;
        Ok(num_to_write)
    }
}