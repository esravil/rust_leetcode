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

    fn new(val: i32) -> Self {

        let dummy_node = Box::new(

            Node {

                val,
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

        while i < index {

            if let Some(node) = curr {

                if i == index { // if found, we return assoc val

                    return node.val;

                }

                curr = &node.next // iterate through the nodes
                i += 1; // inc the loop cond

            }

        }

        -1

    }

    fn insert_head(&mut self, val: i32) {

        // new node
        let new_node = Box::new(

            val,
            next: self.head.next.take(), // we take the Node owned by Box out with take

        );

        self.head.next = Some(new_node); // now we place it in a option enum, as specced by node enum next field

        // this part is a bit iffy bc of the # of method chains, but its to update the tail val
        if self.head.as_ref().unwrap().next.is_none() {

            self.tail = &mut self.head.next.as_mut().unwrap() as *mut Node;

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

            (*self.tail).next = new_node;
            self.tail = ptr;

        }

    }

    fn remove(&mut self, index: usize) -> bool {

        let mut i = 0;
        let mut curr = &self.head;

        // iter to the node before the indexed node
        while i < index {

            if let Some(ref mut node) = curr {

                curr = &node.next;
                i += 1;

            } else {

                return false;

            }

        }

        if let Some(removed_node) = curr.next {

            curr.next = removed_node.next;

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

    fn drop(&self) {

        let mut curr = self.head.next.take();
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