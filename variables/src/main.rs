fn main() {
    // let (디폴트 불변) 은 문자열->숫자 변형이 가능하고
    let spaces = "   ";
    let spaces = spaces.len();

    println!("1st spaces: {}", spaces);

    // let mut 은 문자열->숫자 변형이 불가
    let mut spaces2 = "   ";
    let spaces2 = spaces2.len();

    println!("2nd spaces: {}", spaces2);
}