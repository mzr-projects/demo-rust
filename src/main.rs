/*
    Variables in Rust are immutable by default (Safety, Concurrency, Speed)
*/
fn main() {
    /*
       We don't have to specify the type of variable in RUST it can detect it automatically
    */
    let age = 2;
    // we cannot do this because variables are immutable in RUST age = 2;

    /*
        Although we can do it explicitly. Here we assigned 32-bit integer id
    */
    let id: i32 = 3;
    println!("id is {}", id);

    let (my_score, your_score) = (90, 80);
    println!("My score is {},Your scope is : {}", my_score, your_score);

    /*
        Here we defined a mutable variable so we can change its value afterward
    */
    let mut mut_age = 38;
    mut_age = 40;
    println!("mut_age is {}", mut_age);

    /*
        The value (here 42) must be determined in compile time and must be CONSTANT
        The name must have _ in it (ORDER_STATUS)
    */
    const ORDER_STATUS: i32 = 42;
    println!("const order_status is {}", ORDER_STATUS);

    {
        let y = 10;
        println!("age is:{}, y is:{}", age, y)
    }

    /*
    Here we have error because the y is in another scope can't be seen from HERE
    There is no Garbage collection in RUST variables immediately drops when they go out of the scope
    */
    //print!("age is:{}, y is:{}", age, y);

    /*
        In the following code x outside and x inside have nothing to do with each others
    */
    let x = 99;
    {
        let x = 100;
        print!("x in block = {}, ", x);
    }
    println!("x outside the block = {}", x);

    /*
        Here the first mem will be discarded and the second one will be replaced,
    */
    let mem = "More vars";
    let mem = mem.len();
    println!("mem is {}", mem);

    /*
       We cant use uninitialized variables we get compile time error
    */
    //let var: i32;
    //println!("var is {}", var);

    /*
        Even the following code does not work, because this error happens in compile time and
        this is going to be evaluated in Run time, so the compiler cant make sure the variable will
        be initialized
    */
    /* let var: i32;
     if true {
         var = 100;
     }
     println!("var is {}", var);*/

    /*
        But the following code works, Here the compiler can guarantee the variable will be
        initialized no matter what
    */
    let var: i32;
    if true {
        var = 100;
    } else {
        var = 101;
    }
    println!("var is {}", var);
}
