fn main() {
    
    let mut a: [i32; 3] = [1, 2, 3];
    let length = a.len() - 1;
    delete_end(&mut a, length);
    println!("{:?}", a);

}

fn delete_end(arr: &mut [i32], length: usize) {

    arr[length] = 0; // soft delete, the length is decremented by 1

}
