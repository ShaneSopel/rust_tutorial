//Rust Tutorial #4 - Data Types

fn main() {
     
    //Scalar Types
    //default is an integer
    let x = 2;
    println!("x is {}",x);
    // i8 -128 to 127
    // i16
    // i32
    // i64
    // i128

    //unsigned integer
    let y : u32 = 972;
    println!("y is {}", y);
    //u8 0 to 255
    //u64
    //u128

    //floating points single and double 
    let float1 : f32 = 2.7;
    let float2 : f64 = 10.9;
    println!("float1 is {}",float1);
    println!("float2 is {}",float2);

    //bools 
    let true_or_false: bool = true;
    println!("bool is {}", true_or_false);

    //char
    let letter: char = 's';
 
    //Compound Types
    //tuple
    let mut typ: (i32, bool, char) = (1, true, 's');
    let typ1: (i8, bool, char) = (1, true, 's');

    println!{"tup part 0 is {}", typ.0};
    println!{"tup part 1 is {}", typ.1};
    println!{"tup part 2 is {}", typ.2};

    println!{"tup1 part 0 is {}", typ1.0};
    println!{"tup1 part 1 is{}", typ1.1};
    println!{"tup1 part 1 is{}", typ1.2};


    typ.0 = 10;

    println!{"tup part 0 {}", typ.0};

    // cannot add things to tuples ..yet?

    // Arrays
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Needs to be initialized.  
    //let mut arr1: [i32; 5] = [];
    arr[4] = 45;
    println!("{}", arr[4]);

    // Will not work. different int types.
    //let ux: u8 = 4;
    //let y: u16 = ux;
    //println!("{}, {}", ux, y);
    
}
