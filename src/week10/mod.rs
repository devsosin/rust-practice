pub fn get_position(pos: Vec<i32>) -> u8 {
    if pos[0] > 0 && pos[1] > 0 {
        return 1;
    } else if pos[0] > 0 && pos[1] < 0 {
        return 4;
    } else if pos[0] < 0 && pos[1] < 0 {
        return 3;
    } else {
        return 2;
    }
}

pub fn make_n_dimension(num_list: Vec<i32>, n: usize) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    for idx in (0..num_list.len() / n).collect::<Vec<usize>>() {
        result.push(num_list[n * idx..n * (idx + 1)].to_vec());
    }

    result
}

pub fn throw_ball(numbers: Vec<u32>, k: u32) -> u32 {
    let mut result = 0;
    let length = numbers.len();
    for _ in 0..k - 1 {
        result += 2;
        if result >= length {
            result -= length;
        }
    }

    numbers[result]
}

pub fn rotate_array(numbers: Vec<i32>, direction: &str) -> Vec<i32> {
    // 한칸만 이동 (left or right)
    let mut result: Vec<i32> = Vec::new();
    if direction == "left" {
        // python list extend 느낌
        result.append(&mut numbers[1..numbers.len()].to_vec());
        result.push(numbers[0]);
    } else {
        result.push(numbers[numbers.len() - 1]);
        result.append(&mut numbers[0..numbers.len() - 1].to_vec());
    }

    result
}

#[test]
fn practice_1() {
    assert_eq!(1, get_position(vec![2, 4]));
    assert_eq!(2, get_position(vec![-7, 9]));
}

#[test]
fn practice_2() {
    assert_eq!(
        vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]],
        make_n_dimension(vec![1, 2, 3, 4, 5, 6, 7, 8], 2)
    );
    // assert_eq!(98, make_n_dimension(100, 2));
}

#[test]
fn practice_3() {
    assert_eq!(3, throw_ball(vec![1, 2, 3, 4], 2));
    assert_eq!(3, throw_ball(vec![1, 2, 3, 4, 5, 6], 5));
    assert_eq!(2, throw_ball(vec![1, 2, 3], 3));
}

#[test]
fn practice_4() {
    assert_eq!(vec![3, 1, 2], rotate_array(vec![1, 2, 3], "right"));
    assert_eq!(
        vec![455, 6, 4, -1, 45, 6, 4],
        rotate_array(vec![4, 455, 6, 4, -1, 45, 6], "left")
    );
}
