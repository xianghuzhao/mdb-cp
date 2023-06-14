use std::process;

use log::error;

fn main() {
    env_logger::init();

    let res = mdb_cp::run();

    if let Err(ref err) = res {
        error!("<Error> {}", err);
        process::exit(1);
    }
}
