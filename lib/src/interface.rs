use std::io::{BufRead, BufReader};

use crate::{
    BasicMatchResult, Condition, Config, Error, MatchResult, Matcher, MatcherKind, ReMatcher,
};
use clap::{AppSettings, Clap};

/// Interface between input parser and
/// config options
pub struct Interface {
    opts: Opts,
    config: Config,
}

impl Interface {
    pub fn from_args() -> Result<Self, Error> {
        Self {
            opts: Opts::parse(),
            config: Config::default(),
        }
        .create_config()
    }

    /// Creates a config from opts
    pub fn create_config(mut self) -> Result<Self, Error> {
        // if a config file is specified
        // load the entire config from file and ignore the rest!
        if let Some(config_file) = &self.opts.config_file {
            self.config = serde_json::from_reader(std::fs::File::open(config_file)?)?;
        } else {
            self.config = Config::new(vec![Condition::new(
                Matcher::new(
                    MatcherKind::Re(ReMatcher::new(&self.opts.regex)),
                    vec![],
                    vec![],
                    self.opts.not,
                ),
                MatchResult::Basic(BasicMatchResult::new(&self.opts.output)),
                self.opts.print_input,
                None,
            )]);
        }

        Ok(self)
    }

    fn check_stdin(&self) -> Result<String, Error> {
        let stdin = std::io::stdin();
        let reader = BufReader::new(stdin);

        let mut result = String::new();
        for line in reader.lines() {
            result.push_str(&self.config.check(&line?)?);
        }

        Ok(result)
    }

    fn check_file(&self, input_file: &str) -> Result<String, Error> {
        let reader = BufReader::new(std::fs::File::open(input_file)?);

        let mut result = String::new();
        for line in reader.lines() {
            result.push_str(&self.config.check(&line?)?);
        }

        Ok(result)
    }

    pub fn check(&self) -> Result<String, Error> {
        if let Some(input_file) = &self.opts.input_file {
            self.check_file(input_file)
        } else {
            self.check_stdin()
        }
    }
}

#[derive(Default, Clap)]
#[clap(version = "0.1.0", author = "Lukas Krickl <lukas@krickl.dev>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    input_file: Option<String>,

    #[clap(short, long)]
    config_file: Option<String>,

    #[clap(short, long)]
    not: bool,

    #[clap(short, long)]
    print_input: bool,

    #[clap(short, long)]
    output: String,

    #[clap(short, long)]
    regex: String,
}