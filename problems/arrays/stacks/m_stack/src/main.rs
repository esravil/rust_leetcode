// lets implement a min stack struct solution with the following:
// constructor
// push
// pop
// top
// get min
// all should run constant time

// solution: have a struct with 2 vectors,
// first vector = all nums
// second vector = all minimum values

struct MinStack {

    stack: Vec<i32>,
    min_stack: Vec<i32>,

}

impl MinStack {

    // dont forget that the struct / constructor doesnt determine the mutability of an instance
    // it depends on the reference var, if it has the mutable keyword!

    // constructor
    fn new() -> Self {

        Self {

            stack: Vec::new(),
            min_stack: Vec::new(),

        }

    }

    // push
    fn push(&mut self, val: i32) {

        // we indiscriminately push all items into the main stack
        self.stack.push(val);

        // in order to add to the minstack, we need to check if its empty or if the current val <= top element in the min_stack
        if self.min_stack.is_empty() || val <= *self.min_stack.last().unwrap() {

            self.min_stack.push(val);

        }

    }

    // pop
    fn pop(&mut self) {

        // we can pop as we wish in the main stack
        // we just have to check if the popped element was the min element, if so we also pop from there too
        // we dont do anything otherwise, {}
        // we can accomplish this with the if let expression easily
        if let Some(val) = self.stack.pop() {

            if val == *self.min_stack.last().unwrap() {

                self.min_stack.pop();

            }

        }

    }

    // top
    fn top(&self) -> i32 {

        *self.stack.last().unwrap()

    }

    // getmin
    fn get_min(&self) -> i32 {

        *self.min_stack.last().unwrap()

    }

}