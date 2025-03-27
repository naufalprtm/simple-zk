use sc_cli::RunCmd;

#[derive(Debug, clap::Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: Option<Subcommand>,

    #[clap(flatten)]
    pub run: RunCmd,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    /// Build the chain
    Build(sc_cli::BuildCmd),

    /// Run the chain
    Run(sc_cli::RunCmd),
}

fn main() -> sc_cli::Result<()> {
    let cli = Cli::parse();

    match &cli.subcommand {
        Some(Subcommand::Build(cmd)) => cmd.run(),
        Some(Subcommand::Run(cmd)) => cmd.run(),
        None => cli.run.run(),
    }
}