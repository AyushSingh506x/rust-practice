fn main() {
    println!("The twelve days of Christmas!");
    let days = ["first", "second", "third", "fourth", "fifth",
        "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three french hens",
        "four calling birds",
        "FIVE GOLD RINGS",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];
    for i in 0..12 {
        println!("On the {} day of Christmas,", days[i]);
        println!("My true love sent to me");
        for j in (0..=i) {
            println!("{}", gifts[j]);
        }
        println!();
    }
}