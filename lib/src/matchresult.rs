use serde::{Deserialize, Serialize};

pub trait MatchResponse: Clone + ToString {}

/// A matchresult is a simple output that will
/// called upon if a condition is met
#[derive(Clone, Serialize, Deserialize)]
pub enum MatchResult {
    Basic(BasicMatchResult),
}

impl MatchResponse for MatchResult {}

impl Default for MatchResult {
    fn default() -> Self {
        Self::Basic(BasicMatchResult::default())
    }
}

impl ToString for MatchResult {
    fn to_string(&self) -> String {
        match self {
            Self::Basic(res) => res.to_string(),
        }
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct BasicMatchResult {
    message: String,
}

impl BasicMatchResult {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl ToString for BasicMatchResult {
    fn to_string(&self) -> String {
        self.message.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_convert_results_to_string() {
        let res = MatchResult::Basic(BasicMatchResult::new("message!"));

        assert_eq!(&res.to_string(), "message!");
    }
}
