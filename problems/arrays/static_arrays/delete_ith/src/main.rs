fn main() {
    
    let mut arr: [i32; 3] = [1, 2, 3];
    let l = arr.len();

    delete_ith(&mut arr, l, 0);

    println!("{:?}", arr);

}

fn delete_ith(arr: &mut [i32], length: usize, i: usize) {

    // so assume we delete first element
    // all elements will need to shift left 1 unit.
    
    // therefore, we need to iterate from beginning of arr.
    // for each ith value, we swap with the i + 1 index val
    // therefore, the 2 last values will always be the same.

    // since we want to remove the i
    // also, we wont actually iterate from 0th always, since i can vary

    // therefore, index is the next val (i + 1), and index - 1 = index
    for index in i + 1 .. length {

        arr[index - 1] = arr[index];

    }

}