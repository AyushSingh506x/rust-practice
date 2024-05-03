mod nth_fibonacci;
mod temperature_convertor;

use std::io;
fn main() {
    // let mut str = String::new();
    // io::stdin()
    //     .read_line(&mut str)
    //     .expect("error");
    // let  n:i32 = str.trim().parse().expect("enter a number");
    // nth_fibonacci::fibonacci(n);
    let mut  unit = String::new();
    let mut temp = String::new();
    println!("enter the unit of temperature by typing C or F: ");
    io::stdin()
        .read_line(&mut unit)
        .expect("error");
    let unit:i32= match unit.trim().parse(){
        Ok(unit)=>unit,
        Err(_)=>{
            -1
        }
    };
    println!("enter the temperature: ");
    io::stdin()
        .read_line(&mut temp)
        .expect("error");
    let  temp:i32 = match temp.trim().parse(){
        Ok(temp)=>temp,
        Err(_)=>{
            -1
        }
    };
    temperature_convertor::temp_convertor(unit,temp);
    let mut x = 0;
    let mut xyz="ayush";
    let mut xyzz = xyz;
    }



