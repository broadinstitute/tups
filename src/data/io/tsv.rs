use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error as IoError, Write};
use crate::data::io::line_parser::LineParser;
use crate::error::Error;

pub(crate) struct TsvReader {
    line_parser: LineParser,
    pub(crate) header: Vec<String>,
    lines: Box<dyn Iterator<Item=Result<String, IoError>>>,
}

pub(crate) struct TsvWriter<W: Write> {
    inner: W,
}

fn error_missing_col(col: &str) -> Error {
    Error::from(format!("Missing column {}.", col))
}

impl TsvReader {
    pub(crate) fn new<R: BufRead + 'static>(reader: R, line_parser: LineParser)
                                            -> Result<TsvReader, Error> {
        let mut lines = Box::new(reader.lines());
        let header =
            line_parser.parse(&lines.next().ok_or_else(|| {
                Error::from("No header line")
            })??)?;
        Ok(TsvReader { line_parser, header, lines })
    }
    pub(crate) fn from_file(file: &str, line_parser: LineParser) -> Result<TsvReader, Error> {
        Self::new(BufReader::new(File::open(file)?), line_parser)
    }
    pub(crate) fn col_to_i(&self, col: &str) -> Result<usize, Error> {
        self.header.iter().position(|s| s.as_str() == col)
            .ok_or_else(|| { error_missing_col(col) })
    }
    pub(crate) fn cols_to_is(&self, cols: &[String]) -> Result<Vec<usize>, Error> {
        let mut i_by_col: BTreeMap<&String, usize> = BTreeMap::new();
        for (i, col) in self.header.iter().enumerate() {
            if cols.contains(col) {
                i_by_col.insert(col, i);
            }
        }
        let mut is: Vec<usize> = Vec::new();
        for col in cols {
            match i_by_col.get(col) {
                None => { return Err(error_missing_col(col)); }
                Some(i) => { is.push(*i) }
            }
        }
        Ok(is)
    }
}

impl Iterator for TsvReader {
    type Item = Result<Vec<String>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lines.next() {
            None => { None }
            Some(Err(io_error)) => { Some(Err(Error::from(io_error))) }
            Some(Ok(string)) => { Some(self.line_parser.parse(&string)) }
        }
    }
}

impl<W: Write> TsvWriter<W> {
    pub(crate) fn new(inner: W, header: &[String]) -> Result<TsvWriter<W>, Error> {
        let mut tsv_writer = TsvWriter { inner };
        tsv_writer.write(header)?;
        Ok(tsv_writer)
    }
    pub(crate) fn write(&mut self, fields: &[String]) -> Result<(), Error> {
        self.inner.write_fmt(format_args!("{}\n", fields.join("\t")))?;
        Ok(())
    }
}