use std::{collections::HashMap, iter::Enumerate};

pub fn split_array(numbers: Vec<u32>, num1: usize, num2: usize) -> Vec<u32> {
    // numbers[num1..num2];
    // println!("{:?}", numbers[num1..num2]);
    // numbers[0..num1].to_vec()

    // slicing할 경우 벡터가 아니라 [] 리스트와같은 자료형으로 변함
    numbers[num1..num2 + 1].to_vec()
}

pub fn space_age(age: u32) -> String {
    // PROGRAMMERS-962
    let age_str = "abcdefghij";

    let age_vector = age_str.chars().collect::<Vec<char>>();

    let age_char = age.to_string().chars().collect::<Vec<char>>();

    let mut result: Vec<char> = vec![];
    for c in age_char {
        let idx: usize = c.to_string().parse().unwrap();
        result.push(age_vector[idx]);
    }

    result.iter().collect::<String>()
}

pub fn ordering_patient(emergency: Vec<u32>) -> Vec<u32> {
    let mut indices = (0..emergency.len()).collect::<Vec<_>>();
    indices.sort_by_key(|&i| &emergency[i]);

    let mut result: Vec<u32> = vec![];
    for idx in indices.iter() {
        // 순위로 바꿈
        result.push(emergency.len() as u32 - *idx as u32);
    }

    result
}

pub fn get_pairs(n: u32) -> u32 {
    let mut count = 0;
    for num1 in 1..n + 1 {
        // 만약 나누어 떨어지지 않으면 만들 수 없는 값임 -> 넘어감
        if n % num1 > 0 {
            // 이게 시간초과를 많이 줄여줌 (파이썬 기준 1.152초 -> 0.302초)
            continue;
        }
        // n을 num으로 나눈 만큼만 돌면 됨 (이게 곧 break이랑 같은 의미인듯?)
        for num2 in 1..n / num1 + 1 {
            if num1 * num2 == n {
                count += 1;

                // n이 되면 그 이후는 계산 안해도 되니까 break 이거 자체는 크게 줄여주지 못하는 듯?
                // 파이썬 기준 0.4초 -> 0.3초
                break;
            }
        }
    }
    count
}

#[test]
fn practice_1() {
    assert_eq!(vec![2, 3, 4], split_array(vec![1, 2, 3, 4, 5], 1, 3));
    assert_eq!(vec![3, 5], split_array(vec![1, 3, 5], 1, 2));
}

#[test]
fn practice_2() {
    assert_eq!("cd", space_age(23));
    assert_eq!("fb", space_age(51));
    assert_eq!("baa", space_age(100));
}

#[test]
fn practice_3() {
    assert_eq!(vec![3, 1, 2], ordering_patient(vec![3, 76, 24]));
    assert_eq!(
        vec![7, 6, 5, 4, 3, 2, 1],
        ordering_patient(vec![1, 2, 3, 4, 5, 6, 7])
    );
    assert_eq!(
        vec![2, 4, 3, 5, 1],
        ordering_patient(vec![30, 10, 23, 6, 100])
    );
}

#[test]
fn practice_4() {
    assert_eq!(49, get_pairs(1000000));
    assert_eq!(49, get_pairs(1000000));
}
