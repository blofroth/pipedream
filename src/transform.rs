use std::io::{BufReader, Read, BufRead};
use std::io;
use std::cmp;

use rocket::{Data};
use rocket::data::DataStream;

pub trait LinesTransform {
    fn transform(&mut self, line: &String) -> TfResult;
}

pub enum TfResult {
    Yield(String),
    Skip,
    Stop
}

pub struct LinesTransformer<T: LinesTransform> {
    reader: BufReader<DataStream>,
    finished: bool,
    curr_line: String,
    num_read: usize,
    transform: T
}

impl<T: LinesTransform> LinesTransformer<T> {
    pub fn new(data: Data, transform: T) -> LinesTransformer<T> {
        LinesTransformer {
            reader: BufReader::new(data.open()),
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

impl <T: LinesTransform> Read for LinesTransformer<T> {
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