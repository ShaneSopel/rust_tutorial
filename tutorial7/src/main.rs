//Rust Tutorial #7 - Conditions and Control Flow (if/else if/else)

fn main() {
    
    // Condition operators in rust
    // <
    // >
    // <=
    // >=
    // !=
    // ==

    let cond = 2 < 3;
    println!("{}", cond);

    let cond1 = 2 <= 2;
    println!("{}", cond1);

    // run into an error here different types....
    //let cond2 = 2 <= 2.2;
    //println("{}", cond2);
    
    // && - AND
    // || - OR
    // !  - NOT

    // applied in this order !, && , ||

    let cond3 = (2 as f32) <= 2.2;

    let cond4 = true && cond3;
    println!("{}", cond4);


    let cond5 = (2 as f32) <= 2.2;
    let cond6 = false || !cond5;

    println!("{}", cond6);


    // if, else if , else

    let food = "cookie";

    if food == "cookie"
    {
        println!("The food is a cookie.");
    }
    
    // else

    let food1 = "pasta";

    if food1 == "cookie"
    {
        println!("The food is a cookie.");
    }
    else
    {
       println!("I wanted a cookie.");
    }

    // else if
    let food2 = "bread";
    if food2 == "cookie"
    {
        println!("The food is a cookie.");
    }
    else if food2 == "fruit"
    {
        println!("Im healthy.")
    }
    else if food2 == "bread"
    {
        println!("mmmmmmm. carbs :)")
    }
    else
    {
       println!("I wanted a cookie.");
    }

}
