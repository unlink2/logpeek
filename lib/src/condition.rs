use crate::Error;
use crate::MatchResponse;
use crate::Matchable;
use crate::Matcher;

use serde::{Deserialize, Serialize};

pub trait Checkable {
    fn check(&self, input: &str, path: Option<&str>) -> Result<String, Error>;
}

/// Condition for matching
/// If the match returns true it will output the 'then'
/// result
/// otherwise it will process an else condition if it exists
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Condition<TMatcher, TResponse>
where
    TMatcher: Matchable + Default,
    TResponse: MatchResponse + Default,
{
    if_match: Matcher<TMatcher>,
    then: TResponse,
    output_input: bool,
    else_then: Option<Box<Self>>,
}

impl<TMatcher, TResponse> Condition<TMatcher, TResponse>
where
    TMatcher: Matchable + Default,
    TResponse: MatchResponse + Default,
{
    // new that avoids exposing box to the outside
    pub fn new(
        if_match: Matcher<TMatcher>,
        then: TResponse,
        output_input: bool,
        else_then: Option<Self>,
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

    fn exec(&self, input: &str, path: Option<&str>, output: &mut String) -> Result<(), Error> {
        if self.if_match.matches(input, path)? {
            if self.output_input {
                output.push_str(&self.then.to_string().replace("{}", input));
            } else {
                output.push_str(&self.then.to_string());
            }
        } else if let Some(else_then) = &self.else_then {
            else_then.exec(input, path, output)?;
        }

        Ok(())
    }
}

impl<TMatcher, TResponse> Checkable for Condition<TMatcher, TResponse>
where
    TMatcher: Matchable + Default,
    TResponse: MatchResponse + Default,
{
    fn check(&self, input: &str, path: Option<&str>) -> Result<String, Error> {
        let mut output = "".to_string();

        self.exec(input, path, &mut output)?;

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use crate::BasicMatchResult;
    use crate::MatchResult;
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

        assert_eq!(&cond.check("test: Message", None).unwrap(), "Then Result!");
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
            &cond.check("test: Message", None).unwrap(),
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

        assert_eq!(&cond.check("test: Message", None).unwrap(), "");
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

        assert_eq!(&cond.check("test: Message", None).unwrap(), "Else Result!");
    }
}
