use crate::processor::process_one_line;
use crate::rules::{parse_rule};

#[test]
fn test_single_rule() {
    let rule_str = "\
type=single
ptype=regexp
pattern=(\\S+) sshd\\[\\d+\\]: Accepted.*for (\\S+) from (\\S+) port (\\d+)\\s
desc=ssh login to $1 from $3 for user $2
action=write - $2 logged in to $1 from $3 port $4
window=0
thresh=0";

    let rules = vec![parse_rule(rule_str).unwrap()];

    process_one_line(&rules, "");
}