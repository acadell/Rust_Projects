fn main() {
    let mystr = "microspectrophotometries";
    let sum = letter_sum(&mystr);
    println!("Sum: {}", sum);

    let sum = letter_sum_recursive(&mystr);
    println!("Sum Recursive: {}", &sum);
}

fn letter_sum(letters : &str) -> u32{
    let mut result = 0;
    for ch in letters.chars() {
        let ch_num = ch as u32 - 96;
        result = result + ch_num;
    }
    result
}

fn letter_sum_recursive(letters : &str) -> u32{
    let mut chars = letters.chars();
    let letter = chars.next();
    if letter.is_some() {
        (letter.unwrap() as u32 - 96) + letter_sum_recursive(chars.as_str())
    }
    else { 
        0 
    }
}