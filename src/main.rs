use std::process;

fn main() {
    let res = mdb_cp::run();

    if let Err(ref err) = res {
        eprintln!("<Error> {}", err);
        process::exit(1);
    }
}
