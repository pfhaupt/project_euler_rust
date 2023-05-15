fn main() {
    let mut record = 0;
    for n in 100..1000 {
        for m in 100..1000 {
            let product = n * m;
            let mut palindrome = 0;
            let mut tmp_product = product;
            while tmp_product != 0 {
                let digit = tmp_product % 10;
                palindrome = palindrome * 10 + digit;
                tmp_product /= 10;
            }
            if palindrome == product {
                if palindrome > record {
                    record = palindrome;
                }
            }
        }
    }
    println!("{}", record);
}
