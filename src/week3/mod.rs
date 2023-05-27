use std::{collections::HashMap, vec};

// 나눗셈 division
// 몫은 quotient / 나머지는 remainder
pub fn get_remainder(num1: u8, num2: u8) -> u8 {
    num1 % num2
}

pub fn get_middle_number(list: &mut Vec<i16>) -> i16 {
    list.sort();
    let mid_idx = list.len() / 2;

    list[mid_idx]
}

pub fn get_most_frequenct_num(list: &mut Vec<u16>) -> i32 {
    let mut counter: HashMap<u16, usize> = HashMap::new();
    let mut max_count = 0;
    let mut max_keys: Vec<u16> = vec![];
    for num in list.iter() {
        // 키가 존재하는지 체크, 없으면 default값 0 넣기, 그것에 값에 1 더하기
        *counter.entry(*num).or_insert(0) += 1;
        // if counter.contains_key(num) {
        //     // https://stackoverflow.com/questions/30414424/how-can-i-update-a-value-in-a-mutable-hashmap
        //     *counter.get_mut(num).unwrap() += 1;
        // } else {
        //     counter.insert(*num, 1);
        // };

        if counter[num] > max_count {
            max_count = counter[num];
            max_keys.clear();
            max_keys.push(*num);
        } else if counter[num] == max_count {
            max_keys.push(*num);
        }
    }

    if max_keys.len() > 1 {
        -1
    } else {
        max_keys.pop().unwrap() as i32
    }
}

pub fn hate_even(num: u16) -> Vec<u16> {
    let mut result: Vec<u16> = vec![];
    for i in 1..num + 1 {
        if i % 2 == 1 {
            result.push(i);
        }
    }
    result
}

#[test]
fn practice_1() {
    assert_eq!(1, get_remainder(3, 2));
    assert_eq!(0, get_remainder(10, 5));
}

#[test]
fn practice_2() {
    let mut vec1 = vec![1, 2, 7, 10, 11];
    let mut vec2 = vec![9, -1, 0];
    assert_eq!(7, get_middle_number(&mut vec1));
    assert_eq!(0, get_middle_number(&mut vec2));
}

#[test]
fn practice_3() {
    let mut vec1 = vec![1, 2, 3, 3, 3, 4];
    let mut vec2 = vec![1, 1, 2, 2];
    let mut vec3 = vec![1];
    assert_eq!(3, get_most_frequenct_num(&mut vec1));
    assert_eq!(-1, get_most_frequenct_num(&mut vec2));
    assert_eq!(1, get_most_frequenct_num(&mut vec3));
}

#[test]
fn practice_4() {
    assert_eq!(vec![1, 3, 5, 7, 9], hate_even(10));
    assert_eq!(vec![1, 3, 5, 7, 9, 11, 13, 15], hate_even(15));
}
