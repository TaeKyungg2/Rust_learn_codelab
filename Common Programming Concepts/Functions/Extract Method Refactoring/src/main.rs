fn main() {
    let year1993 = 1993;
    year(year1993);

    let year2021 = 2021;
    year(year2021);
}

fn year(year1993: i32) {
    println!("{}: ten years ago was {}", year1993, year1993 - 10);
}