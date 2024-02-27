use chrono::{TimeZone, Utc};

use crate::processor::process_one_line;
use crate::rules::{parse_rule, RuleContext};

#[test]
fn test_single_rule() {
    let rule_str = r"
type=SingleWithThreshold
ptype=RegExp
pattern=sshd\[\d+\]: Failed .+ for (\S+) from ([\d.]+) port \d+ ssh2
desc=user $1 ip $2
action=test Too many SSH login failures for user $1 from $2 \
mail root@localhost
thresh=3
window=300";

    let rule = parse_rule(rule_str).unwrap();
    let rule_context = RuleContext::new(rule, "", 0);
    let rules = vec![rule_context];

    let base_date = Utc.ymd(2024, 10, 17);
    let data = vec![
        ("Oct 17 12:00:01 host2 sshd[2181]: Failed password for bob from 10.1.1.1 port 55529 ssh2", base_date.and_hms_opt(12, 0, 1).unwrap()),
        ("Oct 17 12:01:09 host2 sshd[2183]: Failed password for jim from 10.6.1.9 port 55530 ssh2", base_date.and_hms_opt(12, 1, 9).unwrap()),
        ("Oct 17 12:02:16 host2 sshd[2181]: Failed password for bob from 10.1.1.1 port 55534 ssh2", base_date.and_hms_opt(12, 2, 16).unwrap()),
        ("Oct 17 12:03:43 host2 sshd[2187]: Failed password for jim from 10.1.1.1 port 55538 ssh2", base_date.and_hms_opt(12, 3, 43).unwrap()),
        ("Oct 17 12:04:56 host2 sshd[2189]: Failed password for bob from 10.1.1.1 port 55543 ssh2", base_date.and_hms_opt(12, 4, 56).unwrap()),
    ];

    for (line, timestamp) in data {
        process_one_line(&rules, line, timestamp);
    }
}