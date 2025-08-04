fn main() {

    let mut arr = [1, 2, 3, 4, 5];
    delete_ith(&mut arr, 0);
    println!("{:?}", &arr[..]);

}

fn delete_ith(arr: &mut [i32], i: usize) {

    // why is i usize and not i32?
    // A: because you cannot index via the i32 without casting via as

    // intuition:
    // since we want to delete the ith index, lets do the extreme condition
    // lets delete the 0th index element
    // if we delete 0th index element, all other elements on right is shifted to the left 1 space
    // we will need to swap the next index with the previous index starting from i + 1

    if arr.len() >= 1 { // rust does NOT provide a method called is empty in the methods for static arrs

        for index in i + 1 .. arr.len() { // as long as we have elements (len), we iter

            arr[index - 1] = arr[index]; // this works because the index = i + 1 index val
            // so index - 1 = ith val, we shift everything to the left

        }

    }

}