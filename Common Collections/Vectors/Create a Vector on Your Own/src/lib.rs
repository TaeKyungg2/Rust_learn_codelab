pub fn create_array() -> [i32; 4] {
    let a = [10, 20, 30, 40]; // a plain array
    a
}

pub fn create_vector() -> Vec<i32> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    v.push(40);
    v
}

