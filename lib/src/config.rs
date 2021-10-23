use crate::{Condition, Error};
use serde::{Deserialize, Serialize};
use trickle::{Node, Receiver};

// TODO use this for output handling in the future!
pub struct Event<T>
where
    T: Receiver<String, Error>,
{
    node: Node<String, T, Error>,
    config: Config,
}

impl<T> Event<T>
where
    T: Receiver<String, Error>,
{
    pub fn new(config: Config, node: Node<String, T, Error>) -> Self {
        Self { node, config }
    }

    pub fn check(&mut self, input: &str) -> Result<bool, Error> {
        self.node.handle_mut(&self.config.check(input)?)
    }
}

/// Covers all conditions needed for execution
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Config {
    conditions: Vec<Condition>,
}

impl Config {
    pub fn new(conditions: Vec<Condition>) -> Self {
        Self { conditions }
    }

    pub fn check(&self, input: &str) -> Result<String, Error> {
        let mut output = "".to_string();
        for cond in &self.conditions {
            let res = cond.check(input)?;
            if !res.is_empty() {
                output.push_str(&res);
                output.push('\n');
            }
        }
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use crate::BasicMatchResult;
    use crate::MatchResult;
    use crate::Matcher;
    use crate::MatcherKind;
    use crate::ReMatcher;

    use super::*;

    #[test]
    fn it_should_run_all_conditions() {
        let config = Config::new(vec![
            Condition::new(
                Matcher::new(
                    MatcherKind::Re(ReMatcher::new("test")),
                    vec![],
                    vec![],
                    false,
                ),
                MatchResult::Basic(BasicMatchResult::new("Then Result 1!")),
                false,
                None,
            ),
            Condition::new(
                Matcher::new(
                    MatcherKind::Re(ReMatcher::new("test")),
                    vec![],
                    vec![],
                    false,
                ),
                MatchResult::Basic(BasicMatchResult::new("Then Result 2!")),
                false,
                None,
            ),
            Condition::new(
                Matcher::new(
                    MatcherKind::Re(ReMatcher::new("warning")),
                    vec![],
                    vec![],
                    false,
                ),
                MatchResult::Basic(BasicMatchResult::new("Then Result 3!")),
                false,
                None,
            ),
        ]);

        assert_eq!(
            &config.check("test: Message").unwrap(),
            "Then Result 1!\nThen Result 2!\n"
        );
    }
}
