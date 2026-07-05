use lofty;
use std::env;
// here comes the fucking worst rust code ever


fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
