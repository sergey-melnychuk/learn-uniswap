use std::time::{SystemTime, UNIX_EPOCH};

use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::{
        address,
        utils::{format_ether, format_units, parse_ether},
        Address, U256,
    },
    providers::{Provider, ProviderBuilder},
    signers::{k256::ecdsa::SigningKey, local::PrivateKeySigner},
};
use flashloans_course::contracts::{uniswapv2::IUniswapV2Router01, weth::IWETH, IERC20};

// https://book.getfoundry.sh/tutorials/forking-mainnet-with-cast-anvil
const MAINNET_FORK_URL: &str = "http://127.0.0.1:8545/";
const WALLET_SECRET_KEY: &str = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

// https://github.com/Uniswap/default-token-list/blob/main/src/tokens/mainnet.json
const WETH_CONTRACT_ADDRESS: Address = address!("C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2");
const USDT_CONTRACT_ADDRESS: Address = address!("dAC17F958D2ee523a2206206994597C13D831ec7");
// const WBTC_CONTRACT_ADDRESS: Address = address!("2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599");

const ETH_UNISWAPV2_ROUTER: Address = address!("7a250d5630B4cF539739dF2C5dAcb4c659F2488D");

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let provider = ProviderBuilder::new().on_http(MAINNET_FORK_URL.parse()?);

    let secret_key = SigningKey::from_slice(&hex::decode(WALLET_SECRET_KEY)?)?;
    let signer = PrivateKeySigner::from_signing_key(secret_key);
    let wallet = EthereumWallet::from(signer);
    let wallet_address = wallet.default_signer().address();
    println!("wallet: {wallet_address}");

    let gas_price = provider.get_gas_price().await?;
    println!("gas price: {gas_price}");

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_millis();

    let nonce = provider
        .get_transaction_count(wallet.default_signer().address())
        .await?;

    let weth = IWETH::new(WETH_CONTRACT_ADDRESS, &provider);

    let balance = weth
        .balanceOf(wallet_address)
        .call()
        .await?
        ._0;
    println!("WETH balance: {}", format_ether(balance));

    let nonce = if balance.is_zero() {
        let tx = weth
            .deposit()
            .into_transaction_request()
            .with_to(WETH_CONTRACT_ADDRESS)
            .with_nonce(nonce)
            .with_gas_limit(100_000)
            .with_max_fee_per_gas(gas_price * 2)
            .with_max_priority_fee_per_gas(1_000_000_000)
            .with_value(parse_ether("1")?);
        let tx = tx.build(&wallet).await?;
        let receipt = provider.send_tx_envelope(tx).await?.get_receipt().await?;
        println!("deposit TX: {}", receipt.transaction_hash);

        let balance = weth
            .balanceOf(wallet_address)
            .call()
            .await?
            ._0;
        println!("WETH balance: {}", format_ether(balance));
        nonce + 1
    } else {
        nonce
    };

    let target_address = USDT_CONTRACT_ADDRESS;
    let target = IERC20::new(target_address, &provider);
    let target_symbol = target.symbol().call().await?._0;
    let target_decimals = target.decimals().call().await?._0;

    let router = IUniswapV2Router01::new(ETH_UNISWAPV2_ROUTER, &provider);
    let amounts = router
        .getAmountsOut(parse_ether("1")?, vec![WETH_CONTRACT_ADDRESS, target_address])
        .call()
        .await?
        .amounts;
    let weth_amount = amounts[0];
    let target_amount = amounts[1];
    println!("1 WETH = {} {target_symbol}", format_units(target_amount, target_decimals)?);

    let tx = weth.approve(ETH_UNISWAPV2_ROUTER, weth_amount)
        .into_transaction_request()
        .with_nonce(nonce)
        .with_gas_limit(100_000)
        .with_max_fee_per_gas(gas_price * 2)
        .with_max_priority_fee_per_gas(1_000_000_000);
    let tx = tx.build(&wallet).await?;
    let receipt = provider.send_tx_envelope(tx).await?.get_receipt().await?;
    println!("approve TX: {}", receipt.transaction_hash);
    let nonce = nonce + 1;

    let call = router.swapExactTokensForTokens(
        weth_amount,
        target_amount,
        vec![WETH_CONTRACT_ADDRESS, target_address],
        wallet_address,
        U256::from(now + 1000 * 60 * 5), // 5 minutes
    );
    let tx = call.into_transaction_request()
        .with_nonce(nonce)
        .with_gas_limit(200_000)
        .with_max_fee_per_gas(gas_price * 2)
        .with_max_priority_fee_per_gas(1_000_000_000);
    let tx = tx.build(&wallet).await?;
    let receipt = provider.send_tx_envelope(tx).await?.get_receipt().await?;
    println!("uniswap TX: {}", receipt.transaction_hash);

    let balance = weth
        .balanceOf(wallet_address)
        .call()
        .await?
        ._0;
    println!("WETH balance: {}", format_ether(balance));

    let balance = target.balanceOf(wallet_address).call().await?._0;
    println!("{target_symbol} balance: {}", format_units(balance, target_decimals)?);

    Ok(())
}

/*

wallet: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
gas price: 22410828174
WETH balance: 0.000000000000000000
deposit TX: 0xed155072ce6421f4cd48e1d68247e163b01b6bd7cb947fd05455c9592f270ad4
WETH balance: 1.000000000000000000
1 WETH = 1952.909814504881918610 SUSHI
approve TX: 0x27b69b19adbe7904d83ff9e486e3e14674a07b4ddc6369c36b78dc0849cfee22
uniswap TX: 0x4d0c81d7e9ccfc22ce81097977eba21c4b4c6a32f1c69ac81d585c779b01bdc1
WETH balance: 0.000000000000000000
SUSHI balance: 1952.909814504881918610

*/
