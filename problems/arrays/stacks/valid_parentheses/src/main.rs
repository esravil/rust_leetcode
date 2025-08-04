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

        for c in s.chars() {

            // check if the closing bracket in the hashmap
            if hm.contains_key(&c) {

                // .get returns Option<&T>, so unwrap gets &T, and * gets T itself
                if !stack.is_empty() && stack[stack.len() - 1] == *hm.get(&c).unwrap() { // check if last element in stack is the same as the corresponding value to the character

                    stack.pop();

                } else {

                    return false; // we found the error where we have a closing paren, but stack either has no opening bracket or last stack item doesnt correspond.

                }

            } else {

                stack.push(c); // add the opening brackets to the stack

            }

        }

        stack.is_empty(); // if we have in order and valid paren, stack should be empty

    }

}

fn main() {

    let s = "([{}])";
    if Solution::valid_parentheses(s) {

        println!("Valid string.");

    }

}