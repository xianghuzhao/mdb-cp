use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Arg {
    pub config: String,

    /// Use gzip in copy
    #[arg(short, long)]
    pub gzip: bool,

    /// Imply yes for all prompt
    #[arg(short, long)]
    pub yes: bool,

    /// Optional mongodump command path
    #[arg(long, value_name = "COMMAND")]
    pub dump: Option<String>,

    /// Optional mongorestore command path
    #[arg(long, value_name = "COMMAND")]
    pub restore: Option<String>,
}

pub fn parse_arg() -> Arg {
    Arg::parse()
}
