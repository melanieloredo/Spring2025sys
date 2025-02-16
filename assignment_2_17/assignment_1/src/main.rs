const FREEZING_POINT_F: f64 = 32.0;

fn main() {
    //initial temp of 32 to start
   let mut temp_f: f64 = 32.0;
   let celsius = fahrenheit_to_celsius(temp_f);
   println!("{:.2}F to celsius is {:.2}C", temp_f, celsius); //print with 2 decimal points

   //loop for printing out the next 5 variables
   for _ in 0..5{
    temp_f += 1.0; //incr
    let celsius = fahrenheit_to_celsius(temp_f);
    println!("{:.2}F to celsius is {:.2}C", temp_f, celsius);
   }
}


fn fahrenheit_to_celsius(f: f64) -> f64{
    (5.0 / 9.0) * (f - FREEZING_POINT_F)
}

fn celsius_to_fahrenheit(c: f64) -> f64{
    (9.0 / 5.0) * (c + FREEZING_POINT_F)
}
