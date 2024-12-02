use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(
    author,
    about,  //="The main crate in the workspace",
    long_about = None
)]
/// List of possible command line arguments
pub struct Args {}

pub fn process_args() {
    Args::parse();
}
