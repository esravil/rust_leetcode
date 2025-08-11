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

struct LinkedList<T> {

    head: Option<Box<Node<T>>>,
    tail: Option<*mut Node<T>>,

}

impl<T: Copy> LinkedList<T> {



}