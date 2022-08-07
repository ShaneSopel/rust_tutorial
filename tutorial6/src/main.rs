// Rust Tutorial #6 - Arithmetic and Type Casting

use std::io;

fn main() {

    //let x : u8 = 9; // 0 - 255
    //let y : i8 = 10; // -128 - 127

    // will not work with different types. 
    //let z = x + y;
    //println!("{}", z);

    let x : u8 = 9; // 0 - 255
    let y : u8 = 10; // -128 - 127

    // will work with same types. 
    let z = x + y;
    println!("{}", z);

    //adding float types
    let _x1: f32 = 12.0; 
    let _y1: f32 = 10.0;

    let _z1 = _x1 + _y1;
    println!("{}", _z1);

    // doing other math operations
    
    // attempting max overflow error
    //let x2: u8 = 255; 
    //let y2: u8 = 1;

    //let z2 = x+y; 
    //println!("{}", z2);

    // attempting max negative overflow error
    //let x2: u8 = 255; 
    //let y2: u8 = 1;

    //let z2 = y-x;    let _x4: f64 = 255.0; 

    // attempting divide with whole integer
    let _x2: u8 = 255; 
    let _y2: u8 = 10;

    let _z2 = _x2 / _y2; 
    println!("{}", _z2);

   // attempting divide with float integer
   let _x3: f64 = 255.0; 
   let _y3: f64 = 10.0;
    
   let _z3 = _x3 / _y3; 
   println!("{}", _z3);

   // attempting multiplication
   let _x4: f64 = 255.0; 
   let _y4: f64 = 10.0;
    
   let _z4 = _x4 * _y4; 
   println!("{}", _z4);

   // attempting mod
   let _x5: f64 = 255.0; 
   let _y5: f64 = 10.0;
    
   let _z5 = _x5 % _y5; 
   println!("{}", _z5);


   //grabbing string and converting to number. 
   let mut input = String::new();
   io::stdin().read_line(&mut input).expect("Expected to read line");
   
   let int_input: i64 = input.trim().parse().unwrap();

   println!("{}", int_input + 2);
    
}
