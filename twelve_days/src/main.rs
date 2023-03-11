

fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts = ["a partridge in a pear tree", "two turtle doves", "three french hens", "four calling birds", "five golden rings", "six geese a-laying", "seven swans a-swimming", "eight maids a-milking", "nine ladies dancing", "ten lords a-leaping", "eleven pipers piping", "twelve drummers drumming"];
    let mut day = 0;
    while day < 12 {
        println!("On the {} day of Christmas my true love gave to me", days[day]);

        for gift in (0..day+1).rev(){
            if day > 0 && gift == 0 {
                println!("And {}", gifts[gift]);
                continue;
            } else {
                println!("{}", gifts[gift]);
            }
        }
        println!("");
        day += 1;
    }
}
