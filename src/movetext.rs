use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{char, digit1, multispace0, newline, space1, multispace1},
    combinator::{opt, value, peek},
    multi::{many0, many_till},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Move<'s> {
    pub san: &'s str,
    pub comment: Option<&'s str>,
}

impl<'s> Move<'s> {
    pub fn parse_movetext(s: &'s str) -> IResult<&str, Vec<Self>> {
        let (s, (res, _)) = many_till(Self::parse, Self::movetext_end)(s)?;

        Ok((s, res))
    }

    fn parse(s: &'s str) -> IResult<&str, Self> {
        let (s, (san, _, comment)) = preceded(
            tuple((multispace0, opt(Self::move_num), multispace0)),
            tuple((is_not(" \n"), multispace1, opt(Self::comment))),
        )(s)?;

        Ok((s, Self { san, comment }))
    }

    fn movetext_end(s: &str) -> IResult<&str, &str> {
        alt((value("", peek(tuple((multispace0, char('['))))), delimited(multispace0, alt((tag("1-0"), tag("0-1"), tag("1/2-1/2"), tag("*"))), multispace0)))(s)
    }

    fn move_num(s: &str) -> IResult<&str, ()> {
        terminated(terminated(digit1, many0(char('.'))), multispace0)(s).map(|x| (x.0, ()))
    }

    fn comment(s: &str) -> IResult<&str, &str> {
        alt((
            delimited(char('{'), is_not("}"), char('}')),
            delimited(char(';'), is_not("\n"), newline),
        ))(s)
    }
}
