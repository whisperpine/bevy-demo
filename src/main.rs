//! A game demo built with Bevy.

#![cfg_attr(debug_assertions, allow(unused))]
#![cfg_attr(
    not(debug_assertions),
    deny(warnings, missing_docs),
    deny(clippy::todo, clippy::unwrap_used)
)]
#![cfg_attr(
    not(any(test, debug_assertions)),
    deny(clippy::print_stdout, clippy::dbg_macro)
)]

fn main() {
    println!("\n#### main ####\n");
}
