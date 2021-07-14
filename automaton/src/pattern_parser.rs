use crate::pattern::Pattern;
use nom::branch::alt;
use nom::character::complete::{char, satisfy};
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

pub fn parse(s: &str) -> Pattern {
    choose_parser(s).unwrap().1
}

fn choose_parser(s: &str) -> IResult<&str, Pattern> {
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
    Ok((s, Pattern::Empty))
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
    let (no_used, c) = satisfy(|c| c.is_ascii_alphabetic())(s)?;
    Ok((no_used, Pattern::Literal(c)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_parser_test() {
        let (_, ast) = literal_parser("a").unwrap();
        assert_eq!(ast, Pattern::Literal('a'));

        let parsed = literal_parser("|");
        assert_eq!(parsed.ok(), None);
    }

    #[test]
    fn empty_parser_test() {
        let (_, ast) = empty_parser("").unwrap();
        assert_eq!(ast, Pattern::Empty);
    }

    #[test]
    fn brackets_parser_test() {
        let (_, ast) = brackets_parser("(a)").unwrap();
        assert_eq!(ast, Pattern::Literal('a'));
    }

    #[test]
    fn repeat_parser_test() {
        let (_, ast) = repeat_parser("a*").unwrap();
        assert_eq!(ast, Pattern::Repeat(Box::new(Pattern::Literal('a'))));
    }

    #[test]
    fn concatnate_parser_test() {
        let (_, ast) = concatnate_parser("a*b").unwrap();
        let expect = Pattern::Concatnate {
            first: Box::new(Pattern::Repeat(Box::new(Pattern::Literal('a')))),
            second: Box::new(Pattern::Literal('b')),
        };
        assert_eq!(ast, expect);
    }

    #[test]
    fn choose_parser_test() {
        let (_, ast) = choose_parser("(a|)").unwrap();
        let expect = Pattern::Choose {
            first: Box::new(Pattern::Literal('a')),
            second: Box::new(Pattern::Empty),
        };
        assert_eq!(ast, expect);
    }
}
