// linked list impl

// node
struct Node {

    val: i32,
    next: Option<Box<Node>>, 

}

// since we are guaranteed to have a dummy node, we don't wrap Box<Node> in option enum!
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

        // next is set to None, since this is the constructor and 1 node only so far in the linked list
        // lets make a raw ptr for the tail tracking
        let dummy_node_ptr = &*dummy_node as *const Node as *mut Node; // dummy_node is a Box<Node>, we dont want ownership so we reference operator it. We also deref ownership from Box, only get Node as raw ptr!

        // return the linked list struct

        Self {

            head: dummy_node, // don't need Some variant here bc its a Box<Node>
            tail: dummy_node_ptr,

        }

    }

    fn insert_head(&mut self, val: i32) {

        // create said node
        let new_node = Box::new(

            Node {

                val,
                head: self.head.next.take(), // since we have a recursive data struct, we know self.head.next = Option enum variant, so we basically need to extract Box<Node> from enum. Now the prev head.next is None. This happens due to the take() method!

            }

        );

        // now since this was None, we can move the new_node in
        self.head.next = Some(new_node);

        // now we must update the tail as needed

        if self.head.next.as_ref().unwrap().next.is_none() { // since we know self.head.next is a Option<Box<Node>>, as_ref makes it Option<&Box<Node>>, and .unwrap() makes it &Box<Node>. now we check if this Box node next is None as well with .next.is_none()

            self.tail = &mut **self.head.next.as_mut().unwrap() as *mut Node; // we first get self.head.next, which is Option<Box<Node>>, then we know we have some val Box<Node>, so we can do Option<&mut Box<Node>>, unwrapping gets up the &mut Box<Node>, now we deref twice and get &mut Node and cast as *mut Node

        };

    }

    fn insert_tail(&mut self, val: i32) {

        let new_node = Box::new(

            val,
            next: None,

        )

        // raw ptr for tail
        let nn_ptr = &*new_node as *const Node as *mut Node;

        unsafe {

            (*self.tail).next = Some(new_node); // we add after last node, the ptr points to option var some
            self.tail = nn_ptr; // tail has a next, so we reassign tail to the val it pointed to, raw ptr
        }

    }

    fn remove(&mut self, index: usize) {

        let mut i = 0; // we will be iterating using this var
        let mut curr = &self.head;

        while i < index {

            // why is the Some param Some(ref mut node) instead of just Some(node)? Is it because the node val will be referenced via curr = node and thus need to mutable as well for each iter?
            if let Some(ref mut node) = curr.next { // node = curr.next

                curr = node;
                i += 1;

            } else {

                return false; // index out of bounds case

            }

        }

        // curr.next val is moved to removed node, curr.next is now None, and ready for assignment
        if let Some(removed_node) = curr.next.take() { // curr.next is a Option<Box<Node>>, and take() basically takes the value out of the Option enum, makes curr.next = None. Removed_node is now the Boxed node.

            curr.next = removed_node.next; // skip removed node

            if curr.next.is_none() {

                // update self.tail ptr
                self.tail = &mut **curr as *mut Node;

            }

        }

    }

    fn get_values(&self) -> Vec<i32> {

        let mut vals = Vec::new();
        let mut curr = &self.head.next;
        while let Some(node) = curr {

            vals.push(node.val); // we push the stack data
            curr = &node.next;, // ref heap data

        }

    }

}

impl Drop for LinkedList {

    fn drop(&mut self) {

        // iter all elements
        let mut curr = self.head.next.take(); // the self.head.next val moves into curr
        while Some(node) = curr {

            curr = node.next;

        }

        self.tail = std::ptr::null_mut();

    }

}