
//Rust Tutorial #3 - Variables, Constants and Shadowing

pub fn fun()
{
    println!("You have entered the fun function");
    
    let x = 4; 
    println!("x is {}", x);

    // different interior scope. 
    // x is only equal to two here. 
    {
        let x = x - 2;
        println!("x is {}", x);
    }
    // x goes back to being 4 here. 
    let x = x + 1;
    println!("x is {}", x);
}

fn main() {

    // constant vars
    const SECONDS_IN_MINUTE: u32 = 60;
    const MINUTES_IN_HOUR: u32 = 60;
    
    println!("Seconds in minutes {}", SECONDS_IN_MINUTE);

    let x = 3;
    println!("x is {}", x);
    
    // Cannot do this. All variables that we define are immutable. 
    // Must redeclare
    //x = 5; 
    //println!("x is {}", x);

    // how to make muttable 
    let mut y = 4;
    println!("y is: {}", y);
    y = 5;
    println!("y is: {}", y);

    // how to get around muttable rules. basically redleclaring. 
    let x = 4;
    println!("x is {}", x);
    let x = 5;
    println!("x is {}", x);

    fun();
}


