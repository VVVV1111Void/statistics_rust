use std::cmp::Ordering;
use std::collections::HashMap;
use std::vec;

fn count_occurence(arr: &[i32]) -> HashMap<i32, i32> {
    let mut nums: HashMap<i32, i32> = HashMap::new();
    for n in arr {
        let count = nums.entry(*n).or_insert(0);
        *count += 1;
    } // Counts the occurence of each number
    nums
}

pub fn mode(array: &[i32]) -> Vec<i32> {
    let nums = count_occurence(&array);
    let first = array.first();
    match first {
        None => {
            vec![]
        }
        Some(first) => {
            let mut the_modes: Vec<i32> = vec![*first]; // possible candidates alongside
            let mut prev_amt: i32 = nums.get(first).copied().unwrap(); // value

            for (k, pot_amt) in nums {
                // iterates over nums
                if k != the_modes[0] {
                    match prev_amt.cmp(&pot_amt) {
                        Ordering::Less => {
                            the_modes = vec![k]; // create a new vector of possible modes
                            prev_amt = pot_amt;
                        }
                        Ordering::Greater => {}               // No action
                        Ordering::Equal => the_modes.push(k), // Append candidate
                    }
                }
            }
            the_modes
        }
    }
}
