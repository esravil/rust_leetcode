struct Solution;

impl Solution {

    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {

       // we are given a mutable vector sorted, and in non-decreasing order (aka meaning we prob have dupes)
       // remove the dupes, but keep the order

       if nums.is_empty() {

        return 0; // return 0

       }

       // since we handled edge case, knowing that the vector has to have a value:
       let mut k = 1; // k represents the number of unique values
       
       for i in 1 .. nums.len() { // we start at index 1

        if nums[i - 1] != nums[i] {

            nums[k] = nums[i]; // we update starting from the second index, since 1st index is unique
            k += 1; // increase number of unique values

        }

       }

       k as i32

    }

}

fn main() {
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let k = Solution::remove_duplicates(&mut nums);
    println!("k = {}, nums = {:?}", k, &nums[..k as usize]);
}