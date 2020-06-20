pub fn is_armstrong_number(num: u32) -> bool {
    let num_string = num.to_string();
    let num_length = num_string.len() as u32;
    // better to split into multiple iterators
    let sum_squared: u32 = num_string
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .map(|x| x.pow(num_length))
        .sum();
    //    let sum_squared = num_string
    //        .chars()
    //        .fold(0, |acc, x| acc + x.to_digit(10).unwrap().pow(num_length));
    sum_squared == num
}
