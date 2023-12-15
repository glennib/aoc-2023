use aoc_cache::get;
use std::path::Path;
use winnow::{
    error::{ErrorKind, ParserError},
    token::any,
    PResult, Parser,
};

pub const MY_COOKIE: &str = include_str!("my.cookie");

pub fn match_and_move_1<'s, O, E: ParserError<&'s str>>(
    mut parser: impl Parser<&'s str, O, E>,
) -> impl FnMut(&mut &'s str) -> PResult<O> {
    move |input: &mut &'s str| match parser.parse_peek(*input) {
        Ok((_remaining, output)) => {
            let _res: PResult<char> = any.parse_next(input);
            Ok(output)
        }
        Err(_) => Err(ParserError::from_error_kind(input, ErrorKind::Slice)),
    }
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn day_number(file: &str) -> &str {
    let prefixed_number = Path::new(file)
        .file_stem()
        .and_then(|f| f.to_str())
        .unwrap();
    prefixed_number.strip_prefix('0').unwrap_or(prefixed_number)
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn get_input(day_number: &str) -> String {
    get(
        &format!("https://adventofcode.com/2023/day/{day_number}/input"),
        MY_COOKIE,
    )
    .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{day_number, match_and_move_1};
    use winnow::{error::InputError, Parser};

    #[test]
    pub fn match_and_move_1_works() {
        let input = &mut "ottootto";
        let res = match_and_move_1::<_, InputError<_>>("otto").parse_next(input);
        assert_eq!(res, Ok("otto"));
        assert_eq!(*input, "ttootto");
    }

    #[test]
    pub fn get_right_advent_day_number() {
        assert_eq!("7", day_number("/directory/files/07.rs"));
        assert_eq!("24", day_number("/directory/files/24.rs"));
    }
}
