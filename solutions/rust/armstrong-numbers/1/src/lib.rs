pub fn is_armstrong_number(num: u32) -> bool {
    let number_string = num.to_string();
    let digits: Vec<u32> = number_string
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let n = digits.len();
    let mut sum:u32 = 0;
    for a in digits{
        sum+={a.pow(n.try_into().unwrap())};
    }
    sum == num
}
