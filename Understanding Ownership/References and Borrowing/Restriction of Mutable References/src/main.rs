fn main() {
    let mut s = String::from("hello");
    //
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM
    r3.push('v');
    println!("{}, {}, and {}", r3,s, 1);
}
