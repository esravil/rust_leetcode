fn main() {

    let mut v: Vec<i32> = Vec::with_capacity(5);
    insert_end(&mut v, 100);
    println!("{:?}", &v[..]);

}

fn insert_end(arr: &mut Vec<i32>, val: i32) {

    // first check if capacity of arr if full (in rust, arrs are known size so your length = capacity, but vectors are diff since unknown size
    if arr.len() < arr.capacity() {

        arr.push(val);

    }


}