use crate::Error;
use crate::MatchResult;
use crate::Matchable;
use crate::Matcher;

use serde::{Deserialize, Serialize};

pub trait Checkable {
    fn check(&self, input: &str) -> Result<String, Error>;
}

/// Condition for matching
/// If the match returns true it will output the 'then'
/// result
/// otherwise it will process an else condition if it exists
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Condition<T>
where
    T: Matchable + Default,
{
    if_match: Matcher<T>,
    then: MatchResult,
    output_input: bool,
    else_then: Option<Box<Condition<T>>>,
}

impl<T> Condition<T>
where
    T: Matchable + Default,
{
    // new that avoids exposing box to the outside
    pub fn new(
        if_match: Matcher<T>,
        then: MatchResult,
        output_input: bool,
        else_then: Option<Condition<T>>,
    ) -> Self {
        match else_then {
            Some(else_then) => Self {
                if_match,
                then,
                output_input,
                else_then: Some(Box::new(else_then)),
            },

            None => Self {
                if_match,
                then,
                output_input,
                else_then: None,
            },
        }
    }

    fn exec(&self, input: &str, output: &mut String) -> Result<(), Error> {
        if self.if_match.matches(input)? {
            if self.output_input {
                output.push_str(&self.then.to_string().replace("{}", input));
            } else {
                output.push_str(&self.then.to_string());
            }
        } else if let Some(else_then) = &self.else_then {
            else_then.exec(input, output)?;
        }

        Ok(())
    }
}

impl<T> Checkable for Condition<T>
where
    T: Matchable + Default,
{
    fn check(&self, input: &str) -> Result<String, Error> {
        let mut output = "".to_string();

        self.exec(input, &mut output)?;

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use crate::BasicMatchResult;
    use crate::MatcherKind;
    use crate::ReMatcher;

    use super::*;

    #[test]
    fn it_should_process_if() {
        let cond = Condition::new(
            Matcher::new(
                MatcherKind::Re(ReMatcher::new("test")),
                vec![],
                vec![],
                false,
            ),
            MatchResult::Basic(BasicMatchResult::new("Then Result!")),
            false,
            None,
        );

        assert_eq!(&cond.check("test: Message").unwrap(), "Then Result!");
    }

    #[test]
    fn it_should_output_input() {
        let cond = Condition::new(
            Matcher::new(
                MatcherKind::Re(ReMatcher::new("test")),
                vec![],
                vec![],
                false,
            ),
            MatchResult::Basic(BasicMatchResult::new("Then {} Result!")),
            true,
            None,
        );

        assert_eq!(
            &cond.check("test: Message").unwrap(),
            "Then test: Message Result!"
        );
    }

    #[test]
    fn it_should_not_process_else_if_none() {
        let cond = Condition::new(
            Matcher::new(
                MatcherKind::Re(ReMatcher::new("warning")),
                vec![],
                vec![],
                false,
            ),
            MatchResult::Basic(BasicMatchResult::new("Then Result!")),
            false,
            None,
        );

        assert_eq!(&cond.check("test: Message").unwrap(), "");
    }

    #[test]
    fn it_should_process_else() {
        let cond = Condition::new(
            Matcher::new(
                MatcherKind::Re(ReMatcher::new("warning")),
                vec![],
                vec![],
                false,
            ),
            MatchResult::Basic(BasicMatchResult::new("Then Result!")),
            false,
            Some(Condition::new(
                Matcher::new(
                    MatcherKind::Re(ReMatcher::new("test")),
                    vec![],
                    vec![],
                    false,
                ),
                MatchResult::Basic(BasicMatchResult::new("Else Result!")),
                false,
                None,
            )),
        );

        assert_eq!(&cond.check("test: Message").unwrap(), "Else Result!");
    }
}
