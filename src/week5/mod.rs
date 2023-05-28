pub fn discount_clothes(price: u32) -> u32 {
    if price >= 500000 {
        return price * 80 / 100;
    } else if price >= 300000 {
        return price * 90 / 100;
    } else if price >= 100000 {
        return price * 95 / 100;
    }
    price
}

pub fn ice_americano(money: u32) -> Vec<u32> {
    vec![money / 5500, money % 5500]
}

pub fn print_age(age: u16) -> u16 {
    2023 - age
}

pub fn reverse_array(num_list: Vec<u32>) -> Vec<u32> {
    let mut number_list = num_list;
    number_list.reverse();
    // mem::swap (&, &)

    number_list
}

#[test]
fn practice_1() {
    assert_eq!(142500, discount_clothes(150000));
    assert_eq!(464000, discount_clothes(580000));
}

#[test]
fn practice_2() {
    assert_eq!(vec![1, 0], ice_americano(5500));
    assert_eq!(vec![2, 4000], ice_americano(15000));
}

#[test]
fn practice_3() {
    assert_eq!(1983, print_age(40));
    assert_eq!(2000, print_age(23));
}

#[test]
fn practice_4() {
    assert_eq!(vec![5, 4, 3, 2, 1], reverse_array(vec![1, 2, 3, 4, 5]));
    assert_eq!(
        vec![2, 1, 1, 1, 1, 1],
        reverse_array(vec![1, 1, 1, 1, 1, 2])
    );
    assert_eq!(
        vec![5, 3, 1, 1, 1, 0, 1],
        reverse_array(vec![1, 0, 1, 1, 1, 3, 5])
    );
}
