pub fn trim_me(input: &str) -> String {

    input.trim().to_string()
}

pub fn compose_me(input: &str) -> String {
    input.to_string()+" world!"
}

pub fn replace_me(input: &str) -> String {
    /* TODO: Replace "cars" in the string with "balloons"! */
    return input.replace("cars","balloons").to_string()
}
fn main(){
    let x=String::from("hdddd");
    println!("hello world");
}