pub fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

pub fn sub(num1: i32, num2: i32) -> i32 {
    num1 - num2
}

pub fn mul(num1: u32, num2: u32) -> u32 {
    num1 * num2
}

pub fn div(num1: u32, num2: u32) -> u32 {
    num1 / num2
}

#[test]
fn practice_1() {
    assert_eq!(5, add(2, 3));
    assert_eq!(102, add(100, 2));
}

#[test]
fn practice_2() {
    assert_eq!(-1, sub(2, 3));
    assert_eq!(98, sub(100, 2));
}

#[test]
fn practice_3() {
    assert_eq!(12, mul(3, 4));
    assert_eq!(513, mul(27, 19));
}

#[test]
fn practice_4() {
    assert_eq!(2, div(10, 5));
    assert_eq!(3, div(7, 2));
}
