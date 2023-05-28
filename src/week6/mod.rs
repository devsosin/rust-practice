pub fn reverse_string(my_string: String) -> String {
    let string_ref: &str = &my_string;
    let mut chars: Vec<char> = string_ref.chars().collect::<Vec<_>>();
    chars.reverse();

    let rev_string: String = chars.iter().collect::<String>();
    rev_string
}

pub fn print_right_triangle(n: u8) -> u8 {
    for i in 0..n {
        for _ in 0..i + 1 {
            print!("*")
        }
        println!()
    }

    1
}

pub fn count_odd_even(num_list: Vec<u32>) -> Vec<u32> {
    let mut result: Vec<u32> = vec![0, 0];

    for num in num_list.iter() {
        if num % 2 == 1 {
            result[1] += 1;
        } else {
            result[0] += 1;
        }
    }

    result
}

pub fn repeat_string(my_string: &str, n: u8) -> String {
    let mut repeated_string: Vec<char> = vec![];
    for s in my_string.chars().into_iter() {
        for _ in 0..n {
            repeated_string.push(s)
        }
    }

    repeated_string.iter().collect::<String>()
}

#[test]
fn practice_1() {
    assert_eq!("noraj", reverse_string("jaron".to_string()));
    assert_eq!("daerb", reverse_string("bread".to_string()));
}

#[test]
fn practice_2() {
    print_right_triangle(3);
    // assert_eq!(-1, print_right_triangle(2, 3));
    // assert_eq!(98, print_right_triangle(100, 2));
}

#[test]
fn practice_3() {
    assert_eq!(vec![2, 3], count_odd_even(vec![1, 2, 3, 4, 5]));
    assert_eq!(vec![0, 4], count_odd_even(vec![1, 3, 5, 7]));
}

#[test]
fn practice_4() {
    assert_eq!("hhheeellllllooo", repeat_string("hello", 3));
}
