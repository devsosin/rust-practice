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

pub fn div2(num1: u16, num2: u16) -> u32 {
    // 정수형 나눗셈의 경우 정수로만 결과를 반환하기 때문에 f64로 변환
    let num1 = num1 as f64;
    let num2 = num2 as f64;
    let result = num1 / num2 * 1000 as f64;

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

pub fn fraction_add(numer1: u32, denom1: u32, numer2: u32, denom2: u32) -> Vec<u32> {
    // 최소공배수 구하기
    // 1. 각 숫자 구성 찾기 2 -> [2], 4 -> [2, 2]
    // 2. 겹치는 것들 가져오기 -> 분모
    // 3. 서로 겹치지 않는 부분 따로 관리하기

    // 각각에 곱해지는 값을 분자에 곱해서 더하기

    // 두 값의 최대공약수 구해서 나누기 (분자, 분모)
    // 1. 2부터 둘 중 작은 값까지 공통적으로 나눠지는 값 찾아서 더이상 나눠떨어지지 않을 때까지 나누기
    // 값 반환

    let result: Vec<u32> = Vec::new();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn practice_1_1() {
        assert_eq!(5, add(2, 3));
        assert_eq!(102, add(100, 2));
    }

    #[test]
    fn practice_1_2() {
        assert_eq!(-1, sub(2, 3));
        assert_eq!(98, sub(100, 2));
    }

    #[test]
    fn practice_1_3() {
        assert_eq!(12, mul(3, 4));
        assert_eq!(513, mul(27, 19));
    }

    #[test]
    fn practice_1_4() {
        assert_eq!(2, div(10, 5));
        assert_eq!(3, div(7, 2));
    }

    #[test]
    fn practice_2_1() {
        assert_eq!(1500, div2(3, 2));
        assert_eq!(2333, div2(7, 3));
        assert_eq!(62, div2(1, 16));
    }

    #[test]
    fn practice_2_2() {
        assert_eq!(-1, is_eqaul(2, 3));
        assert_eq!(1, is_eqaul(11, 11));
        assert_eq!(-1, is_eqaul(7, 99));
    }

    #[test]
    fn practice_2_3() {
        assert_eq!(vec![5, 4], fraction_add(1, 2, 3, 4));
        assert_eq!(vec![29, 6], fraction_add(9, 2, 1, 3));
    }
}
