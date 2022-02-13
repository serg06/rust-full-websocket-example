pub use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(default_value_t = 3000)]
    pub port: u32,
}
