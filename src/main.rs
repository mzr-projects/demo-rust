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

    println!("############ Loops");
    loops(buf, buf2);
    println!("############ strings");
    strings();
    println!("############ Ownership");
    ownership();
    println!("############ Reference");
    reference_stuff();
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

fn strings() {
    /*
        Here word is an immutable string, known as string slice
    */
    let word = "Hello World";
    let word_bytes = word.bytes();
    println!("word_bytes = {:?}", word_bytes);

    /*
        my_string is a mutable string known as owned string
    */
    let mut my_string = String::from("Hello, Rust!");
    println!("1st my_string: {}", my_string);
    my_string = String::from("Mutable Hello, Rust!");
    println!("2nd my_string: {}", my_string);

    let slice_string_to_mutable_string = word.to_string();
    println!("slice_string_to_owned_string = {}", slice_string_to_mutable_string);

    let mut string_with_capacity = String::with_capacity(5);
    string_with_capacity = String::from("sad_sad");
    string_with_capacity.push('!');
    string_with_capacity.push_str(", dont worry bro!");
    println!("string_with_capacity = {}", string_with_capacity);
}

fn ownership() {
    let mut string_with_capacity = String::with_capacity(5);
    string_with_capacity = String::from("sad_sad");
    /*
        We can't use the following code because we have moved the string_with_capacity variable to
        the s2 and that's no longer available to access:

        let s2 = string_with_capacity;
        println!("string_with_capacity = {}", string_with_capacity);

        string_with_capacity stores in stack memory with a pointer, len and capacity push onto
        the stack which the pointer points to an address in the HEAP section of the memory when
        we use "let s2 = string_with_capacity" the pointer, len and capacity of the new s2 variable
        points to the same heap address so rust will invalidate the string_with_capacity variable
        because in RUST only one owner is valid.
    */

    /*
        But if we want to just copy the value instead of moving it,
        we can do just like the following
    */
    let string_with_capacity_clone = string_with_capacity.clone();
    println!("string_with_capacity = {}", string_with_capacity);
    println!("string_with_capacity_clone = {}", string_with_capacity_clone);

    let s1 = String::from("hello");
    do_stuff(s1);
    /*
        We cant use the following code because the s1 is moved to the function and cant be accessed.
            println!("s1 = {}", s1);
    */

    let mut s3 = String::from("hello");
    s3 = do_stuff(s3);
    println!("s3 = {}", s3);
}

fn do_stuff(s: String) -> String {
    s.to_uppercase()
}

fn reference_stuff() {
    let s1 = String::from("abc");
    /*
        Here we pass a reference to do_stuff_with_reference, and the s1 retains the ownership of the
        value.When we use reference,
        RUST will create a pointer to s1 and references are immutable by default
    */
    do_stuff_with_reference(&s1);
    /*
        We can use s1 after passing the reference because the value never moved we are just passing
        the reference
    */
    println!("s1 = {}", s1);

    /*
        Here we can change the value of string with by making it mutable
    */
    let mut s2 = String::from("abc");
    do_stuff_with_reference_mut(&mut s2);
}

/*
    The & indicates the reference to a type, here we have a reference to String type
    here the do_stuff_with_reference borrows the reference from s1 in reference_stuff method
*/
fn do_stuff_with_reference(s: &String) {
    println!("reference s1 = {}", s);
}

fn do_stuff_with_reference_mut(s: &mut String) {
    /*
        Here s. the "." itself will auto dereference s down into the actual value
        we also can do like the following
            (*s).insert_str(0,"Hi, ");
        Here * will dereference s and gives us the value
    */
    s.insert_str(0, "Hi, ");
    println!("reference s2 with mut = {}", s);
}