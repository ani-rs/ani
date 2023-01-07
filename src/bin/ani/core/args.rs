use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    stream: String,
    download: String,
    schedule: String,
    grab: String,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct StreamArgs {
    #[arg(long)]
    range: String,

    #[arg(short)]
    r: String,

    #[arg(long)]
    special: String,

    #[arg(short)]
    s: String,

    #[arg(long)]
    quality: String,
    #[arg(short)]
    q: String,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct DownloadArgs {
    #[arg(long)]
    range: String,

    #[arg(short)]
    r: String,

    #[arg(long)]
    special: String,

    #[arg(short)]
    s: String,

    #[arg(long)]
    quality: String,

    #[arg(short)]
    q: String,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct GrabArgs {
    #[arg(long)]
    provider: String,

    #[arg(short)]
    p: String,

    #[arg(long)]
    range: String,

    #[arg(short)]
    r: String,

    #[arg(long)]
    special: String,

    #[arg(short)]
    s: String,

    #[arg(long)]
    quality: String,

    #[arg(short)]
    q: String,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct ScheduleArgs {
    #[arg(short)]
    r: String,

    #[arg(long)]
    special: String,

    #[arg(short)]
    s: String,

    #[arg(long)]
    quality: String,

    #[arg(short)]
    q: String,
}

pub fn init() {
    let args = Args::parse();
    if !args.stream.is_empty() {
        println!("Schedule")
    }

    if !args.stream.is_empty() {
        println!("Stream")
    }

    if !args.download.is_empty() {
        println!("Download")
    }
}