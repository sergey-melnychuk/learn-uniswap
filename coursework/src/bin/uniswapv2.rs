use alloy::{
    primitives::{
        address,
        utils::{format_units, parse_units},
        Address,
    },
    providers::{Provider, ProviderBuilder},
};
use flashloans_course::contracts::{
    uniswapv2::{IUniswapV2Factory, IUniswapV2Router01},
    IERC20,
};

const ETH_UNISWAPV2_FACTORY: Address = address!("5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f");
const ETH_UNISWAPV2_ROUTER: Address = address!("7a250d5630B4cF539739dF2C5dAcb4c659F2488D");

const ETH_WETH_ADDRESS: Address = address!("C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2");
const ETH_SUSHI_ADDRESS: Address = address!("6B3595068778DD592e39A122f4f5a5cF09C90fE2");

const ETH_RPC_ENDPOINT: &str = "https://eth.llamarpc.com";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let provider = ProviderBuilder::new().on_http(ETH_RPC_ENDPOINT.parse()?);

    let latest_block = provider.get_block_number().await?;
    println!("Latest block number: {latest_block}");

    let factory = IUniswapV2Factory::new(ETH_UNISWAPV2_FACTORY, &provider);
    let pair_addr = factory
        .getPair(ETH_WETH_ADDRESS, ETH_SUSHI_ADDRESS)
        .call()
        .await?
        .pair;
    println!("Pair: {:?}", pair_addr);

    let weth_erc20 = IERC20::new(ETH_WETH_ADDRESS, &provider);
    let weth_decimals = weth_erc20.decimals().call().await?._0;

    let sushi_erc20 = IERC20::new(ETH_SUSHI_ADDRESS, &provider);
    let sushi_decimals = sushi_erc20.decimals().call().await?._0;

    let one = parse_units("1", weth_decimals)?;

    let router = IUniswapV2Router01::new(ETH_UNISWAPV2_ROUTER, &provider);
    let amounts = router
        .getAmountsOut(one.into(), vec![ETH_WETH_ADDRESS, ETH_SUSHI_ADDRESS])
        .call()
        .await?
        .amounts;

    let weth_out = format_units(amounts[0], weth_decimals)?;
    let sushi_out = format_units(amounts[1], sushi_decimals)?;
    println!("{} WETH = {} SUSHI", weth_out, sushi_out);

    Ok(())
}
