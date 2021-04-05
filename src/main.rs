use std::io::Read;
fn main() {
    let input = std::io::stdin()
        .bytes()
        .next().unwrap();
        println!("{:?}", input);
}
