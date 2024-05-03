use std::io;
pub(crate) fn fibonacci(mut n:u32){
    let  mut x=0;
    let  mut y=1;
    let  mut z=0;
    while n >0{
        z = x+y;
        x=y;
        y=z;
        n-=1;
    }
    println!("{z}");

}