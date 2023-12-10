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

#[cfg(test)]
mod tests {
    use crate::match_and_move_1;
    use winnow::{error::InputError, Parser};

    #[test]
    pub fn match_and_move_1_works() {
        let input = &mut "ottootto";
        let res = match_and_move_1::<_, InputError<_>>("otto").parse_next(input);
        assert_eq!(res, Ok("otto"));
        assert_eq!(*input, "ttootto");
    }
}
