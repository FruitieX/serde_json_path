use nom::character::complete::char;
use nom::{branch::alt, combinator::map, multi::many0, sequence::preceded, IResult};
use serde_json::Value;

use self::segment::{parse_segment, PathSegment};

pub(crate) mod primitive;
pub mod segment;
pub mod selector;

pub(crate) trait QueryValue {
    fn query_value<'b>(&self, current: &'b Value, root: &'b Value) -> Vec<&'b Value>;
}

#[derive(Debug, PartialEq)]
pub struct Path {
    kind: PathKind,
    pub segments: Vec<PathSegment>,
}

#[derive(Debug, PartialEq)]
pub enum PathKind {
    Root,
    Current,
}

impl Path {
    pub fn is_root(&self) -> bool {
        matches!(self.kind, PathKind::Root)
    }

    pub fn is_current(&self) -> bool {
        matches!(self.kind, PathKind::Current)
    }
}

impl QueryValue for Path {
    fn query_value<'b>(&self, current: &'b Value, root: &'b Value) -> Vec<&'b Value> {
        let mut query = match self.kind {
            PathKind::Root => vec![root],
            PathKind::Current => vec![current],
        };
        for segment in &self.segments {
            let mut new_query = Vec::new();
            for q in &query {
                new_query.append(&mut segment.query_value(q, root));
            }
            query = new_query;
        }
        query
    }
}

fn parse_path_segments(input: &str) -> IResult<&str, Vec<PathSegment>> {
    many0(parse_segment)(input)
}

pub fn parse_path(input: &str) -> IResult<&str, Path> {
    alt((
        map(preceded(char('$'), parse_path_segments), |segments| Path {
            kind: PathKind::Root,
            segments,
        }),
        map(preceded(char('@'), parse_path_segments), |segments| Path {
            kind: PathKind::Current,
            segments,
        }),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::parse_path;

    #[test]
    fn root_path() {
        {
            let (_, p) = parse_path("$").unwrap();
            assert!(p.is_root());
        }
        {
            let (_, p) = parse_path("$.name").unwrap();
            assert_eq!(p.segments[0].segment.as_dot_name().unwrap(), "name");
        }
        {
            let (_, p) = parse_path("$.names['first_name']..*").unwrap();
            assert_eq!(p.segments[0].segment.as_dot_name().unwrap(), "names");
            let clh = p.segments[1].segment.as_long_hand().unwrap();
            assert_eq!(clh[0].as_name().unwrap(), "first_name");
            assert!(p.segments[2].segment.is_wildcard());
        }
    }

    #[test]
    fn current_path() {
        {
            let (_, p) = parse_path("@").unwrap();
            assert!(p.is_current());
        }
    }
}
