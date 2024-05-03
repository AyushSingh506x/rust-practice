use std::io;
pub(crate) fn temp_convertor(mut unit:i32,mut temp:i32){

    let mut fahrenheit = 0;
    let mut celsius:i32 = 0;

    match unit {
        1 =>{
            let fahrenheit=(temp*(9/5))+32;
            println!("{}", fahrenheit);
        }
        2 =>{
            let celsius = (temp-32)/(9/5);
            println!("{}",fahrenheit);
        }
        _ => {println!("failed to convert")}
    }


}