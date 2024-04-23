use clap::Parser;
use csv::{ReaderBuilder, Trim, Writer};
use toy_engine::{Config, Engine, Error};

fn main() -> Result<(), Error> {
    // Parse the program config.
    let config = Config::parse();

    // Open the input file and process its content.
    let mut engine = Engine::default();
    let reader = ReaderBuilder::new()
        .trim(Trim::All)
        .from_path(config.input_file)?;
    engine.load_from_reader(reader)?;
    // Dump the accounts to stdout.
    let writer = Writer::from_writer(std::io::stdout());
    engine.dump_accounts(writer)?;

    Ok(())
}
