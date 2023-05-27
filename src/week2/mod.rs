pub fn div2(num1: u16, num2: u16) -> u32 {
    // 정수형 나눗셈의 경우 정수로만 결과를 반환하기 때문에 f64로 변환
    let num1 = num1 as f64;
    let num2 = num2 as f64;
    let result = num1 / num2 * 1000.0;

    // 계산 결과를 정수형으로 다시 변환 (정수부분만 가져오기)
    result as u32
}

pub fn is_eqaul(num1: u32, num2: u32) -> i8 {
    if num1 == num2 {
        1
    } else {
        -1
    }
}

// 공약수
fn get_divisor(num: &u32) -> Vec<u32> {
    let mut divisor_list: Vec<u32> = Vec::new();
    let mut num: u32 = *num;
    for i in 2..num + 1 {
        loop {
            if num % i == 0 {
                divisor_list.push(i);
                num /= i
            } else {
                break;
            }

            if num == 0 {
                break;
            }
        }
    }
    divisor_list
}

// 공배수 찾기
fn get_common_multiple(divisor1: &Vec<u32>, divisor2: &Vec<u32>) -> Vec<u32> {
    // 겹치면 1개만
    let mut j: usize = 0;
    let mut common_multiple: Vec<u32> = Vec::new();
    for num1 in divisor1.iter() {
        while divisor2[j] < *num1 {
            common_multiple.push(divisor2[j]);
            j += 1;
        }
        common_multiple.push(*num1);
        if num1 == &divisor2[j] {
            j += 1;
        }
    }

    while j < divisor2.len() {
        common_multiple.push(divisor2[j]);
        j += 1;
    }

    common_multiple
}

// 안 겹치는 요소 찾기
fn get_multiple_num(divisor: &Vec<u32>, common_multiple: &Vec<u32>) -> u32 {
    let mut j: usize = 0;
    let mut result = 1;

    for num in divisor.iter() {
        while common_multiple[j] < *num {
            result *= common_multiple[j];
            j += 1;
        }
        if common_multiple[j] == *num {
            j += 1;
        }
    }

    while j < common_multiple.len() {
        result *= common_multiple[j];
        j += 1;
    }

    result
}

// 최대공약수 찾기
fn get_divisor_num(divisor1: &Vec<u32>, divisor2: &Vec<u32>) -> u32 {
    let mut common_divisor: u32 = 1;
    let mut j: usize = 0;
    for num in divisor1.iter() {
        while divisor2[j] < *num {
            j += 1;
            if j == divisor2.len() {
                return common_divisor;
            }
        }
        if divisor2[j] == *num {
            common_divisor *= num;
            j += 1;
        }
    }

    common_divisor
}

pub fn fraction_add1(numer1: u32, denom1: u32, numer2: u32, denom2: u32) -> Vec<u32> {
    // 풀이 1.
    // 최소공배수 구하기
    // 1. 약수 찾기
    let divisor1 = get_divisor(&denom1);
    let divisor2 = get_divisor(&denom2);

    // 2. 공배수 찾기
    let cm = get_common_multiple(&divisor1, &divisor2);

    // 3. 자기자신과 공약수 비교하여 곱할 값 찾기
    let mn1: u32 = get_multiple_num(&divisor1, &cm);
    let mn2: u32 = get_multiple_num(&divisor2, &cm);

    // 분자에 곱해서 더하기
    let numer = numer1 * mn1 + numer2 * mn2;

    let head: Vec<u32> = get_divisor(&numer);
    let body_number = denom1 * mn1;
    let body = get_divisor(&body_number);

    // 최대공약수 구하기
    let dn: u32 = get_divisor_num(&head, &body);

    vec![numer / dn, body_number / dn]
}

pub fn fraction_add2(numer1: u32, denom1: u32, numer2: u32, denom2: u32) -> Vec<u32> {
    // 풀이 2.
    // 분자에 서로의 분모 곱해서 더하기
    let head = numer1 * denom2 + numer2 * denom1;
    let body = denom1 * denom2;

    // 둘 중 작은 값
    let min_value = if head > body { &body } else { &head };

    // 최대공약수 구하기
    let mut max_common_divisor = 1;
    for i in 2..*min_value {
        if head % i == 0 && body % i == 0 {
            max_common_divisor = i;
        }
    }

    vec![head / max_common_divisor, body / max_common_divisor]
}

pub fn multiply_list(list: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    for num in list.iter() {
        result.push(num * 2)
    }

    result
}

#[test]
fn practice_1() {
    assert_eq!(1500, div2(3, 2));
    assert_eq!(2333, div2(7, 3));
    assert_eq!(62, div2(1, 16));
}

#[test]
fn practice_2() {
    assert_eq!(-1, is_eqaul(2, 3));
    assert_eq!(1, is_eqaul(11, 11));
    assert_eq!(-1, is_eqaul(7, 99));
}

#[test]
fn practice_3() {
    assert_eq!(vec![5, 4], fraction_add2(1, 2, 3, 4));
    assert_eq!(vec![29, 6], fraction_add2(9, 2, 1, 3));
}

#[test]
fn practice_4() {
    assert_eq!(vec![2, 4, 6, 8, 10], multiply_list(vec![1, 2, 3, 4, 5]));
    assert_eq!(
        vec![2, 4, 200, -198, 2, 4, 6],
        multiply_list(vec![1, 2, 100, -99, 1, 2, 3])
    );
}
