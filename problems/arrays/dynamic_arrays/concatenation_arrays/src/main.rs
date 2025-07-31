struct Solution;

impl Solution {

    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {

        // given vector
        // want to make a new vec
        // must fulfill nums[i] = new[i], and nums[i] = new[i + n]
        // essentially meaning how many times we want to loop this array into n times
        // they specifically want 2 iterations of nums
        // how can we do this in 1 pass?
        // if we do iter from 0 to len new vector, we will get error

        let l = nums.len();

        // let mut v: Vec<i32> = vec![0; l * 2]; // population is redundant
        // let mut v: Vec<i32> = Vec::with_capacity(l * 2); // cannot use 

        // for i in 0..nums.len() { // didnt use l just to be more verbose

        //     v[i] = nums[i];
        //     v[i + l] = nums[i];
        //     // cannot index elements in the v vector if no values, need to push

        // }

        let mut v: Vec<i32> = Vec::with_capacity(l * 2);
        v.extend(&nums);
        v.extend(&nums); // most concise


        v // return the vector

    }

}

fn main() {

    let v = vec![1, 2, 3];
    let doubled_vector = Solution::get_concatenation(v);
    println!("{:?}", &doubled_vector[..]);

}