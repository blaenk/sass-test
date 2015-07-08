extern crate time;
extern crate sass_rs;

use time::PreciseTime;
use sass_rs::sass_context::SassFileContext;

fn main() {
    let input = "scss/screen.scss";

    let (libsass_duration, libsass_out) = {
        let start = PreciseTime::now();
        let mut file_context = SassFileContext::new(&input);

        let out = file_context.compile().unwrap();
        let end = PreciseTime::now();

        let dur = start.to(end);

        (dur, out)
    };

    let (sassc_duration, sassc_out) = {
        let start = PreciseTime::now();
        let mut command =
            ::std::process::Command::new("sassc");

        command.arg(&input);

        let out = command.output().unwrap();
        let out = String::from_utf8(out.stdout).unwrap();
        let end = PreciseTime::now();

        let dur = start.to(end);

        (dur, out)
    };

    println!("equal: {}", libsass_out == sassc_out);
    println!("libsass elapsed: {}", libsass_duration);
    println!("sassc   elapsed: {}", sassc_duration);
}
