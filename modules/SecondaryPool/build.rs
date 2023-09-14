use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("SecondaryIssuePoolFactory", "abi/SecondaryIssuePoolFactory.json")?
        .generate()?
        .write_to_file("src/abi/factory.rs")?;

    Ok(());

    Abigen::new("SecondaryIssuePool", "abi/SecondaryIssuePool.json")?
        .generate()?
        .write_to_file("src/abi/pool.rs")?;

    Ok(())
}
