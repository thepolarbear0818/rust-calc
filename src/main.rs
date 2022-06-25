use std::io;
use std::{i32};


fn main() {
    let mut calc_option = String::new();
    
    let mut a = String::new();

    let mut b = String::new(); 

    println!("Input Your First Number");

    io::stdin()
    .read_line(&mut a)
    .expect("Failed to read line");

    let a: i32 = a.trim().parse().ok().expect("Input not an integer");

    println!("Input Your Second Number");

    io::stdin()
    .read_line(&mut b)
    .expect("Failed to read line");
    let b: i32 = b.trim().parse().ok().expect("Input not an integer");

    println!("Input 0 For Addition");
    println!("Input 1 For Subtraction");
    println!("Input 2 For Multiplication");
    println!("Input 3 For Division");
    println!("Input 4 For Mod");




    io::stdin()
    .read_line(&mut calc_option)
    .expect("Failed to read line");

    let x: i32 = calc_option.trim().parse().expect("Input not an integer");
    if x == 0{
       println!("You have choosen Addition");

       println!("Your Number is {}", a + b);
        
    }

    if x == 1{
        println!("You Have Choosen Subtraction");
        println!("Your Number is {}", a - b);
    }

    if x == 2{
        println!("You Have Choosen Multiplication");
        println!("Your Number is {}", a * b);
    }

    if x == 3{
        println!("You Have Choosen Division");
        println!("If the number is a fraction or a decimal, This program will return 0");
        println!("Your Number is {}", a / b);
    }

    if x == 4{
        println!("You Have Choosen Mod");
        println!("Your Number is {}", a % b);
    }

}
