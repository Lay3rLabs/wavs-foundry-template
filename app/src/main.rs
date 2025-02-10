use alloy::{
    hex,
    primitives::Address,
    providers::{Provider, ProviderBuilder},
};
use bindings::simplesubmit::SimpleSubmit;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let url = "http://127.0.0.1:8545";
    let provider = ProviderBuilder::new().on_builtin(url).await?;

    let block = provider.get_block_number().await?;
    if block == 0 {
        println!("No blocks / interactions on {url} yet.");
        return Ok(());
    }

    println!("Block: {:?}", block);

    // TODO: update me with your contract!
    let contract_address = "0x36c02da8a0983159322a80ffe9f24b1acff8b570".parse::<Address>()?;
    let contract = SimpleSubmit::new(contract_address, provider.clone());

    let resp = contract.getData(1).call().await?;

    let hex = resp.data;
    println!("Weather Response Hex: {:?}\n", hex);

    let ascii = to_ascii(hex.to_string().as_str())?;
    println!("Weather Response ASCII: {:?}", ascii);

    Ok(())
}

pub fn to_ascii(hex: &str) -> Result<String> {
    let bytes = hex::decode(hex)?;
    if !bytes.iter().all(u8::is_ascii) {
        return Err(eyre::eyre!("Invalid ASCII bytes"));
    }
    Ok(String::from_utf8(bytes).unwrap())
}
