mod cli;
mod client;
use cli::*;
fn main()
{
    let _ = Cli::new().start();
}
