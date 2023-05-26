pub mod week1;
pub mod week2;

pub fn my_func() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_test() {
        assert_eq!(true, my_func());
    }
}
