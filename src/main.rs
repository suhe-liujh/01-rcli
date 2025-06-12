use clap::Parser;
use rcli::*;

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(csv_opts) => {
            process_csv(&csv_opts.input, &csv_opts.output)?;
        }
    }

    Ok(())
}
