// was overthinking this problem
// not like we implement methods for a new struct entirely, we can
// use vector methods

struct MinStack {

    stack: Vec<i32>,
    min_stack: Vec<i32>,

}

impl MinStack {

    fn new() -> Self {

        Self {

            stack: Vec::new(),
            min_stack: Vec::new(),

        }

    }

    fn push(&self, val: i32) {

        self.stack.push(val);

        

    }

}