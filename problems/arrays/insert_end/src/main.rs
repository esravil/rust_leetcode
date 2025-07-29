// for this example, we are guaranteed the insertion is at the end

fn insert_end(arr: &mut [i32], length: usize, n: i32) {

    arr[length] = n;

} // for arrays, if mutable, the arr type inside the sq brackets, and mutable keyword outside

fn main() {

    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    let length = arr.len() - 1; // or else this returns 5, 5 is out of bounds

    insert_end(&mut arr, length, 6); // dont forget in mut ref, need &mut before var
    println!("{:?}", arr);

}