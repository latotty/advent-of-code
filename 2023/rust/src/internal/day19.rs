use rayon::iter::{ParallelBridge, ParallelIterator, IntoParallelRefIterator};
use std::{
    collections::{HashMap, VecDeque},
    ops::Range,
    sync::{Arc, atomic::AtomicU64},
};

pub fn process1(input: &str) -> u32 {
    let mut split = input.split("\n\n");
    let rules_map = Arc::new(
        split
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let mut split = line.split('{');
                let name = split.next().unwrap();
                split = split.next().unwrap().trim_end_matches('}').split(',');
                (name, split.map(Rule::from).collect())
            })
            .collect::<HashMap<&str, Vec<Rule>>>(),
    );

    split
        .next()
        .unwrap()
        .lines()
        .par_bridge()
        .map_with(
            rules_map,
            |rules_map: &mut Arc<HashMap<&str, Vec<Rule>>>, part: &str| -> u32 {
                let properties: &HashMap<PropertyName, u32> = &part[1..part.len() - 1]
                    .split(',')
                    .map(Property::from)
                    .map(|prop| (prop.name, prop.value))
                    .collect::<HashMap<PropertyName, u32>>();

                let mut target = RuleTarget::Next("in".to_string());
                while let RuleTarget::Next(ref next) = target {
                    if let Some(rules) = rules_map.get(next.as_str()) {
                        for rule in rules {
                            if let Some(new_target) = rule.check(properties) {
                                target = new_target.clone();
                                break;
                            }
                        }
                    } else {
                        panic!("invalid target {next}");
                    }
                }

                if target == RuleTarget::Allow {
                    properties.iter().map(|(_, value)| value).sum()
                } else {
                    0
                }
            },
        )
        .filter(|n| n > &0)
        .sum::<u32>()
}

pub fn process2(input: &str) -> u64 {
    let mut split = input.split("\n\n");
    let rules_map = Arc::new(
        split
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let mut split = line.split('{');
                let name = split.next().unwrap();
                split = split.next().unwrap().trim_end_matches('}').split(',');
                (name, split.map(Rule::from).collect())
            })
            .collect::<HashMap<&str, Vec<Rule>>>(),
    );
    let result: AtomicU64 = AtomicU64::new(0);

    let mut queue: Vec<(RuleTarget, HashMap<PropertyName, Range<u32>>)> = vec![(
        RuleTarget::Next("in".to_string()),
        HashMap::from([
            (PropertyName::X, 1..4001),
            (PropertyName::M, 1..4001),
            (PropertyName::A, 1..4001),
            (PropertyName::S, 1..4001),
        ]),
    )];

    while !queue.is_empty() {
        let next_queue: Vec<(RuleTarget, HashMap<PropertyName, Range<u32>>)> = queue.par_iter().map_with(rules_map.clone(), |rules_map, (name, properties)| {
            let name = match name {
                RuleTarget::Next(name) => name,
                RuleTarget::Allow => {
                    result.fetch_add(calculate_properties_value(properties), std::sync::atomic::Ordering::Relaxed);
                    return vec![];
                },
                RuleTarget::Reject => {
                    return vec![];
                }
            };
            if let Some(rules) = rules_map.get(name.as_str()) {
                let mut result = Vec::new();
                let mut properties = properties.clone();
                for rule in rules {
                    let splitted = rule.split(&properties);
    
                    if let Some(prop_ranges) = splitted.1 {
                        result.push((splitted.0.clone(), prop_ranges));
                    }
    
                    if let Some(prop_ranges) = splitted.2 {
                        properties = prop_ranges;
                    }
                }
                result
            } else {
                panic!("invalid target {name}");
            }
        }).flatten().collect();
        queue = next_queue;
    }

    result.load(std::sync::atomic::Ordering::Relaxed)
}

fn calculate_properties_value(properties: &HashMap<PropertyName, Range<u32>>) -> u64 {
    let mut result = 1;
    for (_, range) in properties {
        result *= range.len() as u64;
    }
    result
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum PropertyName {
    X,
    M,
    A,
    S,
}
#[derive(Debug)]
struct Property {
    name: PropertyName,
    value: u32,
}
impl From<&str> for Property {
    fn from(value: &str) -> Self {
        let name = &value[0..1];
        let amount = value[2..].parse::<u32>().unwrap();
        match name {
            "x" => Self {
                name: PropertyName::X,
                value: amount,
            },
            "m" => Self {
                name: PropertyName::M,
                value: amount,
            },
            "a" => Self {
                name: PropertyName::A,
                value: amount,
            },
            "s" => Self {
                name: PropertyName::S,
                value: amount,
            },
            _ => panic!("invalid property name {value}"),
        }
    }
}

#[derive(Debug)]
enum Rule {
    Gt(Property, RuleTarget),
    Lt(Property, RuleTarget),
    Always(RuleTarget),
}

type RuleSplitResponse<'a> = (
    &'a RuleTarget,
    Option<HashMap<PropertyName, Range<u32>>>,
    Option<HashMap<PropertyName, Range<u32>>>,
);

