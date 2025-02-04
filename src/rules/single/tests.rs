use crate::rules::{Action, ActionWrite, ContinueType, Pattern};
use crate::rules::{parse_rule, PatternType};
use crate::rules::Rule;

#[test]
fn test_parse_rule_single_ok() {
    let rule_str = "\
type=single
ptype=regexp
pattern=(\\S+) sshd\\[\\d+\\]: Accepted.*for (\\S+) from (\\S+) port (\\d+)\\s
desc=ssh login to $1 from $3 for user $2
action=write - $2 logged in to $1 from $3 port $4
window=0
thresh=0";

    let r = parse_rule(rule_str).unwrap();

    let Rule::Single(foo) = r;
    assert_eq!(foo.continue_, ContinueType::TakeNext);
    assert_eq!(foo.pattern_type, PatternType::Regex);
    if let Pattern::Regex(pattern_re) = foo.pattern {
        assert_eq!(
            pattern_re.as_str(),
            "(\\S+) sshd\\[\\d+\\]: Accepted.*for (\\S+) from (\\S+) port (\\d+)\\s"
        );
    }
    assert_eq!(foo.description, "ssh login to $1 from $3 for user $2");
    assert_eq!(foo.window, 0);
    assert_eq!(foo.threshold, 0);
    assert_eq!(foo.action.len(), 1);
    assert_eq!(foo.action[0], Action::Write(ActionWrite {}));
}

#[test]
fn test_parse_rule_single_with_cont_ok() {
    let rule_str = "\
# some comment
type=single
ptype=regexp
pattern=(\\S+) sshd\\[\\d+\\]: Accepted.*for (\\S+) from (\\S+) port (\\d+)\\s
desc=ssh login to $1 from $3 for \
    user $2
action=write - $2 logged in to $1 from $3 port $4
window=0
thresh=0";

    let r = parse_rule(rule_str).unwrap();

    let Rule::Single(foo) = r;
    assert_eq!(foo.continue_, ContinueType::TakeNext);
    assert_eq!(foo.pattern_type, PatternType::Regex);
    if let Pattern::Regex(pattern_re) = foo.pattern {
        assert_eq!(
            pattern_re.as_str(),
            "(\\S+) sshd\\[\\d+\\]: Accepted.*for (\\S+) from (\\S+) port (\\d+)\\s"
        );
    }
    assert_eq!(foo.description, "ssh login to $1 from $3 for user $2");
    assert_eq!(foo.window, 0);
    assert_eq!(foo.threshold, 0);
    assert_eq!(foo.action.len(), 1);
    assert_eq!(foo.action[0], Action::Write(ActionWrite {}));
}
