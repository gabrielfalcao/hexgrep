// use clap::{ValueEnum, Parser};
use clap::Parser;
use hex;
use hex::FromHexError;
use iocore::rsvfilematch;
use iocore::open_read;
use std::io::Read;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::fmt::Write;
use iocore::Exception;

#[derive(Debug)]
pub enum Error {
    HexDecodeError(FromHexError),
    IOException(iocore::Exception),
}

impl From<FromHexError> for Error {
    fn from(e: FromHexError) -> Self {
        Error::HexDecodeError(e)
    }
}

impl From<Exception> for Error {
    fn from(e: Exception) -> Self {
        Error::IOException(e)
    }
}

impl std::error::Error for Error{}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::IOException(e) => write!(f, "I/O Core Exception: {}", e),
            Error::HexDecodeError(e) => write!(f, "Hex Decode Exception: {}", e),
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Argv {
    #[arg(value_name = "pattern", required = true, help= "hex")]
    pub pattern: Vec<String>,

    #[arg(long, short)]
    pub progress: bool,
}

pub fn contains_pattern(origin: &[u8], matchsq: &[u8]) -> Option<usize> {
    let limit = matchsq.len();
    if limit > origin.len() {
        return None
    }
    if limit == 0 {
        return None
    }
    let (position, _) = origin.iter().enumerate().find(|(i, _)| {
        if origin.len() < *i+limit {
            return false;
        }
        let matches = &origin[*i..*i+limit] == matchsq;
        matches
    })?;
    Some(position)
}

fn main() -> Result<(), Error>{
    let args = Argv::parse();
    let mut pattern = Vec::<u8>::new();
    for p in args.pattern.iter().map(|s|s.replace("0x", "")).filter(|s| hex::decode(s).is_ok()).collect::<Vec<String>>() {
        pattern.extend(hex::decode(p)?);
    }
    let limit = pattern.len();
    let bar = if args.progress == true {
        let bar = ProgressBar::new(limit as u64);
        bar.set_prefix(format!("\x1b[1;38;5;208m{}\x1b[0m", hex::encode(pattern.clone())));
        bar.set_style(ProgressStyle::with_template("\x1b[1;38;5;202mSearching pattern {prefix}\n\x1b[1;38;5;33m{msg}").unwrap()
                      .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()));
        Some(bar)
    } else {
        None
    };


    rsvfilematch(
        args.pattern.iter().filter(|s| !hex::decode(s.replace("0x", "")).is_ok()).map(|s| s.clone()).collect::<Vec<String>>(),
        |path| {
            match &bar {
                Some(bar) => {
                    bar.inc(1);
                    bar.set_message(format!("{}", path.display()));
                },
                None => {}
            };

            let spath = format!("{}", path.display());

            let mut buf = Vec::<u8>::new();
            match open_read(&spath) {
                Ok(mut fd) => {
                    match fd.read_to_end(&mut buf) {
                        Ok(_size) => {
                            match contains_pattern(&buf, &pattern.clone()) {
                                Some(position) => {
                                    match &bar {
                                        Some(bar) => {
                                            bar.suspend(||{
                                                println!("\x1b[1;38;5;220m{}:\x1b[1;38;5;208m{}\x1b[0m", position, spath);
                                            });
                                        },
                                        None => {}
                                    };

                                    true
                                },
                                None => false
                            }
                        },
                        Err(_) => false
                    }
                },
                Err(_) => {
                    false
                }
            }
        })?;
       Ok(())
}
