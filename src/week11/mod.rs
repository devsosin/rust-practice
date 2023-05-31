pub fn dice_count(dice_box: Vec<u32>, n: u32) -> u32 {
    (dice_box[0] / n) * (dice_box[1] / n) * (dice_box[2] / n)
}

fn count_divisor(n: u32) -> u32 {
    let mut result: u32 = 1;
    for i in 2..n + 1 {
        if n % i == 0 {
            result += 1;
        }
    }
    result
}

pub fn composite_number(n: u32) -> u32 {
    let mut result: u32 = 0;
    for i in 1..n + 1 {
        if count_divisor(i) > 2 {
            result += 1;
        }
    }

    result
}

pub fn make_max(numbers: Vec<u32>) -> u32 {
    let mut max_value: u32 = 0;
    for (i, num1) in numbers.iter().enumerate() {
        for (j, num2) in numbers.iter().enumerate() {
            if i != j {
                if max_value < num1 * num2 {
                    max_value = num1 * num2;
                }
            }
        }
    }
    max_value
}

pub fn find_max_factorial(n: u32) -> u32 {
    let mut result: u32 = 1;
    // n일 경우 반례 2, 2 발생
    for i in 2..n + 1 {
        if result * i > n {
            return i - 1;
        }
        result *= i;
    }
    1
}

#[test]
fn practice_1() {
    assert_eq!(1, dice_count(vec![1, 1, 1], 1));
    assert_eq!(12, dice_count(vec![10, 8, 6], 3));
}

#[test]
fn practice_2() {
    assert_eq!(5, composite_number(10));
    assert_eq!(8, composite_number(15));
}

#[test]
fn practice_3() {
    assert_eq!(20, make_max(vec![1, 2, 3, 4, 5]));
    assert_eq!(744, make_max(vec![0, 31, 24, 10, 1, 9]));
}

#[test]
fn practice_4() {
    assert_eq!(10, find_max_factorial(3628800));
    assert_eq!(3, find_max_factorial(7));
}
