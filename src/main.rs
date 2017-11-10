extern crate dumplingh;

use std::io::stderr;
use std::process::exit;
use self::dumplingh::{Options, Error};


fn main() {
    let result = actual_main();
    exit(result);
}

fn actual_main() -> i32 {
    if let Err(err) = result_main() {
        err.print_error(&mut stderr());
        err.exit_value()
    } else {
        0
    }
}

fn result_main() -> Result<(), Error> {
    let opts = Options::parse();
    println!("{:#?}", opts);

    Ok(())
}
