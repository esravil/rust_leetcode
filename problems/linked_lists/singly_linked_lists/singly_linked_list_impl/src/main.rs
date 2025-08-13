// singly linked list

// node with val being int specified
struct Node {

    val: i32,
    next: Option<Box<Node>>,

}

struct LinkedList {

    head: Box<Node>,
    tail: *mut Node,

}

impl LinkedList {

    fn new() -> Self {

        let dummy_node = Box::new(

            Node {

                val: -1,
                next: None,

            }

        );

        // ptr
        let d_ptr = &*dummy_node as *const Node as *mut Node;

        Self {

            head: dummy_node,
            tail: d_ptr,

        }

    }

    fn get(&self, index: usize) -> i32 { // returning i32

        // we have to iter to the val, since linked
        let mut i = 0;
        let mut curr = &self.head.next; // we iter from the first non dummy

        while i <= index { // needs to be inclusive of index or else wont ever work.

            if let Some(node) = curr {

                if i == index { // if found, we return assoc val

                    return node.val;

                }

                curr = &node.next; // iterate through the nodes
                i += 1; // inc the loop cond

            }

        }

        -1

    }

    fn insert_head(&mut self, val: i32) {

        let new_node = Box::new(Node {
            val,
            next: self.head.next.take(), // the take method is used on Option or option type
            // this take method take the option enum type and then if has a Some variant, returns the Some val and original option is none
            // if none if the variant of the option, it keeps original option as none, 
            // just basically facilitates a move out of the enum
        });

        self.head.next = Some(new_node); // this None option is now set to a some new node val!

        // If the list was empty, update tail to the new node
        if self.head.next.as_ref().unwrap().next.is_none() {
            self.tail = &mut **self.head.next.as_mut().unwrap() as *mut Node;
        }
    }

    fn insert_tail(&mut self, val: i32) {

        let new_node = Box::new(

            Node {

                val,
                next: None,

            }

        );

        // ptr
        let ptr = &*new_node as *const Node as *mut Node;

        unsafe {

            (*self.tail).next = Some(new_node);
            self.tail = ptr;

        }

    }

    fn remove(&mut self, index: usize) -> bool {

        let mut i = 0;
        let mut curr = &mut self.head;

        // iter to the node before the indexed node
        while i < index {

            if let Some(ref mut node) = curr.next { // q1: why must we make ref mut node = curr.next? why not simply do Some(node) = curr.next? is it because next is a Option<Box<Node>>?

                curr = node;
                i += 1;

            } else {

                // index is found to be invalid / out of bounds
                return false;

            }

        }

        // curr.next = None, and its val is now moved to removed node.
        if let Some(removed_node) = curr.next.take() { // take() moves the next Option enum val in the curr.next and turns it to None, then Some(removed) node gets the moved value

            // now we can assign curr.next to skip next node
            curr.next = removed_node.next; // skip removed node

            if curr.next.is_none() {

                self.tail = &mut **curr as *mut Node; // we do "**" on curr bc its a Option<Box<Node>>, thus yielding raw ptr to Node only

            }

            true

        } else {

            false

        }

    }

    fn get_values(&self) -> Vec<i32> {

        let mut values = Vec::new();
        let mut curr = &self.head.next;

        while let Some(node) = curr {

            values.push(node.val);
            curr = &node.next;

        }
        
        values
            
    }

}

impl Drop for LinkedList {

    fn drop(&mut self) {

        let mut curr = self.head.next.take(); // the Option<Box<Node>> variant is moved to curr, self.head.next is None
        // now that curr is assigned to node.
        // curr = node.next, 
        while let Some(node) = curr {

            curr = node.next;

        }

        self.tail = std::ptr::null_mut(); // since we drop mem refs from nodes being stored on heap

    }

}

// Example usage
fn main() {
    let mut list = LinkedList::new();
    
    // Insert elements
    list.insert_head(1); // List: 1
    list.insert_tail(2); // List: 1 -> 2
    list.insert_head(3); // List: 3 -> 1 -> 2
    list.insert_tail(4); // List: 3 -> 1 -> 2 -> 4
    
    // Get values
    println!("Get index 1: {}", list.get(1)); // 1
    println!("Get index 5: {}", list.get(5)); // -1
    
    // Remove elements
    println!("Remove index 1: {}", list.remove(1)); // true, List: 3 -> 2 -> 4
    println!("Remove index 0: {}", list.remove(0)); // true, List: 2 -> 4
    println!("Remove index 5: {}", list.remove(5)); // false, List: 2 -> 4
    
    // Get all values
    println!("Values: {:?}", list.get_values()); // [2, 4]
}