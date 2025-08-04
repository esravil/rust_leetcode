fn main() {

    let mut arr = [1, 2, 3];
    delete_end(&mut arr);
    println!("{:?}", &arr[..]);

}

fn delete_end(arr: &mut [i32]) {

    if arr.len() >= 1 { // cannot simply do arr.len(), the usize cannot be a bool val

        arr[arr.len() - 1] = 0; // soft delete

    };

}