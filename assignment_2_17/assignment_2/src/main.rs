fn main() {
    let array: [i32; 10] = [1, 2, 9, 22, 7, 3, 15, 18, 5, 24];
    //for loop for stating if even/odd
    //and for divisible by 3/5
    for num in array.iter(){
        if is_even(*num){
            println!("{} is even", num);
        } else{
            println!("{} is odd", num);
        }

        if num % 3 == 0 && num % 5 == 0{
            println!("FizzBuzz");
        } else if num % 3 == 0{
            println!("Fizz");
        }else if num % 5 == 0{
            println!("Buzz");
        }
    }

    //for the sum of all numbers in array
    let mut i = 0;
    let mut sum = 0;
    while i < array.len(){
        sum += array[i]; //add number
        i +=1; //incr counter
    }
    //print sum
    println!("The sum of all the numbers in the array is: {}", sum);

    //store the first variable as largest
    let mut largest = &array[1];
    //iterate through checking if there is a larger number
    for num in array.iter(){
        if num > largest{
            largest = num;
        }
    }
    //print largest number
    println!("The largest number in the array is: {}", largest);
}

//function to check if even
fn is_even(n: i32) -> bool{
    n % 2 == 0 //if num is divisible by 2 then even
}

