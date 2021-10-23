use std::io::{BufRead, BufReader};

use crate::{
    BasicMatchResult, Condition, Config, Error, MatchResult, Matcher, MatcherKind, ReMatcher,
};
use clap::Parser;

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
        } else if let Some(json) = &self.opts.json {
            self.config = serde_json::from_str(json)?;
        } else {
            self.config = Config::new(vec![Condition::new(
                Matcher::new(
                    MatcherKind::Re(ReMatcher::new(
                        self.opts.regex.as_ref().unwrap_or(&"".into()),
                    )),
                    vec![],
                    vec![],
                    self.opts.not,
                ),
                MatchResult::Basic(BasicMatchResult::new(
                    self.opts.output.as_ref().unwrap_or(&"".to_string()),
                )),
                self.opts.print_input,
                None,
            )]);
        }

        Ok(self)
    }

    fn check_stdin(&self, writer: &mut dyn std::io::Write) -> Result<(), Error> {
        let stdin = std::io::stdin();
        let reader = BufReader::new(stdin);

        for line in reader.lines() {
            if writer.write(self.config.check(&line?)?.as_bytes())? > 0 {
                writer.write_all(&[b'\n'])?;
            }
        }

        Ok(())
    }

    fn check_file(&self, input_file: &str, writer: &mut dyn std::io::Write) -> Result<(), Error> {
        let reader = BufReader::new(std::fs::File::open(input_file)?);

        for line in reader.lines() {
            if writer.write(self.config.check(&line?)?.as_bytes())? > 0 {
                writer.write_all(&[b'\n'])?;
            }
        }

        Ok(())
    }

    pub fn check(&self, writer: &mut dyn std::io::Write) -> Result<(), Error> {
        if let Some(input_file) = &self.opts.input_file {
            self.check_file(input_file, writer)
        } else {
            self.check_stdin(writer)
        }
    }

    pub fn print_json(&self, writer: &mut dyn std::io::Write) -> Result<(), Error> {
        if self.opts.print_json
            && writer.write(serde_json::to_string(&self.config)?.as_bytes())? > 0
        {
            writer.write_all(&[b'\n'])?;
        }
        Ok(())
    }
}

#[derive(Default, Parser)]
#[clap(version = "0.1.0", author = "Lukas Krickl <lukas@krickl.dev>")]
pub struct Opts {
    input_file: Option<String>,

    #[clap(short, long)]
    config_file: Option<String>,

    #[clap(short, long)]
    json: Option<String>,

    #[clap(short, long)]
    not: bool,

    #[clap(short, long)]
    print_input: bool,

    #[clap(long)]
    print_json: bool,

    #[clap(short, long)]
    output: Option<String>,

    #[clap(short, long)]
    regex: Option<String>,
}
