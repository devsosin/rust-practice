## Rust와 친해지기

1. [Week 1 (사칙연산)](src/week1/)
    - +, -, *, /

2. Week 2
    - 두 수의 나눗셈 (형변환)
    - 숫자 비교하기
    - 분수의 덧셈
    - 배열 두 배 만들기

### Sosin Note
1. 숫자 비교 시 참조자(&num)로 반복문 돌리면 값을 가져올 수 있도록 역참조derefer(*num) 해야함
2. 벡터 indexing할 때 index 자료형은 usize로 하는 것이 편함
3. **string을 단일 문자로** let chars: Vec<char> = my_string.chars().collect::<Vec<_>>();
4. **숫자 -> 각 자릿수 단일 문자로** let age_char = age.to_string().chars().collect::<Vec<char>>();
5. **단일 문자 -> 숫자로** c.to_string().parse().unwrap(); (usize)
6. **range를 벡터로** let mut vec: Vec<u32> = (0..10).collect();
7. vec slicing하면 array가 나옴 (to_vec으로 벡터화)
