use crate::pattern::Pattern;
use nom::branch::{alt, permutation};
use nom::character::complete::{anychar, char};
use nom::combinator::{map, opt};
use nom::IResult;

pub fn choose_parser(s: &str) -> IResult<&str, Pattern> {
    unimplemented!()
}

pub fn brackets_parser(s: &str) -> IResult<&str, Pattern> {
    alt((
        map(
            permutation((char('('), choose_parser, char(')'))),
            |(_, choose, _)| choose,
        ),
        literal_parser,
    ))(s)
}

pub fn literal_parser(s: &str) -> IResult<&str, Pattern> {
    let (no_used, c) = anychar(s)?;
    Ok((no_used, Pattern::Literal(c)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_parser_test() {
        let (_, ast) = literal_parser("a").unwrap();
        assert_eq!(ast, Pattern::Literal('a'));
    }
}
