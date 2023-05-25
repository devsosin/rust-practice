use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    // 정수형끼리 나누면 정수 반환
    println!("{}", 5 / 3); // 1
    println!("{}", type_of(5 / 3)); // i32

    // 실수형끼리 나누면 실수 반환
    println!("{}", 10.0 / 5.0); // 2
    println!("{}", type_of(10.0 / 5.0)); // f64

    for i in 2..4 {
        // 나누어 떨어지면,
        if 4 % i == 0 {
            // rest, numbers
            println!("{}", 4 / i);
        }
    }
}
