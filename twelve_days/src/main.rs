

fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts = ["a partridge in a pear tree", "two turtle doves", "three french hens", "four calling birds", "five golden rings", "six geese a-laying", "seven swans a-swimming", "eight maids a-milking", "nine ladies dancing", "ten lords a-leaping", "eleven pipers piping", "twelve drummers drumming"];
    // let mut day = 0;
    for (day_num, day) in days.iter().enumerate() {
        println!("On the {} day of Christmas my true love gave to me", day);

        for gift in (0..day_num+1).rev(){
            if day_num > 0 && gift == 0 {
                println!("And {}", gifts[gift]);
                continue;
            } else {
                println!("{}", gifts[gift]);
            }
        }
    }
}
