use anyhow::Result;
use substreams_ethereum::Abigen;

fn main() -> Result<()> {
    Abigen::new("Factory", "abi/factory.json")?
        .generate()?
        .write_to_file("src/abi/factory.rs")?;

    Abigen::new("Pool", "abi/pool.json")?
        .generate()?
        .write_to_file("src/abi/pool.rs")?;

    Abigen::new("Erc20", "abi/erc20.json")?
        .generate()?
        .write_to_file("src/abi/erc20.rs")?;

    Ok(())
}
