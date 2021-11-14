// use crate::contains_duplicate_217::contains_duplicate;
use crate::missing_number_268::missing_number;

// mod contains_duplicate_217;
mod missing_number_268;

fn main() {
    let nums: Vec<i32> = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
    println!("{}", missing_number(nums));
}