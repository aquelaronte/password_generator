use std::env;

mod flags;
mod gen;
mod io;

pub fn init() {
    let args: Vec<String> = env::args().collect();

    if args.len() - 1 == 0 {
        io::main()
    } else {
        flags::main()
    }
}
