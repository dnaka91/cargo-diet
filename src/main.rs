#[macro_use]
extern crate structopt;

use structopt::StructOpt;

mod options {
    #[derive(Debug, StructOpt)]
    #[structopt(bin_name = "cargo")]
    pub enum Command {
        /// Add an include directive minimizing bandwidth usage when downloading from crates.io
        #[structopt(name = "diet")]
        #[structopt(
            after_help = "This command allows you to add an include directive to minimize the package size of your crate."
        )]
        Diet(Args),
    }

    #[derive(Debug, StructOpt)]
    pub struct Args {}
}

fn main() -> anyhow::Result<()> {
    let options::Command::Diet(_args) = options::Command::from_args();
    cargo_diet::fun()?;
    Ok(())
}
