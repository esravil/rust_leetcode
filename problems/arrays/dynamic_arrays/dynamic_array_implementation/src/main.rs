// implementing a dynamic array without using vector methods itself.

// this arr can have type specified via vector
struct DynamicArray<T> {

    data: Box<[Option<T>]>,
    len: usize,
    capacity: usize,

}

// this dynamic array only allows for us to have types that have copy trait implemented, hence known size data
impl<T: Copy> DynamicArray<T> {

    fn new() -> Self {

        Self { // the type unwraps to DynamicArray

            data: Box::new([]), // empty val, not using vector because that is high lvl implementation
            len: 0,
            capacity: 0,

        }

    }

    fn resize(&mut self) {

        // we call this if len >= capacity, so no check here needed
        // create a new arr with double capacity from allocator
        let new_cap = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let mut data = vec![None; new_cap].into_boxed_slice();

        for i in 0..self.len {

            data[i] = self.data[i]; // access data in the struct

        }

        // ref to data is new data vec
        self.data = data;
        self.capacity = new_cap;


    }

    fn push(&mut self, val: T) {

        if self.len >= self.capacity {

            self.resize();

        }

        self.data[self.len] = Some(val); // keep in mind that the len is representative of how many elements there are
        self.len += 1;

    }

    fn get(&self, index: usize) -> Option<&T> {

        if index < self.len {

            self.data[index].as_ref() 

        } else {

            None

        }

    }

    fn len(&self) -> usize {

        self.len

    }

    fn capacity(&self) -> usize {

        self.capacity

    }

}

fn main() {

    let mut v = DynamicArray::new();

    v.push(999);
    v.push(111);
    v.push(1);
    v.push(5);
    v.push(33);

    for i in 0 .. v.len() {

        if let Some(val) = v.get(i) {

            println!("{}: {}", i, val);
        
        }

    }

    let c = v.capacity() as f64;
    let l = v.len() as f64;
    let p: f64 = l / c;

    println!("Total capacity: {}", c);
    println!("# of values: {}", l);


    println!("% of space used: {:2}", p);

}