use core::num;

use rayon::prelude::*;

fn main() {
    // Create a large vector of integers from 1 to 1,000,000
    let numbers:Vec<i32> = (1..1_000_001).collect();

    //sequential sum
    let seq_sum: i32 = numbers.iter().sum();
    println!("Sequential Sum: {}", seq_sum);

    //parallel sum using Rayon
    let par_sum: i32 = numbers.par_iter().sum();
    println!("Parallel Sum: {}", par_sum);

    //paraller map example
    let squared_numbers: Vec<i32> = numbers.par_iter().map(|x| x * x).collect();
    println!("First 10 squared numbers: {:?}", &squared_numbers[..10]);

    //filter example
    let even_numbers: Vec<i32> = numbers.par_iter().filter(|x| *x % 2 == 0).cloned().collect::<Vec<_>>().into_iter().take(10).collect();
    println!("First 10 even numbers: {:?}", even_numbers);
}
