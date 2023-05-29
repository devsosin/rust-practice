pub fn remove_char(my_string: String, letter: String) -> String {
    let mut result: Vec<char> = vec![];
    let chars: Vec<char> = my_string.chars().collect::<Vec<_>>();

    let letter: char = letter.chars().collect::<Vec<_>>()[0];

    // string 으로 들어왔을 때 char로 변경하는 방법

    for c in chars.iter() {
        if *c != letter {
            result.push(*c);
        }
    }
    // let rev_string: String = chars.iter().collect::<String>();
    let result: String = result.iter().collect::<String>();
    result
}

pub fn check_angle(angle: u8) -> u8 {
    if angle < 90 {
        return 1;
    } else if angle == 90 {
        return 2;
    } else if angle < 180 {
        return 3;
    } else {
        return 4;
    }
}

pub fn lamb_skewers(n: u32, k: u32) -> u32 {
    12000 * n + (k - (n / 10)) * 2000
}

pub fn sum_even(n: u32) -> u32 {
    // let mut result = 0;
    // for num in 1..n + 1 {
    //     if num % 2 == 0 {
    //         result += num;
    //     }
    // }
    (n / 2) * (n / 2 + 1)
}

#[test]
fn practice_1() {
    // char 는 작은따옴표, String은 큰따옴표
    assert_eq!("abcde", remove_char("abcdef".to_string(), "f".to_string()));
    assert_eq!("Cdbe", remove_char("BCBdbe".to_string(), "B".to_string()));
}

#[test]
fn practice_2() {
    assert_eq!(1, check_angle(70));
    assert_eq!(3, check_angle(91));
    assert_eq!(4, check_angle(180));
}

#[test]
fn practice_3() {
    assert_eq!(124000, lamb_skewers(10, 3));
    assert_eq!(768000, lamb_skewers(64, 6));
}

#[test]
fn practice_4() {
    assert_eq!(30, sum_even(10));
    assert_eq!(6, sum_even(4));
}
