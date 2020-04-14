use std::env;

#[derive(Debug)]
pub enum ParseError {
    TooFewArgs,
    TooManyArgs,
    InvalidInteger(String),
}

#[derive(Debug)]
pub struct Frame {
    pub width: usize,
    pub height: usize,
}

struct ParseArgs(env::Args);

impl ParseArgs {
    fn new() -> ParseArgs {
        ParseArgs(env::args())
    }
    fn require_arg(&mut self) -> Result<String, ParseError> {
        match self.0.next() {
            None => Err(ParseError::TooFewArgs),
            Some(arg) => Ok(arg),
        }
    }
    fn require_no_arg(&mut self) -> Result<(), ParseError> {
        match self.0.next() {
            Some(_) => Err(ParseError::TooManyArgs),
            None => Ok(()),
        }
    }
}

fn parse_usize(s: &str) -> Result<usize, ParseError> {
    match s.parse() {
        Err(_) => Err(ParseError::InvalidInteger(s.into())),
        Ok(x) => Ok(x),
    }
}

pub fn parse_args() -> Result<Frame, ParseError> {
    let mut args = ParseArgs::new();

    args.require_arg()?;

    let width = args.require_arg()?;
    let height = args.require_arg()?;
    args.require_no_arg()?;
    let width = parse_usize(&width)?;
    let height = parse_usize(&height)?;

    Ok(Frame{
        width,
        height,
    })

}