impl Rule {
    fn check<'a>(&'a self, props: &HashMap<PropertyName, u32>) -> Option<&'a RuleTarget> {
        match self {
            Rule::Gt(prop, target)
                if &prop.value < props.get(&prop.name).expect("should contain prop") =>
            {
                Some(target)
            }
            Rule::Lt(prop, target)
                if &prop.value > props.get(&prop.name).expect("should contain prop") =>
            {
                Some(target)
            }
            Rule::Always(target) => Some(target),
            _ => None,
        }
    }

    fn split<'a>(&'a self, props: & HashMap<PropertyName, Range<u32>>) -> RuleSplitResponse<'a> {
        match self {
            Rule::Gt(prop, target) => {
                let prop_range = props.get(&prop.name).expect("should contain prop");
                let prop_ranges = if prop_range.end <= prop.value {
                    None
                } else {
                    Some([
                        props
                            .iter()
                            .map(|(k, v)| {
                                if k == &prop.name {
                                    (
                                        *k,
                                        Range {
                                            start: prop.value+1,
                                            end: v.end,
                                        },
                                    )
                                } else {
                                    (*k, v.clone())
                                }
                            })
                            .collect::<HashMap<PropertyName, Range<u32>>>(),
                        props
                            .iter()
                            .map(|(k, v)| {
                                if k == &prop.name {
                                    (
                                        *k,
                                        Range {
                                            start: v.start,
                                            end: prop.value+1,
                                        },
                                    )
                                } else {
                                    (*k, v.clone())
                                }
                            })
                            .collect::<HashMap<PropertyName, Range<u32>>>(),
                    ])
                };

                (
                    target,
                    prop_ranges
                        .as_ref()
                        .map(|prop_ranges| prop_ranges[0].clone()),
                    if let Some(prop_ranges) = &prop_ranges {
                        Some(prop_ranges[1].clone())
                    } else {
                        Some(props.clone())
                    },
                )
            }
            Rule::Lt(prop, target) => {
                let prop_range = props.get(&prop.name).expect("should contain prop");
                let prop_ranges = if prop_range.start >= prop.value {
                    None
                } else {
                    Some([
                        props
                            .iter()
                            .map(|(k, v)| {
                                if k == &prop.name {
                                    (
                                        *k,
                                        Range {
                                            start: v.start,
                                            end: prop.value,
                                        },
                                    )
                                } else {
                                    (*k, v.clone())
                                }
                            })
                            .collect::<HashMap<PropertyName, Range<u32>>>(),
                        props
                            .iter()
                            .map(|(k, v)| {
                                if k == &prop.name {
                                    (
                                        *k,
                                        Range {
                                            start: prop.value,
                                            end: v.end,
                                        },
                                    )
                                } else {
                                    (*k, v.clone())
                                }
                            })
                            .collect::<HashMap<PropertyName, Range<u32>>>(),
                    ])
                };

                (
                    target,
                    prop_ranges
                        .as_ref()
                        .map(|prop_ranges| prop_ranges[0].clone()),
                    if let Some(prop_ranges) = &prop_ranges {
                        Some(prop_ranges[1].clone())
                    } else {
                        Some(props.clone())
                    },
                )
            }
            Rule::Always(target) => (
                target,
                Some(props.iter().map(|(k, v)| (*k, v.clone())).collect()),
                None,
            ),
            _ => panic!("invalid rule"),
        }
    }
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        if !value.contains(':') {
            return Rule::Always(RuleTarget::from(value));
        }
        let mut split = value.split(':');

        let condition = split.next().unwrap();
        let target = split.next().unwrap();

        match &condition[1..=1] {
            "<" => Rule::Lt(
                Property::from(condition),
                RuleTarget::from(target),
            ),
            ">" => Rule::Gt(
                Property::from(condition),
                RuleTarget::from(target),
            ),
            _ => panic!("invalid condition {value}"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum RuleTarget {
    Allow,
    Reject,
    Next(String),
}
impl From<&str> for RuleTarget {
    fn from(value: &str) -> Self {
        match value {
            "A" => Self::Allow,
            "R" => Self::Reject,
            _ => Self::Next(value.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rstest::rstest;

    const EXAMPLE_1: &str = indoc! {
        "px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}
        
        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}"
    };

    #[rstest]
    #[case::c1(EXAMPLE_1, 19114)]
    fn process1_test(#[case] input: &str, #[case] expected: u32) {
        let result = process1(input);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::example(EXAMPLE_1, 167409079868000)]
    #[case::c01(indoc! {
        "in{A}"
    }, 4000u64.pow(4))]
    #[case::c01(indoc! {
        "in{R}"
    }, 0)]
    #[case::c02(indoc! {
        "in{x>4001:R,A}"
    }, 4000u64.pow(4))]
    #[case::c03(indoc! {
        "in{x>3999:R,A}"
    }, 3999 * 4000u64.pow(3))]
    #[case::c04(indoc! {
        "in{x<4001:asd,A}
        asd{A}"
    }, 4000u64.pow(4))]
    #[case::c05(indoc! {
        "in{x<4000:R,m<4000:R,a<4000:R,s<4000:R,A}"
    }, 1)]
    #[case::c05(indoc! {
        "in{x<4000:R,m>1:R,a>1:R,s>1:R,A}"
    }, 1)]
    #[case::c06(indoc! {
        "in{x<4000:x,m<4000:m,a<4000:a,s<4000:s,A}
        x{R}
        m{R}
        a{R}
        s{R}"
    }, 1)]
    fn process2_test(#[case] input: &str, #[case] expected: u64) {
        let result = process2(input);

        assert_eq!(result, expected);
    }
}
