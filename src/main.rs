use clap::Parser;
use rcli::*;

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(csv_opts) => {
            let output = if let Some(output) = csv_opts.output {
                output.clone()
            } else {
                format!("output.{}", csv_opts.format)
            };

            process_csv(&csv_opts.input, output, csv_opts.format)?;
        }
        SubCommand::Genpass(genpass_opts) => {
            process_genpass(
                genpass_opts.length,
                genpass_opts.uppercase,
                genpass_opts.lowercase,
                genpass_opts.digits,
                genpass_opts.symbols,
            )?;
        }
    }

    Ok(())
}
