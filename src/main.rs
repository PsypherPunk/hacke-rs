use argh::FromArgs;
use hacke_rs::{ScriptLine, HACKERS};
use rand::rng;
use rand::seq::IndexedRandom;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(FromArgs)]
/// Show a line from the Hackers script.
struct Args {
    #[argh(switch, short = 'v')]
    /// display the current version
    version: bool,
}

fn main() {
    let args: Args = argh::from_env();
    if args.version {
        println!("{VERSION}");
    } else {
        let choice: &ScriptLine = crate::HACKERS
            .choose(&mut rng())
            .expect("Oh man. That's universally stupid, man!");

        println!(
            "\x1b[1m{}\x1b[0m\n\x1b[3m{}\x1b[0m",
            choice.character, choice.line
        );
    }
}
