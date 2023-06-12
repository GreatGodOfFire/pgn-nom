use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::char,
    combinator::{map_parser, value},
    multi::many0,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug, Clone)]
pub struct PgnGameHeader<'s> {
    pub event: &'s str,
    pub site: &'s str,
    // TODO: Would parsing the date using chrono be better?
    pub date: &'s str,
    // TODO: Parse this
    pub round: &'s str,
    pub white: &'s str,
    pub black: &'s str,
    pub result: PgnGameResult,

    pub annotator: Option<&'s str>,
    pub ply_count: Option<&'s str>,
    pub time_control: Option<&'s str>,
    pub time: Option<&'s str>,
    pub termination: Option<&'s str>,
    pub mode: Option<&'s str>,
    pub fen: Option<&'s str>,

    pub others: Vec<(&'s str, &'s str)>,
}

impl<'s> PgnGameHeader<'s> {
    pub(crate) fn parse_header(s: &'s str) -> IResult<&str, PgnGameHeader> {
        let (s, (event, site, date, round, white, black, result)) = tuple((
            named_pgn_tag("Event"),
            named_pgn_tag("Site"),
            named_pgn_tag("Date"),
            named_pgn_tag("Round"),
            named_pgn_tag("White"),
            named_pgn_tag("Black"),
            map_parser(named_pgn_tag("Result"), parse_result),
        ))(s)?;

        let (s, all_others) = many0(pgn_tag)(s)?;
        let mut others = vec![];

        let mut annotator = None;
        let mut ply_count = None;
        let mut time_control = None;
        let mut time = None;
        let mut termination = None;
        let mut mode = None;
        let mut fen = None;

        for (tag, value) in all_others {
            match tag {
                "Annotator" => annotator = Some(value),
                "PlyCount" => ply_count = Some(value),
                "TimeControl" => time_control = Some(value),
                "Time" => time = Some(value),
                "Termination" => termination = Some(value),
                "Mode" => mode = Some(value),
                "FEN" => fen = Some(value),
                _ => others.push((tag, value)),
            }
        }

        Ok((
            s,
            Self {
                event,
                site,
                date,
                round,
                white,
                black,
                result,

                annotator,
                ply_count,
                time_control,
                time,
                termination,
                mode,
                fen,

                others,
            },
        ))
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PgnGameResult {
    WhiteWin,
    BlackWin,
    Draw,
    Other,
}

fn named_pgn_tag(name: &str) -> impl Fn(&str) -> IResult<&str, &str> + '_ {
    move |s| {
        terminated(
            delimited(
                char('['),
                preceded(
                    tuple((tag(name), char(' '))),
                    delimited(char('"'), is_not("\""), char('"')),
                ),
                char(']'),
            ),
            char('\n'),
        )(s)
    }
}

fn pgn_tag(s: &str) -> IResult<&str, (&str, &str)> {
    terminated(
        delimited(
            char('['),
            separated_pair(
                is_not(" "),
                char(' '),
                delimited(char('"'), is_not("\""), char('"')),
            ),
            char(']'),
        ),
        char('\n'),
    )(s)
}

fn parse_result(s: &str) -> IResult<&str, PgnGameResult> {
    alt((
        value(PgnGameResult::WhiteWin, tag("1-0")),
        value(PgnGameResult::BlackWin, tag("0-1")),
        value(PgnGameResult::Draw, tag("1/2-1/2")),
        value(PgnGameResult::Other, tag("*")),
    ))(s)
}
