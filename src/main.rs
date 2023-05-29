use std::any::type_name;

use practice::week8::get_pairs;

// use practice::week6::print_right_triangle;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
use std::time::{Duration, Instant};

fn main() {
    // // 정수형끼리 나누면 정수 반환
    // println!("{}", 5 / 3); // 1
    // println!("{}", type_of(5 / 3)); // i32

    // // 실수형끼리 나누면 실수 반환
    // println!("{}", 10.0 / 5.0); // 2
    // println!("{}", type_of(10.0 / 5.0)); // f64

    // for i in 2..4 {
    //     // 나누어 떨어지면,
    //     if 4 % i == 0 {
    //         // rest, numbers
    //         println!("{}", 4 / i);
    //     }
    // }

    // fraction_add(1, 2, 3, 4);
    // print_right_triangle(3);

    // split_array(vec![1, 2, 3, 4, 5], 1, 3)

    // 시간 체크
    // 100만개 쌍을 구하는 데 0.103초, 0.090초 (파이썬 - 0.468초, 0.527초) 단순계산 rust가 약 5배 정도 빠른듯?
    let start = Instant::now();
    println!("{:?}", get_pairs(1000000));
    println!("{:?}", get_pairs(1000000));
    println!("{:?}", get_pairs(1000000));
    println!("{:?}", get_pairs(1000000));
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
