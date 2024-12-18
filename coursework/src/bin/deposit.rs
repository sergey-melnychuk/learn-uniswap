use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::{
        address,
        utils::{format_ether, parse_ether},
        Address,
    },
    providers::{Provider, ProviderBuilder},
    signers::{k256::ecdsa::SigningKey, local::PrivateKeySigner},
};
use flashloans_course::contracts::weth::IWETH;

const ETH_SEPOLIA_RPC_URL: &str = "https://ethereum-sepolia-rpc.publicnode.com";

// https://github.com/Uniswap/default-token-list/blob/main/src/tokens/sepolia.json
const WETH_CONTRACT_ADDRESS: Address = address!("fFf9976782d46CC05630D1f6eBAb18b2324d6B14");

// 0x5f80F153589d71c91e5937FbeE2a198b43Be581e
const SECRET_KEY: &str = "cafebabecafebabecafebabecafebabecafebabecafebabecafebabecafebabe";

#[tokio::main]
pub async fn main() -> eyre::Result<()> {
    let provider = ProviderBuilder::new().on_http(ETH_SEPOLIA_RPC_URL.parse()?);

    let secret_key = SigningKey::from_slice(&hex::decode(SECRET_KEY)?)?;
    let signer = PrivateKeySigner::from_signing_key(secret_key);
    let wallet = EthereumWallet::from(signer);
    println!("wallet: {}", wallet.default_signer().address());

    let nonce = provider
        .get_transaction_count(wallet.default_signer().address())
        .await?;
    let nonce = nonce - 1; // NOTE: make nonce invalid to avoid unexpected execution
    println!("nonce: {nonce}");

    let gas_price = provider.get_gas_price().await?;
    println!(" gas: {}", format_ether(gas_price));

    let weth = IWETH::new(WETH_CONTRACT_ADDRESS, &provider);

    let balance = weth
        .balanceOf(wallet.default_signer().address())
        .call()
        .await?
        ._0;
    println!("WETH: {}", format_ether(balance));

    let tx = weth
        .deposit()
        .into_transaction_request()
        .with_chain_id(provider.get_chain_id().await?)
        .with_to(WETH_CONTRACT_ADDRESS)
        .with_nonce(nonce)
        .with_gas_limit(100_000)
        .with_max_fee_per_gas(20_000_000_000)
        .with_max_priority_fee_per_gas(1_000_000_000)
        .with_value(parse_ether("0.1")?);

    let tx = tx.build(&wallet).await?;
    let receipt = provider.send_tx_envelope(tx).await?.get_receipt().await?;

    println!("TX: {}", receipt.transaction_hash);

    let balance = weth
        .balanceOf(wallet.default_signer().address())
        .call()
        .await?
        ._0;
    println!("WETH: {}", format_ether(balance));

    Ok(())
}
