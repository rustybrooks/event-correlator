use crate::rules::RuleContext;

mod processor;
mod rules;

pub fn main() {
    let rules: Vec<RuleContext> = Vec::new();
    let _ = processor::process(rules);
}
