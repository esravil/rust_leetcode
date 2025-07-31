use std::collections::HashMap;

struct Solution;

impl Solution {

    pub fn valid_parentheses(s: &str) -> bool {

        let mut v: Vec<char> = Vec::new();
        let hm: HashMap<char, char> = HashMap::from([

            (']', '['),
            (')', '('),
            ('}', '{'),

        ]);

        // iterate each char
        for c in s.chars() {

            if hm.contains_key(&c) {

                if !v.is_empty() && v[v.len() - 1] == *hm.get(&c).unwrap() { // .get might not exist but we alr checked that, hence get returns Option<&char> variant. unwrap gets &char inside the option, and * dereferences it

                    v.pop(); // found matching set of paren
                    // we pop the val if we find its last in the stack and in the hashmap

                } else {

                    return false; // meaning we either got an erroneous char, or we have a mismatch paren

                }

            } else {

                v.push(c); 

            }

        };


        v.is_empty() // will execute based on iter

    }

}

fn main() {

    let s = "([{}])";
    if Solution::valid_parentheses(s) {

        println!("Valid string.");

    }

}