use snafu::prelude::*;
use std::fmt::Debug;

#[derive(Debug, Snafu)]
pub(crate) enum PageParseError<'a> {
    #[snafu(display("Require Info \"{target}\" but not found"))]
    RequireInfoNotFound { target: &'a str },
}

pub(crate) type PageParseResult<'a, T> = Result<T, PageParseError<'a>>;
