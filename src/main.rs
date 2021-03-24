use array2d::Array2D;
use num::clamp;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use std::fs::File;
use std::io::{Error, Write};

fn main() -> Result<(), Error> {
    // There are two ways in which to get a default value, one is to use claps Arg::default_value
    // method, and the other is to use Rust's built in Option::unwrap_or method.

    let path = "output_file.txt";

    let print_file_flag = 1;

    let mut output = File::create(path)?;

    // 50% chance to print 'a', 25% chance to print 'b', 25% chance to print 'c'
    let choices = ['█', '▓', '▒', '░'];
    let weights = [4, 3, 2, 1];
    let mut height = 40;
    let mut width = 400;
    height = clamp(height, 1, height);
    width = clamp(width, 1, width);
    let mut random_choice = ' ';
    println!("The choices are {:?}", choices);
    println!("The weights are {:?}", weights);
    println!("The height is {}", height);
    println!("The width is {}", width);
    let mut array = Array2D::filled_with(' ', width, height);

    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = thread_rng();
    for x in 0..height {
        for y in 0..width {
            random_choice = choices[dist.sample(&mut rng)];
            if print_file_flag == 1 {
                print!("{} ", random_choice);
                write!(output, "{}", random_choice)?;
                array.set(x, y, random_choice); //
            } else {
                print!("{} ", random_choice);
            }
        }
        if print_file_flag == 1 {
            println!();
            write!(output, "\n")?;
        } else {
            println!();
        }
    }

    Ok(())
}
