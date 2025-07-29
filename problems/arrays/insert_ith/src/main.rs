fn main() {
    
    let mut arr: [i32; 5] = [1, 2, 3, 4, 0]; // lets make it so that the 4 is in index 4
    let l = arr.len();

    insert_ith(&mut arr, 3, l, 5);
    println!("{:?}", arr);

}

fn insert_ith(arr: &mut [i32], i: usize, length: usize, value: i32) {

    // in this problem, we must insert i at a trivial location
    // assume the extreme, and i index is 0
    // inserting value there moves every item in arr to the left
    // we also assume that len < capacity from allocator to make problem simpler

    // if we move items in arr to the right, we need to make space to replace first index item
    // given we are solving the extreme

    // iterate backwards
    // last index = length - 1, then iterate backwards 1 by 1
    // we want to iterate to i + 1, since i will simply be replaced
    for index in (i + 1..length).rev() {

        arr[index] = arr[index - 1]; // next index val == prev val

    }

    arr[i] = value;

}