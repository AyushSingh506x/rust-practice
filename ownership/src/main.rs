use std::io;
fn main() {
    let  s  = String::from("hello");
    ownership(s);
    println!("{}",s);
    let x:i64 = 5;
    someInt(x);
    println!("{x}");


}
fn ownership(some_string: String){
println!("{ }",some_string);
}
fn someInt(some_int :i64 ){
    println!("{}",some_int)
}

