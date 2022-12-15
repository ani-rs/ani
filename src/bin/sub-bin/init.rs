extern crate clap;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Name of the person to greet
    #[arg(short, long)]
    stream: String,

    // Number of times to greet
    #[arg(short, long)]
    download: String,

    // Calls the Schedule function
    #[arg(short, long)]
    schedule: String,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct SubArgs {
    /// stream
    #[arg(short, long)]
    stream: String,

    // I am reading the book rn

    /// download
    #[arg(short, long)]
    download: String,

    /// grab
    #[arg(short, long)]
    grab: String,

}

pub fn init() {
    let args = Args::parse();
    let subargs = SubArgs::parse();
    if !args.schedule.is_empty() {
        args.schedule.is_empty();
    }
}