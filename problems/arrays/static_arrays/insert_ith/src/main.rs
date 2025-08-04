// for this problem, we will be inserting a val, x, at index i
// can / should we assume arr is empty? n/a

fn main() {

    let mut arr = [1, 2, 3, 4, 0]; // 0 indicates empty space..
    insert_ith(&mut arr, 3, 99);
    println!("{:?}", arr);

}

fn insert_ith(arr: &mut [i32], i: usize, value: i32) {

    // we will be inserting at ith index, anywhere in the array
    // lets do extreme, insert at 0th index
    // inserting at 0th index shifts all other items to the right 1 index
    // so index + 1 = index position val, since next index will be prev index val
    // iterating backwards would make sense as well, since that clears up space for insertion at the end

    // we do not need check for len against capacity, just assume
    for index in (i .. arr.len() - 1).rev() {

        arr[index + 1] = arr[index];

    }

    arr[i] = value;

}