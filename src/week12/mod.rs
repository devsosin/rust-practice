use std::collections::HashSet;

pub fn remove_vowel(my_string: &str) -> String {
    let mut result: Vec<char> = vec![];
    for s in my_string.chars() {
        if !['a', 'e', 'i', 'o', 'u'].contains(&s) {
            result.push(s);
        }
    }

    result.iter().collect::<String>()
}

pub fn sort_string(my_string: &str) -> Vec<u32> {
    let mut result: Vec<u32> = vec![];
    for s in my_string.chars() {
        // 0 <= s <= 9 방식은 불가능
        if '0' <= s && s <= '9' {
            result.push(s.to_string().parse().unwrap());
        }
        // match s.to_string().parse() {
        //     Ok(i) => result.push(i),
        //     Err(_) => continue,
        // };
    }

    result.sort();
    result
}

pub fn get_sum_of_string(my_string: &str) -> u32 {
    let mut result: u32 = 0;
    for s in my_string.chars() {
        if '0' <= s && s <= '9' {
            result += s.to_string().parse::<u32>().unwrap();
        }
    }
    result

    // let mut result: u32 = 0;

    // for r in sort_string(my_string) {
    //     result += r;
    // }

    // result
}

pub fn find_divisor(n: u32) -> Vec<u32> {
    // set 자료형
    let mut result: HashSet<u32> = HashSet::new();
    let mut rest = n;
    for i in 2..n + 1 {
        if rest < i {
            break;
        }
        loop {
            if rest % i == 0 {
                rest /= i;
                result.insert(i);
            } else {
                break;
            }
        }
    }

    // set를 다시 vec자료형으로
    let mut result2: Vec<u32> = vec![];
    for n in result.iter() {
        result2.push(*n)
    }
    result2.sort();
    result2
}

#[test]
fn practice_1() {
    assert_eq!("bs".to_string(), remove_vowel("bus"));
    assert_eq!("nc t mt y".to_string(), remove_vowel("nice to meet you"));
}

#[test]
fn practice_2() {
    assert_eq!(vec![1, 2, 2, 3, 9], sort_string("hi12392"));
    assert_eq!(vec![2, 2, 4, 8], sort_string("p2o4i8gj2"));
    assert_eq!(vec![0], sort_string("abcde0"));
}

#[test]
fn practice_3() {
    assert_eq!(10, get_sum_of_string("aAb1B2cC34oOp"));
    assert_eq!(16, get_sum_of_string("1a2b3c4d123"));
}

#[test]
fn practice_4() {
    assert_eq!(vec![2, 3], find_divisor(12));
    assert_eq!(vec![17], find_divisor(17));
    assert_eq!(vec![2, 3, 5, 7], find_divisor(420));
}
