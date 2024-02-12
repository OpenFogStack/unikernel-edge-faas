use clap::Parser;

#[derive(Parser, Debug)]
#[command(term_width = 0)]
pub struct Args {
    #[arg(long, short = 'r', value_name = "path", value_hint = clap::ValueHint::DirPath)]
    pub registry: std::path::PathBuf,
    #[arg(
        long,
        short = 'l',
        value_name = "error|warn|info|debug|trace",
        default_value = "info"
    )]
    pub log_lvl: tracing::Level,
}

pub fn args() -> Args {
    Args::parse()
}
