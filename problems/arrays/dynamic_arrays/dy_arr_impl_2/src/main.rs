// implement a dynamic array in rust
struct DynamicArray<T> {

    data: Box<[Option<T>]>, // this shows a slice of Option<T> types in the data!
    len: usize,
    capacity: usize,

}

// implement this type to only hold types that have Copy trait, scalar types
// also, the &self param allows us to call a specific method on an instance of this struct 
// Box() is rudimentary vector, without all the methods included in a vector
// into_boxed_slice converts a vec to a box
impl<T: Copy> DynamicArray<T> {

    pub fn new() -> Self {

        Self {

            data: Box::new([]),
            len: 0,
            capacity: 0,

        }

    }

    pub fn resize(&mut self) {

        // first double size
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };

        // now create a struct in which you can store the prev Dynamic array too
        let mut new_darr = vec![None; new_capacity].into_boxed_slice(); // vector turned box for data field

        for i in 0 .. self.len() {

            new_darr[i] = self.data[i];

        }

        self.capacity = new_capacity;
        self.data = new_darr;


    }

    // validity checks take place
    pub fn push(&mut self, val: T) {

        if self.len >= self.capacity {

            self.resize();

        }

        self.data(self.len) = Some(val);
        self.len += 1;

    }

    // validity checks take place
    // returning Option<&T> is crucial because we want to handle index errors
    // as well as not deep copying contents to return actual value
    pub fn get(&self, index: usize) -> Option<&T> {

        if index < self.len {

            self.data[index].as_ref()

        } else {

            None

        }

    }

    pub fn len(&self) -> usize {

        self.len

    }

    pub fn cap(&self) -> usize {

        self.capacity

    }

}