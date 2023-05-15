
const NUMBERS: [&str; 20] = [
    "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
    "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"
];
const TENS: [&str; 10] = [
    "", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"
];

fn main() {
    // https://projecteuler.net/problem=17
    const MAX: usize = 1000;
    let mut sum = 0;
    for n in 1..=MAX {
        let name = get_name(n);
        sum += name.len();
    }
    println!("{sum}");
}

fn get_name(n: usize) -> String {
    if n < NUMBERS.len() {
        return String::from(NUMBERS[n]);
    }
    if n == 1000 {
        return String::from("onethousand");
    }
    let hund = n / 100;
    let mut tens = (n / 10) % 10;
    let mut ones = n % 20;
    if tens >= 2 {
        ones = n % 10;
    } else {
        tens = 0;
    }
    let mut s100 = String::from(format!("{}hundred", NUMBERS[hund]));
    if tens != 0 || ones != 0 {
        s100 += "and";
    }
    if hund == 0 {
        s100 = String::from("");
    }
    let s10 = String::from(TENS[tens]);
    let s1 = String::from(NUMBERS[ones]);

    return s100 + &s10 + &s1;
}
