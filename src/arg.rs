use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Arg {
    pub config: String,

    /// Imply yes for all prompt
    #[arg(short, long)]
    pub yes: bool,
}

pub fn parse_arg() -> Arg {
    Arg::parse()
}
