use ordinal::Ordinal;

pub fn main() {
    let gifts: [&str; 12] = [
        "twelve drummers drumming",
        "eleven pipers piping",
        "ten lords a-leaping",
        "nine ladies dancing",
        "eight maids a-milking",
        "seven swans -swimming",
        "six geese a-laying",
        "five golden rings",
        "four calling birds",
        "three french hens",
        "two turtle doves",
        "a partridge in a pear tree",
    ];

    for i in 1..13 {
        println!("");
        println!("[Verse {}]", i);
        println!(
            "On the {} day of christmas my true love sent to me",
            Ordinal(i).to_string()
        );

        for j in 12 - i..12 {
            if (i > 1) && (j == 10) {
                println!("{}, and", gifts[j]);
            } else {
                println!("{}", gifts[j]);
            }
        }
    }
}
