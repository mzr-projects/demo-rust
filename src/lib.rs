/*
    We made this function public by pub
*/
pub fn do_something(qty: i32, oz: f64) -> f64 {
    println!("do_something({}, {}) var(s)!", qty, oz);
    oz * 100.00
}