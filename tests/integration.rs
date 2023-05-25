extern crate practice;

#[test] // integration test는 파일 읽어와서 체크 -> 예제 입출력은 단위테스트에서 진행
fn practice1_1() {
    assert_eq!(5, practice::add(2, 3));
    assert_eq!(102, practice::add(100, 2));
}
