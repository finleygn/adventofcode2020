use regex::Regex;
use std::collections::HashMap;

type Bag = String;

#[derive(Debug)]
struct Containable {
    bag: Bag,
    total: u32,
}

#[derive(Debug)]
struct BagRules {
    pub bags: Vec<Bag>,
    pub containment_rules: HashMap<Bag, Vec<Containable>>,
}

impl BagRules {
    pub fn create_from_ruleset(rules: Vec<String>) -> BagRules {
        let line_regex: Regex =
            Regex::new(r"^(.*) bags contain ((\d.*? bags?)|no other bags).$").unwrap();
        let content_regex: Regex = Regex::new(r"^(\d) (.*) bags?$").unwrap();

        let mut bag_rules = BagRules {
            bags: Vec::new(),
            containment_rules: HashMap::new(),
        };

        for plain_rule in rules {
            let rule = line_regex.captures(&plain_rule).unwrap();

            let line_name = rule.get(1).map_or("", |m| m.as_str());
            let no_content = rule.get(2).map_or("", |m| m.as_str());
            let content = rule.get(3).map_or("", |m| m.as_str());

            let contained = {
                if no_content == "no other bags" {
                    vec![]
                } else {
                    let mut item_names: Vec<Containable> = vec![];

                    for containable in content.split(", ") {
                        let item = content_regex.captures(containable).unwrap();
                        let item_count = item.get(1).map_or("", |m| m.as_str());
                        let item_name = item.get(2).map_or("", |m| m.as_str());

                        item_names.push(Containable {
                            bag: String::from(item_name),
                            total: item_count.parse::<u32>().unwrap(),
                        });
                    }

                    item_names
                }
            };

            bag_rules.bags.push(String::from(line_name));
            bag_rules
                .containment_rules
                .insert(String::from(line_name), contained);
        }

        bag_rules
    }

    pub fn bag_can_contain_bag(&self, bag: &Bag, target: &Bag) -> bool {
        for containable in self.containment_rules.get(bag).unwrap() {
            if &containable.bag == target {
                return true;
            } else {
                if self.bag_can_contain_bag(&containable.bag, &target) {
                    return true;
                }
            }
        }

        false
    }

    pub fn sum_containment(&self, bag: &Bag) -> u32 {
        let mut total = 0;

        for containable in self.containment_rules.get(bag).unwrap() {
            total += containable.total;
            total += containable.total * self.sum_containment(&containable.bag);
        }

        total
    }
}

pub fn part1(rules: Vec<String>) -> u32 {
    let bag_rules = BagRules::create_from_ruleset(rules);
    let mut total = 0;

    for bag in &bag_rules.bags {
        if bag_rules.bag_can_contain_bag(bag, &String::from("shiny gold")) {
            total += 1;
        }
    }

    total
}

pub fn part2(rules: Vec<String>) -> u32 {
    let bag_rules = BagRules::create_from_ruleset(rules);
    bag_rules.sum_containment(&String::from("shiny gold"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn d7part1test() {
        let data = util::lines_from_file("./data/day7.txt");
        assert_eq!(119, part1(data));
    }

    #[test]
    fn d7part2test() {
        let data = util::lines_from_file("./data/day7.txt");
        assert_eq!(155802, part2(data));
    }
}
