// since we know that linked lists are comprised of nodes, lets first create a node struct

// linked lists are considered recursive types in rust
// recursive meaning:
// storing data on heap which is unknown size at compile time &&
// can have another value of the same type as part of itself (lists are made up of nodes)

// So basically, box is a smart pointer to heap data. Literally just the pointer to the data on the heap.
struct Node<T> {

    val: T,
    // since we need the pointer to either have a Some or None, use Option enum to wrap boxed data on the heap
    ptr: Option<Box<Node<T>>>,

}

// Linked list struct with dummy head and tail
struct LinkedList<T> {
    head: Box<Node<T>>, // Always points to a dummy node
    tail: *mut Node<T>, // Raw pointer to the last node
}

impl<T: Copy + Debug> LinkedList<T> {
    // Create a new linked list with a dummy node
    fn new(dummy_val: T) -> Self {
        let dummy = Box::new(Node {
            val: dummy_val,
            next: None,
        });
        let dummy_ptr = &*dummy as *const Node<T> as *mut Node<T>;
        LinkedList {
            head: dummy,
            tail: dummy_ptr,
        }
    }

    // Insert a value at the end of the list
    fn insert_at_end(&mut self, val: T) {
        let new_node = Box::new(Node {
            val,
            next: None,
        });
        let new_node_ptr = &*new_node as *const Node<T> as *mut Node<T>;

        unsafe {
            (*self.tail).next = Some(new_node);
            self.tail = new_node_ptr;
        }
    }

    // Remove a node at the given index
    fn remove_at_index(&mut self, index: usize) {
        let mut i = 0;
        let mut curr = &mut self.head;

        // Traverse to the node before the one to remove
        while i < index {
            if let Some(ref mut node) = curr.next {
                curr = node;
                i += 1;
            } else {
                return; // Index out of bounds
            }
        }

        // Remove the node ahead of curr
        if let Some(removed_node) = curr.next.take() {
            curr.next = removed_node.next;
            // Update tail if the removed node was the last one
            if curr.next.is_none() {
                self.tail = &mut **curr as *mut Node<T>;
            }
        }
    }

    // Print the list starting from the node after the dummy
    fn print(&self) {
        let mut curr = &self.head.next;
        while let Some(node) = curr {
            print!("{:?} -> ", node.val);
            curr = &node.next;
        }
        println!();
    }
}

// Implement Drop to avoid memory leaks with raw pointers
impl<T: Copy> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut curr = self.head.next.take();
        while let Some(node) = curr {
            curr = node.next;
        }
        self.tail = std::ptr::null_mut();
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

    // Create a new, empty linked list

    // simple constructor, w no nodes!
    fn new() -> Self {
        LinkedList {
            head: None,
            tail: None
        }
    }

    // Push a value to the front of the list (TO THE FRONT)
    fn push(&mut self, val: T) {

        // instantiating the node
        let mut new_node = Box::new(Node {
            val,
            next: self.head.take()
        });

        // raw pointer to new node, for the case where tail needs to point to smth without having a double free error
        let new_node_ptr = &mut *new_node as *mut Node<T>; // need raw pointer to bypass the ownership rules for new node

        // reassign the head pointer
        self.head = Some(new_node);
        
        // If the list was empty, set the tail to the new node
        if self.tail.is_none() {
            self.tail = Some(new_node_ptr); 
    }

    // Append a value to the end of the list
    fn append(&mut self, val: T) {
        let new_node = Box::new(Node {
            val,
            next: None
        });

        // unowned raw pointer to new node
        let new_node_ptr = &mut *new_node as *mut Node<T>;
        
        match self.tail {
            Some(tail_ptr) => unsafe { // tail pointer points to the current tail
                (*tail_ptr).next = Some(new_node); // deref the raw pointer, giving the Node<T> at the addy, and .next gets the next pointer val
                self.tail = Some(new_node_ptr);
            },
            None => {
                // If the list is empty, set both head and tail
                self.head = Some(new_node);
                self.tail = Some(new_node_ptr);
            }
        }
    }

    // Pop the value from the front of the list
    fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None, // If the list is empty, return None
            Some(node) => {
                self.head = node.next; // Move the next node to the head
                
                // Update tail if the list becomes empty
                if self.head.is_none() {
                    self.tail = None;
                } else if self.tail.is_some() && node.next.is_none() {
                    // If we popped the last node, tail needs to point to the new last node
                    // This case is rare since we usually pop from the front
                    let new_tail = &mut *self.head.as_mut().unwrap() as *mut Node<T>;
                    self.tail = Some(new_tail);
                }
                
                Some(node.val) // Return a copy of the value
            }
        }
    }

    // Check if the list is empty
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

// Implement Drop to avoid memory leaks with raw pointers
impl<T: Copy> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {} // Clear the list to deallocate all nodes
        self.tail = None; // Ensure tail is cleared
    }
}

// Example usage in main
fn main() {
    let mut list: LinkedList<i32> = LinkedList::new(); // i32 implements Copy
    
    // Test pushing and appending elements
    list.push(1);    // List: 1
    list.append(2);  // List: 1 -> 2
    list.push(3);    // List: 3 -> 1 -> 2
    list.append(4);  // List: 3 -> 1 -> 2 -> 4
    
    // Test popping elements
    while let Some(value) = list.pop() {
        println!("Popped: {}", value);
    }
    
    // Check if the list is empty
    println!("Is list empty? {}", list.is_empty());
}