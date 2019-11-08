use std::io::{BufReader, Read, BufRead, Cursor};
use std::io;
use std::cmp;
use remote::RemoteClient;
use reqwest::Response;
use serde::Serialize;
use rocket::data::DataStream;

pub trait ReadSend : Read + Send + 'static {}
pub type CharStream = Box<dyn ReadSend>;

impl ReadSend for Response {}
impl ReadSend for Cursor<Vec<u8>> {}
impl<T: LinesTransform + Send + 'static> ReadSend for LinesTransformer<T> {}
impl ReadSend for DataStream {}

pub trait Command : Serialize {
    fn name(&self) -> String;
    fn execute_local(&self, input: CharStream) -> Result<CharStream, String>;

    fn execute(&self, input: CharStream, remote: bool, client: &RemoteClient) -> Result<CharStream, String> {
        if remote {
            self.execute_remote(input, client)
        } else {
            self.execute_local(input)
        }
    }
    fn execute_remote(&self, input: CharStream, client: &RemoteClient) 
        -> Result<CharStream, String> {
        Ok(Box::new(client.call_remote(input, &self.name(), &self)?))
    }
}

pub trait LinesTransform {
    fn transform(&mut self, line: &str) -> TfResult;
}

pub enum TfResult {
    Yield(String),
    Skip,
    Stop
}

pub struct LinesTransformer<T: LinesTransform + Send> {
    reader: BufReader<CharStream>,
    finished: bool,
    curr_line: String,
    num_read: usize,
    transform: T
}

impl<T: LinesTransform + Send> LinesTransformer<T> {
    pub fn new(input: CharStream, transform: T) -> LinesTransformer<T> {
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
            Ok(0) => {
                self.finished = true;
                Ok(())
            },
            Ok(_) => {       
                match self.transform.transform(&self.curr_line) {
                    TfResult::Yield(line) => {
                        self.num_read = 0;
                        self.curr_line = line;
                        if !self.curr_line.ends_with('\n') {
                            self.curr_line.push('\n');
                        }
                    }
                    TfResult::Skip => {
                        self.curr_line.clear();
                        self.num_read = 0;
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

impl <T: LinesTransform + Send> Read for LinesTransformer<T> {
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

pub fn empty_stream() -> CharStream {
    Box::new(Cursor::new(Vec::new()))
}