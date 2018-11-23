#[cfg(any(unix, windows))]
extern crate dirs;

#[cfg(any(unix, windows))]
#[macro_use] extern crate gumdrop;

#[cfg(any(unix, windows))]
extern crate ketos;

#[cfg(any(unix, windows))]
extern crate linefeed;


#[cfg(any(unix, windows))]
mod repl;

#[cfg(any(unix, windows))]
use repl::run;

#[cfg(any(unix, windows))]
fn main() {
    let status = run();
    std::process::exit(status);
}

#[cfg(not(any(unix, windows)))]
fn main() {
}
