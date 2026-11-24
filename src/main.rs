use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "scindere",
    about = "Resource-aware RNA splicing quantification for single-cell transcriptomics",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
#[allow(clippy::large_enum_variant)] // Keep Clap's short-lived argument values inline.
enum Commands {
    Run(scindere::commands::run::RunArgs),
    Run10x(scindere::commands::run10x::Run10xArgs),
    RunDropest(scindere::commands::run_dropest::RunDropestArgs),
    RunSmartseq2(scindere::commands::run_smartseq2::RunSmartseq2Args),
    DropestBcCorrect(scindere::commands::dropest_bc_correct::DropestBcCorrectArgs),
    WriteOutput(scindere::commands::write_output::WriteOutputArgs),
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let verbose = match &cli.command {
        Commands::Run(a) => a.verbose,
        Commands::Run10x(a) => a.verbose,
        Commands::RunDropest(a) => a.verbose,
        Commands::RunSmartseq2(a) => a.verbose,
        Commands::DropestBcCorrect(a) => a.verbose,
        Commands::WriteOutput(a) => a.verbose,
    };
    let level = if verbose {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };
    env_logger::builder().filter_level(level).init();
    match cli.command {
        Commands::Run(args) => scindere::commands::run::run(args),
        Commands::Run10x(args) => scindere::commands::run10x::run10x(args),
        Commands::RunDropest(args) => scindere::commands::run_dropest::run_dropest(args),
        Commands::RunSmartseq2(args) => scindere::commands::run_smartseq2::run_smartseq2(args),
        Commands::DropestBcCorrect(args) => {
            scindere::commands::dropest_bc_correct::dropest_bc_correct(args)
        }
        Commands::WriteOutput(args) => scindere::commands::write_output::write_output(args),
    }
}
