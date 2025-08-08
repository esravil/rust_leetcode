struct Solution;

impl Solution {

    pub fn concatenate_2_arr(arr: Vec<i32>) -> Vec<i32> {

        // create new arr, where the new arr is double given arr length
        // also, ensure the arr[i] == new_arr[i] && the pattern of values from i to len of arr - 1 indices repeats over and over for the length of the new arr

        let mut new_arr = Vec::new();

        new_arr.extend(&arr);
        new_arr.extend(&arr);

        new_arr

    }   

}

fn main() {

    let v = vec![1, 2, 3];
    let new_vec = Solution::concatenate_2_arr(v);

    println!("{:?}", new_vec);

}