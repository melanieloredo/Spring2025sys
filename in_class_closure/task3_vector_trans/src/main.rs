fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    //Using map and collect, a closure to transform each element of a vector
    vec.into_iter().map(f).collect()
}

/*fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
   //Using a for loop, a closure to transform each element of a vector
   let mut result = Vec::new();
    for x in vec {
        result.push(f(x));
    }
    result
} */

fn main() {
    let numbers = vec![1, 2, 3];

    let doubled = process_vector(numbers.clone(), |x| {
        // Implement: multiply each number by 2
        x * 2
    });

    let replaced = process_vector(numbers, |x| {
        // Implement: if number > 2, replace with 0, else keep number
        if x > 2
           {0}
        else
            {x}
    });

    println!("Doubled: {:?}", doubled);
    println!("Replaced: {:?}", replaced);


}