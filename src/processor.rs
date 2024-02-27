use std::io;

use chrono::prelude::{DateTime, Utc};

use crate::rules::CheckRule;
use crate::rules::RuleContext;

mod tests;

pub fn process_one_line(rules: &Vec<RuleContext>, line: &str, timestamp: DateTime<Utc>) {
    println!("{}", line);
    for rule in rules {
        rule.check_rule(line, timestamp);
    }
}

pub fn process(rules: Vec<RuleContext>) -> io::Result<()> {
    let mut buffer = String::new();
    loop {
        io::stdin().read_line(&mut buffer)?;
        process_one_line(&rules, &buffer, Utc::now());
    }
}
