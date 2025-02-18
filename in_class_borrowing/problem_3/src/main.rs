#[allow(unused_variables, unused_mut)]
fn sum(total: &mut i32, low: i32, high: i32) {
    // Write your code here!
    *total =(low..=high).sum(); //reference the sum
}

fn main() {
    // create necessary variables and test your function for low 0 high 100
    let mut total = 0;
    let low = 0;
    let high = 100;

    sum(&mut total, low, high); //pass total as a ref.
    // total should be 5050

    println!("Total sum of {} to {} is: {}", low, high, total);
}