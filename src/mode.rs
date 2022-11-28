use std::cmp::Ordering;
use std::collections::HashMap;
use std::vec;

pub fn mode(array: &[i32]) -> Vec<i32> {
    let mut nums: HashMap<i32, i32> = HashMap::new();

    for n in array {
        let count = nums.entry(*n).or_insert(0);
        *count += 1;
    } // Counts the occurence of each number in the vector/array

    let first = array.first();
    match first {
        None => {
            vec![] // Empty vector
        }
        Some(first) => {
            let mut the_modes: Vec<i32> = vec![*first]; // Possible mode candidates alongside the current candidate
            let mut prev_amt: i32 = nums.get(first).copied().unwrap(); // value of candidate
                                                                       // Iterates over the nums comparing each one.
                                                                       // Returns num with greatest value
            for (k, pot_amt) in nums {
                if k != the_modes[0] {
                    match prev_amt.cmp(&pot_amt) {
                        Ordering::Less => {
                            the_modes = vec![k]; // create a new vector of possible modes
                            prev_amt = pot_amt
                        }

                        Ordering::Greater => {}

                        Ordering::Equal => the_modes.push(k), // appends the mode candidate
                    }
                }
            }
            the_modes
        }
    }
}
