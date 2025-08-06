// my implementation of dynamic array from scratch

struct DynamicArray<T> {

    data: Box<[Option<T>]>,
    len: usize,
    capacity: usize,

}

impl<T: Copy> DynamicArray<T> {

    pub fn new() -> Self {

        Self {

            data: Box::new([]),
            len: 0,
            capacity: 0,

        }

    }

    pub fn resize(&mut self) {

        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let mut data = vec![None; new_capacity].into_boxed_slice(); // store vec as a box type

        for i in 0 .. self.len() {

            data[i] = self.data[i]; // new arr has the prev array elements until capacity

        };

        self.data = data;
        self.capacity = new_capacity;

    }

    pub fn push(&mut self, val: T) {

        if self.len >= self.capacity {

            self.resize();

        }

        // now we know the arr isnt full
        self.data[self.len] = Some(val);
        self.len += 1;

    }

    pub fn get(&self, index: usize) -> Option<&T> {

        if index < self.len {

            return self.data[index].as_ref();

        } else {

            None

        }

    }

    pub fn len(&self) -> usize {

        self.len

    }

    pub fn capacity(&self) -> usize {

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