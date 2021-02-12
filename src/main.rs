mod get_and_parse;
mod logger;
fn main() {
    let config = get_and_parse::get_and_parse().unwrap();
    println!("{:?}", config);
}
