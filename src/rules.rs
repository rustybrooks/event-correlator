#[cfg(test)]
mod tests;

use anyhow::{anyhow, Result};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, PartialEq)]
enum ContinueType {
    TakeNext,
    DontCont,
    EndMatch,
    GoTo(String),
}

#[derive(Debug, PartialEq)]
enum PatternType {
    Regex,
    Substr,
    NotRegex,
    NotSubstr,
}

#[derive(Debug, PartialEq)]
struct Action {
    foo: u32,
}

struct Single {
    continue_: ContinueType,
    pattern_type: PatternType,
    pattern: String,
    // varmap
    // context
    description: String,
    action: Vec<Action>,
    window: u32,
    threshold: u32,
}

/*
struct Pair {
    continue_: ContinueType,
    pattern_type: PatternType,
    pattern: String,
    // varmap
    // context
    description: String,
    action: Vec<Action>,
    continue2: ContinueType,
    pyte2: PatternType,
    pattern2: String,
    // varmap2
    // context2
    desc2: String,
    action2: Vec<Action>,

    window: u32,
    suppress: u32,
}

struct Suppress {
    pattern_type: PatternType,
    pattern: String,
    // varmap
    // context
    description: String,
}

struct Calendar {
    time: String,
    // context
    description: String,
    action: Vec<Action>,
}

struct Jump {
    continue_: ContinueType,
    pattern_type: PatternType,
    pattern: String,
    // varmap
    // context
    cfset: Vec<Rule>,
    description: String,
    action: Vec<Action>,
    jump: String,
}

 */

enum Rule {
    Single(Single),
    // Pair,
    // Suppress,
    // Calendar,
    // Jump,
}

#[derive(Error, Debug)]
pub enum RuleError {
    #[error("Invalid field (key={key:?}, value={value:?})")]
    InvalidField { key: String, value: String },
}

fn create_single(config: HashMap<String, String>) -> Result<Rule> {
    Ok(Rule::Single(Single {
        continue_: match config
            .get("continue")
            .unwrap_or(&"takenext".to_string())
            .to_lowercase()
            .as_str()
        {
            "takenext" => ContinueType::TakeNext,
            "dontcont" => ContinueType::DontCont,
            "endmatch" => ContinueType::EndMatch,
            "goto" => ContinueType::GoTo("what".to_string()),
            val => {
                return Err(anyhow!(RuleError::InvalidField {
                    key: "continue_".to_string(),
                    value: val.to_string(),
                }));
            }
        },
        pattern_type: match config.get("ptype").unwrap().to_lowercase().as_str() {
            "regexp" => PatternType::Regex,
            "substr" => PatternType::Substr,
            "nregexp" => PatternType::NotRegex,
            "nsubstr" => PatternType::NotSubstr,
            val => {
                return Err(anyhow!(RuleError::InvalidField {
                    key: "pattern_type".to_string(),
                    value: val.to_string(),
                }));
            }
        },
        pattern: config.get("pattern").unwrap().to_string(),
        description: config.get("desc").unwrap().to_string(),
        action: vec![],
        window: config.get("window").unwrap_or(&"0".to_string()).parse()?,
        threshold: config.get("thresh").unwrap_or(&"0".to_string()).parse()?,
    }))
}

fn parse_rule(s: &str) -> Result<Rule> {
    let mut config = HashMap::new();

    let mut current_line: String = "".to_string();
    for line in s.split("\n") {
        if line.starts_with("#") { continue; }

        current_line.push_str(line.trim());

        if line.trim().ends_with("\\") {
            continue;
        }

        let cl_clone = current_line.clone();
        if let Some((key, value)) = cl_clone.split_once('=') {
            config.insert(key.to_string(), value.to_string());
        }

        current_line.clear();
    }

    return match config["type"].as_str() {
        "single" => create_single(config),
        val => {
            return Err(anyhow!(RuleError::InvalidField {
                key: "type".to_string(),
                value: val.to_string(),
            }));
        }
    };
}
