use std::io;

fn main() {
    // 섀도잉을 사용하여 문자열 변수를 정수 변수로 변경
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Using shadowing, spaces: {}", spaces); // 출력: Using shadowing, spaces: 3

    // mut을 사용하여 동일한 변수에 재할당하면 컴파일 타임 에러 발생
    let mut spaces = "   ";
    // spaces = spaces.len(); // 주석 처리하여 에러를 확인할 수 있도록 함
    println!("Using mut, spaces: {}", spaces);
}

