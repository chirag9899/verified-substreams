use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("PrimaryIssuePoolFactory", "abi/PrimaryIssuePoolFactory.json")?
        .generate()?
        .write_to_file("src/abi/factory.rs")?;

    Ok(()),

    Abigen::new("PrimaryIssuePool", "abi/PrimaryIssuePool.json")?
        .generate()?
        .write_to_file("src/abi/pool.rs")?;

    Ok(())
}
