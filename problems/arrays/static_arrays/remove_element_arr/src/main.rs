struct Solution;

impl Solution {

    pub fn remove_element_arr(arr: &mut Vec<i32>, val: i32) -> i32 {

        // given a mutable vec
        // remove all instances of val, and keep order of all other elements in tact, but shifted over
        // then return the number of vals in the arr not equal to val, meaning we can have dupes
        if arr.is_empty() {

            return 0; // edge case if arr has no val

        }

        let mut k = 0; // the number of unique vals not == val
        // definitive iteration
        for i in 0 .. arr.len() {

            if arr[i] != val {

                arr[k] = arr[i];
                k += 1;

            }

        }

        k as i32

    }

}

fn main() {

    let mut v = vec![1, 2, 3, 3, 4, 5];
    let k = Solution::remove_element_arr(&mut v, 3);
    println!("{}", k);


}