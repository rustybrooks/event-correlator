#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::error::Error;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
enum ContinueType {
    TakeNext,
    DontCont,
    EndMatch,
    GoTo(String),
}

enum PatternType {
    Regex,
    Substr,
    NotRegex,
    NotSubstr,
}

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

// #[derive(Debug, Clone)]
// struct CreateRuleError;

#[derive(Debug, Clone)]
pub enum ParseRuleError {
    Stuff,
    InvalidInteger(ParseIntError),
}

impl From<ParseIntError> for ParseRuleError {
    fn from(e: ParseIntError) -> Self {
        Self::InvalidInteger(e)
    }
}

fn create_single(config: HashMap<&str, &str>) -> Result<Rule, ParseRuleError> {
    Ok(Rule::Single(Single {
        continue_: match config
            .get("continue")
            .unwrap_or_else(|| &"Did not find continue_ options")
            .to_lowercase()
            .as_str()
        {
            "takenext" => ContinueType::TakeNext,
            "dontcont" => ContinueType::DontCont,
            "endmatch" => ContinueType::EndMatch,
            "goto" => ContinueType::GoTo("what".to_string()),
            _ => return Err(ParseRuleError::Stuff),
        },
        pattern_type: match config.get("ptype").unwrap().to_lowercase().as_str() {
            "regexp" => PatternType::Regex,
            "substr" => PatternType::Substr,
            "nregexp" => PatternType::NotRegex,
            "nsubstr" => PatternType::NotSubstr,
            _ => return Err(ParseRuleError::Stuff),
        },
        pattern: config.get("pattern").unwrap().to_string(),
        description: config.get("desc").unwrap().to_string(),
        action: vec![],
        window: config.get("pattern").unwrap().parse()?,
        threshold: config.get("threshold").unwrap().parse()?,
    }))
}

fn parse_rule(s: &str) -> Result<Rule, ParseRuleError> {
    /*

    */

    let mut config = HashMap::new();

    for line in s.split("\n") {
        if let Some((key, value)) = line.split_once('=') {
            config.insert(key, value);
        }
    }

    return match config["type"] {
        "single" => create_single(config),
        _ => Err(ParseRuleError::Stuff),
    };
}
