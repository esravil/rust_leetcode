// lets create a singly linked list, with some assistance
// first we need a Node struct

use std::fmt::Debug;

struct Node<T> {

    // nodes have a val, which in this case is generic type T
    // nodes also have a next, which is a pointer to next "linked" node
    // since it can be None, we use the Option enum to wrap this pointer
    // how can we have a pointer to heap data without a library or another struct? using Box type which can also be used for recursive type implementations
    val: T,
    next: Option<Box<Node<T>>>,

}

// now that we have the node struct, we can make the linked list
// for simplicity sake, we will track both head and tail pointers for easier method implementations

// this struct is trait bound by Copy and Debug traits
struct LinkedList<T: Copy> {

    // Remember, pointers can also be None, so its best to use Option enum!
    head: Box<Node<T>>,
    tail: *mut Node<T>, // raw ptr to last node

}

impl<T: Copy + Debug> LinkedList<T> {

    fn new(dummy_val: T) -> Self {

        // create first node in the linked list
        let dummy = Box::new(

            Node {

                val: dummy_val,
                next: None, // none because it is the first node

            }

        );

        // lets get the raw pointer ready to assign to the tail
        let dummy_ptr = &*dummy as *const Node<T> as *mut Node<T>; // since dummy is a boxed Node, we get Node<T> by deref. Then we need to cast to const before casting to mut for safety

        // returning the linked list with the dummy node
        Self {

            head: dummy,
            tail: dummy_ptr,

        }

    }

    fn insert_at_end(&mut self, val: T) {

        let new_node = Box::new(

            Node {

                val,
                next: None,

            }

        );

        // raw pointer to reassign the tail
        let new_node_ptr = &*new_node as *const Node<T> as *mut Node<T>;

        unsafe {

            (*self.tail).next = Some(new_node);
            self.tail = new_node_ptr;

        }

    }

    fn remove_at_index(&mut self, index: usize) {

        let mut i = 0;
        let mut curr = &mut self.head;

        while i < index {

            if let Some(ref mut node) = curr.next { // curr.next = mut ref node, and thus curr = curr.next in body

                curr = node;
                i += 1;

            } else {

                return;

            }

        }

        if let Some(removed_node) = curr.next.take() { // we take the node from the curr.next.take() and it is removed_node. Then we skip over
            // if next no longer exists, we reset tail as curr after deref and casting

            curr.next = removed_node.next;
            if curr.next.is_none() {

                self.tail = &mut **curr as *mut Node<T>;

            }

        }

    }

    fn print(&self) {

        let mut curr = &self.head.next; // start at node after dummy
        while let Some(node) = curr {

            print!("{:?} -> ", node.val);
            curr = &node.next;

        }
        println!();

    }
}

// impl drop for mem safety
impl<T: Copy> Drop for LinkedList<T> {

    fn drop(&mut self) {

        // just iterate through all nodes for now
        let mut curr = self.head.next.take();
        while let Some(node) = curr {

            curr = node.next;

        } 

        self.tail = std::ptr::null_mut(); // null mutable pointer for after all nodes iterated + dropped

    }

}


// Example usage
fn main() {
    let mut list: LinkedList<i32> = LinkedList::new(-1);
    
    // Insert elements at the end
    list.insert_at_end(1); // List: 1
    list.insert_at_end(2); // List: 1 -> 2
    list.insert_at_end(3); // List: 1 -> 2 -> 3
    list.insert_at_end(4); // List: 1 -> 2 -> 3 -> 4
    
    // Print the list
    print!("Initial list: ");
    list.print(); // Prints: 1 -> 2 -> 3 -> 4 ->
    
    // Remove elements at specific indices
    list.remove_at_index(1); // Remove 2, List: 1 -> 3 -> 4
    print!("After removing index 1: ");
    list.print(); // Prints: 1 -> 3 -> 4 ->
    
    list.remove_at_index(0); // Remove 1, List: 3 -> 4
    print!("After removing index 0: ");
    list.print(); // Prints: 3 -> 4 ->
    
    list.remove_at_index(5); // Out of bounds, no change
    print!("After removing index 5: ");
    list.print(); // Prints: 3 -> 4 ->
    
    list.remove_at_index(1); // Remove 4, List: 3
    print!("After removing index 1: ");
    list.print(); // Prints: 3 ->
}