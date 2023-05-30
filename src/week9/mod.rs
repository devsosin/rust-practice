use std::collections::HashMap;

pub fn ant_legion(hp: u32) -> u32 {
    let mut result = hp / 5;
    let rest = hp % 5;

    result += rest / 3 + rest % 3;
    result
}

pub fn morse_decrypt(letter: String) -> String {
    let morse = HashMap::from([
        (".-", "a"),
        ("-...", "b"),
        ("-.-.", "c"),
        ("-..", "d"),
        (".", "e"),
        ("..-.", "f"),
        ("--.", "g"),
        ("....", "h"),
        ("..", "i"),
        (".---", "j"),
        ("-.-", "k"),
        (".-..", "l"),
        ("--", "m"),
        ("-.", "n"),
        ("---", "o"),
        (".--.", "p"),
        ("--.-", "q"),
        (".-.", "r"),
        ("...", "s"),
        ("-", "t"),
        ("..-", "u"),
        ("...-", "v"),
        (".--", "w"),
        ("-..-", "x"),
        ("-.--", "y"),
        ("--..", "z"),
    ]);
    // string 이어붙이기 -> String.push_str(&str)
    let mut result: String = String::from("");
    for m in letter.split_whitespace() {
        result.push_str(&morse[m].chars().next().unwrap().to_string());
        // result.push(morse[m].chars().next().unwrap())
    }

    // result.iter().collect::<String>()
    result
}

pub fn rsp(rsp: String) -> String {
    // 가위 2, 바위 0, 보 5
    let rsp_hash = HashMap::from([('2', '0'), ('0', '5'), ('5', '2')]);

    let mut result: Vec<char> = Vec::new();
    for c in rsp.chars().collect::<Vec<char>>().iter() {
        result.push(rsp_hash[c])
    }
    result.iter().collect::<String>()
}

pub fn split_balls(balls: u32, share: u32) -> u32 {
    // 수열
    // 3C2 3 / 2 * 1
    let mut head: u32 = 1;
    for num in (share + 1..balls + 1).collect::<Vec<u32>>() {
        head *= num;
    }
    let mut body: u32 = 1;
    for num in (1..balls - share + 1).collect::<Vec<u32>>() {
        body *= num;
    }

    head / body
}

#[test]
fn practice_1() {
    assert_eq!(5, ant_legion(23));
    assert_eq!(6, ant_legion(24));
    assert_eq!(201, ant_legion(999));
}

#[test]
fn practice_2() {
    assert_eq!(
        "hello".to_string(),
        morse_decrypt(".... . .-.. .-.. ---".to_string())
    );
    assert_eq!(
        "python".to_string(),
        morse_decrypt(".--. -.-- - .... --- -.".to_string())
    );
}

#[test]
fn practice_3() {
    assert_eq!("0".to_string(), rsp("2".to_string()));
    assert_eq!("052".to_string(), rsp("205".to_string()));
}

#[test]
fn practice_4() {
    assert_eq!(3, split_balls(3, 2));
    assert_eq!(10, split_balls(5, 3));
}
