use std::collections::HashMap;

use anyhow::anyhow;
use regex::Regex;

use crate::rules::{Action, CheckRule, ContinueType, parse_action, Pattern, PatternType, Rule, RuleError};

#[cfg(test)]
mod tests;

pub struct Single {
    continue_: ContinueType,
    pattern_type: PatternType,
    pattern: Pattern,
    // varmap
    // context
    description: String,
    action: Vec<Action>,
    window: u32,
    threshold: u32,
}

impl CheckRule for Single {
    fn check_rule(&self, line: &str) -> bool {
        match &self.pattern {
            Pattern::Regex(re) => { return re.find(line).is_some(); }
            Pattern::Substr(substr) => { line.contains(substr) }
        }
    }
}


pub fn create_single(config: HashMap<String, String>) -> anyhow::Result<Rule> {
    // println!("!! {:#?}", config);
    // println!("parse {:?}", "0".parse::<u32>());

    let pattern_type = match config.get("ptype").unwrap().to_lowercase().as_str() {
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
    };
    let pattern: Pattern;
    pattern = match pattern_type {
        PatternType::Regex | PatternType::NotRegex => Pattern::Regex(Regex::new(config.get("pattern").unwrap()).unwrap()),
        PatternType::Substr | PatternType::NotSubstr => Pattern::Substr(config.get("pattern").unwrap().to_string()),
    };

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
        pattern_type: pattern_type,
        pattern: pattern,
        description: config.get("desc").unwrap().to_string(),
        action: parse_action(config.get("action").unwrap()),
        window: config.get("window").unwrap_or(&"0".to_string()).parse()?,
        threshold: config.get("thresh").unwrap_or(&"0".to_string()).parse()?,
    }))
}
