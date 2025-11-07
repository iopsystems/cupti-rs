use clap::Parser;

mod regenerate;

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    Regenerate(crate::regenerate::Regenerate),
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Regenerate(cmd) => cmd.run(),
    }
}
