use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::Error;

pub trait Matchable {
    fn matches(&self, input: &str, path: Option<&str>) -> Result<bool, Error>;
}

/// A matcher can process an input string and will
/// return a matched result
/// It can match one or many regulal expressions and
/// support inverting (noting) the expression
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Matcher<T>
where
    T: Matchable + Default,
{
    #[serde(deserialize_with = "T::deserialize")]
    #[serde(serialize_with = "T::serialize")]
    kind: T,
    #[serde(default)]
    or: Vec<Matcher<T>>,

    #[serde(default)]
    and: Vec<Matcher<T>>,

    #[serde(default)]
    not: bool,
}

impl<T> Matcher<T>
where
    T: Matchable + Default,
{
    pub fn new(kind: T, or: Vec<Matcher<T>>, and: Vec<Matcher<T>>, not: bool) -> Self {
        Self { kind, or, and, not }
    }
}

impl<T> Matchable for Matcher<T>
where
    T: Matchable + Default,
{
    fn matches(&self, input: &str, path: Option<&str>) -> Result<bool, Error> {
        let mut result = self.kind.matches(input, path)? ^ self.not;

        // or is lazy and will abort once result is true
        for or in self.or.iter() {
            if result {
                break;
            } else {
                result |= or.matches(input, path)?;
            }
        }

        // and is lazy and will abort when result is false
        for and in self.and.iter() {
            if !result {
                break;
            } else {
                result &= and.matches(input, path)?;
            }
        }

        Ok(result)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum MatcherKind {
    Re(ReMatcher),
    AlwaysTrue,
    AlwaysFalse,
}

impl Default for MatcherKind {
    fn default() -> Self {
        Self::AlwaysFalse
    }
}

impl Matchable for MatcherKind {
    fn matches(&self, input: &str, path: Option<&str>) -> Result<bool, Error> {
        match self {
            Self::Re(re) => re.matches(input, path),
            Self::AlwaysTrue => Ok(true),
            Self::AlwaysFalse => Ok(false),
        }
    }
}

/// Regular expression matcher
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ReMatcher {
    expr: String,
}

impl ReMatcher {
    pub fn new(expr: &str) -> Self {
        Self { expr: expr.into() }
    }
}

impl Matchable for ReMatcher {
    fn matches(&self, input: &str, _path: Option<&str>) -> Result<bool, Error> {
        let re = Regex::new(&self.expr)?;

        Ok(re.is_match(input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_match_re() {
        let matcher = ReMatcher::new("test");

        assert!(matcher.matches("test: Message", None).unwrap());
    }

    #[test]
    fn it_should_not_match_re() {
        let matcher = ReMatcher::new("test");

        assert!(!matcher.matches("warning: Message", None).unwrap());
    }

    #[test]
    fn it_should_invert_matcher() {
        let matcher = Matcher::new(
            MatcherKind::Re(ReMatcher::new("test")),
            vec![],
            vec![],
            true,
        );
        assert!(!matcher.matches("test: Message", None).unwrap());
        assert!(matcher.matches("warning: Message", None).unwrap());
    }

    #[test]
    fn it_should_process_or() {
        let matcher = Matcher::new(
            MatcherKind::Re(ReMatcher::new("test")),
            vec![Matcher::new(
                MatcherKind::Re(ReMatcher::new("warning")),
                vec![],
                vec![],
                false,
            )],
            vec![],
            false,
        );
        assert!(matcher.matches("test: Message", None).unwrap());
        assert!(matcher.matches("warning: Message", None).unwrap());
        assert!(!matcher.matches("error: Message", None).unwrap());
    }

    #[test]
    fn it_should_process_and() {
        let matcher = Matcher::new(
            MatcherKind::Re(ReMatcher::new("test")),
            vec![],
            vec![
                Matcher::new(
                    MatcherKind::Re(ReMatcher::new("warning")),
                    vec![],
                    vec![],
                    false,
                ),
                Matcher::new(
                    MatcherKind::Re(ReMatcher::new("and")),
                    vec![],
                    vec![],
                    false,
                ),
            ],
            false,
        );
        assert!(!matcher.matches("test: Message", None).unwrap());
        assert!(!matcher.matches("warning: Message", None).unwrap());
        assert!(!matcher.matches("error: Message", None).unwrap());
        assert!(!matcher.matches("test warning: Message", None).unwrap());
        assert!(matcher.matches("test and warning: Message", None).unwrap());
    }

    #[test]
    fn it_should_always_be_true() {
        let matcher = Matcher::new(MatcherKind::AlwaysTrue, vec![], vec![], false);

        assert!(matcher.matches("test: Message", None).unwrap());
    }

    #[test]
    fn it_should_always_be_false() {
        let matcher = Matcher::new(MatcherKind::AlwaysFalse, vec![], vec![], false);

        assert!(!matcher.matches("test: Message", None).unwrap());
    }
}
