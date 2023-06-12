use header::PgnGameHeader;
use movetext::Move;
use nom::{
    multi::many1,
    IResult,
};

pub mod header;
pub mod movetext;

#[derive(Debug, Clone)]
pub struct PgnGame<'s> {
    pub header: PgnGameHeader<'s>,
    pub moves: Vec<Move<'s>>,
}

impl<'s> PgnGame<'s> {
    pub fn parse_game(s: &'s str) -> IResult<&str, PgnGame> {
        let (s, header) = PgnGameHeader::parse_header(s)?;
        let (s, moves) = Move::parse_movetext(s)?;

        Ok((s, Self { header, moves }))
    }

    pub fn parse_games(s: &'s str) -> IResult<&str, Vec<PgnGame>> {
        many1(Self::parse_game)(s)
    }
}

#[cfg(test)]
mod tests {
    use crate::PgnGame;

    #[test]
    fn single_game() {
        PgnGame::parse_game(include_str!("../single_game.pgn")).unwrap();
    }

    #[test]
    fn multiple_games() {
        let (_, games) = PgnGame::parse_games(include_str!("../16_games.pgn")).unwrap();
        dbg!(games[0].moves.len());

        assert_eq!(games.len(), 16);
    }
}
