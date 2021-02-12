use std::path::Path;
use logger;


fn main() {
    let does_exist = Path::new("./.glitterrc").exists();
    if does_exist == false {

    }
}
