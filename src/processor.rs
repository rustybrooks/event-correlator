mod tests;

use std::io;
use crate::rules::Rule;
use crate::rules::CheckRule;

pub fn process_one_line(rules: &Vec<Rule>, line: &str) {
    println!("{}", line);
    for rule in rules {
        rule.check_rule(line);
    }
}

pub fn process(rules: Vec<Rule>) -> io::Result<()> {
    let mut buffer = String::new();
    loop {
        io::stdin().read_line(&mut buffer)?;
        process_one_line(&rules, &buffer);
    }
}
