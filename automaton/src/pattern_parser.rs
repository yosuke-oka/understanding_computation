use crate::pattern::Pattern;
use nom::branch::alt;
use nom::character::complete::{anychar, char};
use nom::combinator::{eof, map};
use nom::sequence::tuple;
use nom::IResult;

pub fn choose_parser(s: &str) -> IResult<&str, Pattern> {
    alt((
        map(
            tuple((concatnate_or_empty_parser, char('|'), choose_parser)),
            |(concatnate_or_empty, _, choose)| Pattern::Choose {
                first: Box::new(concatnate_or_empty),
                second: Box::new(choose),
            },
        ),
        concatnate_or_empty_parser,
    ))(s)
}

fn concatnate_or_empty_parser(s: &str) -> IResult<&str, Pattern> {
    alt((concatnate_parser, empty_parser))(s)
}

fn concatnate_parser(s: &str) -> IResult<&str, Pattern> {
    alt((
        map(
            tuple((repeat_parser, concatnate_parser)),
            |(repeat, concatnate)| Pattern::Concatnate {
                first: Box::new(repeat),
                second: Box::new(concatnate),
            },
        ),
        repeat_parser,
    ))(s)
}

fn empty_parser(s: &str) -> IResult<&str, Pattern> {
    let (unused, _) = eof(s)?;
    Ok((unused, Pattern::Empty))
}

fn repeat_parser(s: &str) -> IResult<&str, Pattern> {
    alt((
        map(tuple((brackets_parser, char('*'))), |(brackets, _)| {
            Pattern::Repeat(Box::new(brackets))
        }),
        brackets_parser,
    ))(s)
}

fn brackets_parser(s: &str) -> IResult<&str, Pattern> {
    alt((
        map(
            tuple((char('('), choose_parser, char(')'))),
            |(_, choose, _)| choose,
        ),
        literal_parser,
    ))(s)
}

fn literal_parser(s: &str) -> IResult<&str, Pattern> {
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

    #[test]
    fn empty_parser_test() {
        let (_, ast) = empty_parser("").unwrap();
        assert_eq!(ast, Pattern::Empty);
    }
}
