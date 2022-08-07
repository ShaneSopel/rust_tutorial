// Rust Tutorial #8 - Functions, Expressions & Statements

fn main() 
{
    println!("Hello, world!");
    test();
    add_numbers(20,30);

    // statement
    let _x = 20;

    // expression
    let _y = 2 < 3;

    // expression
    let number = 
    {
        let x = 3;
        x+1
    };
    println!("{}", number);

    let result = add_numbers1(30, 30);
    println!("result is {}", result);

    let result2 = add_numbers2(30, 30);
    println!("result is {}", result2);

    let result3 = add_numbers3(30, 30);
    println!("result is {}", result3);


}

fn test()
{
    println!("Test has been called.")
}

fn add_numbers(x: i32, y: i32)
{
    println!("The sum is: {}", x+y);
}

fn add_numbers1(x: i32, y: i32) -> i32
{
    return x+y;
}

fn add_numbers2(x: i32, y: i32) -> i32
{
    let result = x + y;
    result
}

fn add_numbers3(x: i32, y: i32) -> i32
{
    let result = x + y;
    if result > 10
    {
        return result - 10;
    }
    result
}




