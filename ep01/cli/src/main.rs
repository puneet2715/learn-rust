fn main() {
    let number = 5;
    let number2 = add(number,number);
    println!("The number is {number2}");
}

pub fn add(int1: i32, int2: i32) -> i32 {
    return int1 + int2;
}

fn add2(int1: i32, int2: i32) -> i32 {
    int1 + int2
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let number = 5;
        let number2 = add(number,number);
        assert_eq!(number2, number + number);
        assert_ne!(number2, number - number);
    }
}