use crate::rules::Rule;

mod processor;
mod rules;

pub fn main() {
    let rules: Vec<Rule> = Vec::new();
    let _ = processor::process(rules);
}
