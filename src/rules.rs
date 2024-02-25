use std::collections::HashMap;

use anyhow::{anyhow, Result};
use regex::Regex;
use thiserror::Error;

mod single;
mod jump;
mod calendar;
mod suppress;
mod pair;

#[derive(Error, Debug)]
pub enum RuleError {
    #[error("Invalid field (key={key:?}, value={value:?})")]
    InvalidField { key: String, value: String },
}

pub trait CheckRule {
    fn check_rule(&self, line: &str) -> bool;
}

#[derive(Debug, PartialEq)]
pub enum ContinueType {
    TakeNext,
    DontCont,
    EndMatch,
    GoTo(String),
}

#[derive(Debug, PartialEq)]
pub enum PatternType {
    Regex,
    Substr,
    NotRegex,
    NotSubstr,
}

// #[derive(Debug, PartialEq)]
pub enum Pattern {
    Regex(Regex),
    Substr(String),
}

#[derive(Debug, PartialEq)]
pub struct ActionWrite {}

#[derive(Debug, PartialEq)]
pub struct ActionShellCmd {}

#[derive(Debug, PartialEq)]
pub struct ActionPipe {}

#[derive(Debug, PartialEq)]
pub struct ActionShellExec {}

#[derive(Debug, PartialEq)]
pub struct ActionPipeExec {}

#[derive(Debug, PartialEq)]
pub struct ActionLogOnly {}

#[derive(Debug, PartialEq)]
pub enum Action {
    Write(ActionWrite),
    ShellCmd(ActionShellCmd),
    Pipe(ActionPipe),
    ShellExec(ActionShellExec),
    PipeExec(ActionPipeExec),
    LogOnly(ActionLogOnly),
    None,
}

pub enum Rule {
    Single(single::Single),
    // Pair,
    // Suppress,
    // Calendar,
    // Jump,
}

impl CheckRule for Rule {
    fn check_rule(&self, line: &str) -> bool {
        match self {
            Rule::Single(rule) => rule.check_rule(line)
        }
    }
}

// FIXME This simplistically splits actions by ; even though they
// might be contained inside actions.  Will deal with later.
fn parse_action(actions_str: &str) -> Vec<Action> {
    let mut actions = vec![];
    for action_str in actions_str.split(";") {
        let initial_str = action_str.split(" ").next().unwrap_or("").trim();
        let action = match initial_str.to_lowercase().as_str() {
            "write" => Action::Write(ActionWrite {}),
            "shellcmd" => Action::ShellCmd(ActionShellCmd {}),
            "pipe" => Action::Pipe(ActionPipe {}),
            "shellexec" => Action::ShellExec(ActionShellExec {}),
            "pipeexec" => Action::PipeExec(ActionPipeExec {}),
            "logonly" => Action::LogOnly(ActionLogOnly {}),
            _ => {
                Action::None
            }
        };
        actions.push(action)
    }
    return actions;
}

pub fn parse_rule(s: &str) -> Result<Rule> {
    let mut config = HashMap::new();

    let mut current_line: String = "".to_string();
    for line in s.split("\n") {
        if line.starts_with("#") { continue; }

        current_line.push_str(line.trim());

        if line.trim().ends_with("\\") {
            continue;
        }

        if let Some((key, value)) = current_line.clone().split_once('=') {
            config.insert(key.to_string(), value.to_string());
        }

        current_line.clear();
    }

    return match config["type"].as_str() {
        "single" => single::create_single(config),
        val => {
            return Err(anyhow!(RuleError::InvalidField {
                key: "type".to_string(),
                value: val.to_string(),
            }));
        }
    };
}
