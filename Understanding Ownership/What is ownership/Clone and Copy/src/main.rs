fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();    // Clone

    println!("s1 = {}, s2 = {}", s1, s2);

    let mut x = 5;
    let y = x;                // Copy
    x=9;
    println!("x = {}, y = {}", x, y);
}
// python 에서는 불변객체 할당하면 원래 값 버린다.
// 불변객체를 가진변수를 변수에 할당하면 