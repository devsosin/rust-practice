pub fn pizza_split_1(num: u16) -> u16 {
    if num % 7 > 0 {
        num / 7 + 1
    } else {
        num / 7
    }
}

pub fn pizza_split_2(num: u16) -> u16 {
    // 6조각
    // num 사람이 모두 같은 수 6과 num의 공배수
    // let mut max_pizza = 0;
    for i in 6..num * 6 + 1 {
        if i % num == 0 && i % 6 == 0 {
            return i / 6;
            // max_pizza = i;
            // break;
        }
    }

    // 6조각에 한 판
    // max_pizza / 6
    1
}

pub fn pizza_split_3(slice: u8, n: u8) -> u8 {
    if n % slice > 0 {
        n / slice + 1
    } else {
        n / slice
    }
}

pub fn avg(numbers: Vec<u32>) -> f64 {
    let mut sum_value: f64 = 0.0;

    for num in numbers.iter() {
        sum_value += *num as f64
    }

    sum_value / numbers.len() as f64
}

#[test]
fn practice_1() {
    assert_eq!(1, pizza_split_1(7));
    assert_eq!(1, pizza_split_1(1));
    assert_eq!(3, pizza_split_1(15));
}

#[test]
fn practice_2() {
    assert_eq!(1, pizza_split_2(6));
    assert_eq!(5, pizza_split_2(10));
    assert_eq!(2, pizza_split_2(4));
}

#[test]
fn practice_3() {
    assert_eq!(2, pizza_split_3(7, 10));
    assert_eq!(3, pizza_split_3(4, 12));
}

#[test]
fn practice_4() {
    assert_eq!(5.5, avg(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
    assert_eq!(94.0, avg(vec![89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99]));
}
