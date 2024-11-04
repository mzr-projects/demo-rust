/*
    Here we add the do_something method from lib.rs to the scope of the functions in this file
*/
use demo_rust::do_something;
use rand::{thread_rng, Rng};

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

    do_something(12, 101.01);

    let z = thread_rng().gen_range(0..100);
    println!("random value is {}", z);

    /*
        Definition of tuple: tuples are entities with multiple values not necessarily the same type
        For now tuples in RUST have a range of 12 members
    */
    let tuple = (1, "hi", 3.56, 99);
    println!("tuple is {},{},{}", tuple.0, tuple.1, tuple.2);

    /*
        Here there is another way to access tuple members
    */
    let (quantity, name, id, digit) = tuple;
    println!("quantity is {}, name: {}, id: {}, digit: {}", quantity, name, id, digit);

    /*
         Array definition is like the following and they are FIXES size
    */
    let buf = [1, 2, 3];
    /*
        Here we have an array with 3 zeroes
    */
    let buf2 = [0; 3];
    /*
        Here we defined an array with 3 i8 types. We can specify the type and the size explicitly
    */
    let buf3: [i8; 3] = [4, 5, 6];

    /*
        The condition has to be a boolean
    */
    if buf[1] == 2 {
        println!("buf[1] == 2");
    } else if buf[1] == 3 {
        println!("buf[1] == 3");
    } else {
        println!("buf[1] == 4");
    }

    /*
        Here we have an if expression which we return some value out of it, please notice we
        have no ";" at the end of lines and no return as well,
        but we have one at the end of the last curly brace.
    */
    let msg = if buf[1] == 2 {
        "buf[1] == 2"
    } else if buf[1] == 3 {
        "buf[1] == 3"
    } else {
        "buf[1] == 4"
    };
    println!("msg: {}", msg);

    loops(buf, buf2);
}

fn dizzy() -> bool { false }

fn loops(buf: [i32; 3], buf2: [i32; 3]) {
    loop {
        if !dizzy() {
            break;
        }
    }

    while dizzy() {}

    /*
        Rust can iterate over any iterable value. iter() will iterate the array in order
    */
    for x in buf.iter() {
        println!("value = {}", x);
    }

    for i in 0..buf2.len() {
        println!("i = {}, value = {}", i, buf[i]);
    }

    let buf4 = [(1, 2), (3, 4), (5, 6)];
    for (x, y) in buf4.iter() {
        println!("x = {}, y = {}", x, y);
    }

    /*
        Iterate in a RANGE
    */
    for num in 0..10 {
        println!("num = {}", num);
    }
}