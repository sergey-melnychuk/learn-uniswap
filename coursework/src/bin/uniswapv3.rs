use alloy::{
    primitives::{
        address,
        aliases::U24,
        utils::{format_units, parse_units},
        Address, U160,
    },
    providers::{Provider, ProviderBuilder},
};
use flashloans_course::contracts::{uniswapv3::IQuoter, IERC20};

const ETH_UNISWAPV3_QUOTER: Address = address!("b27308f9F90D607463bb33eA1BeBb41C27CE5AB6");
// const ETH_UNISWAPV3_FACTORY: Address = address!("1F98431c8aD98523631AE4a59f267346ea31F984");

const ETH_WBTC_ADDRESS: Address = address!("2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599");
const ETH_WETH_ADDRESS: Address = address!("C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2");
const ETH_USDC_ADDRESS: Address = address!("A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48");

const ETH_RPC_ENDPOINT: &str = "https://eth.llamarpc.com";

const FEE: u32 = 3000;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let provider = ProviderBuilder::new().on_http(ETH_RPC_ENDPOINT.parse()?);

    let latest_block = provider.get_block_number().await?;
    println!("Latest block number: {latest_block}");

    let wbtc_erc20 = IERC20::new(ETH_WBTC_ADDRESS, &provider);
    let wbtc_decimals = wbtc_erc20.decimals().call().await?._0;

    let weth_erc20 = IERC20::new(ETH_WETH_ADDRESS, &provider);
    let weth_decimals = weth_erc20.decimals().call().await?._0;

    let usdc_erc20 = IERC20::new(ETH_USDC_ADDRESS, &provider);
    let usdc_decimals = usdc_erc20.decimals().call().await?._0;

    let quoter = IQuoter::new(ETH_UNISWAPV3_QUOTER, &provider);

    let one = parse_units("1", wbtc_decimals)?;
    let amount = quoter
        .quoteExactInputSingle(
            ETH_WBTC_ADDRESS,
            ETH_USDC_ADDRESS,
            U24::from(FEE),
            one.into(),
            U160::ZERO,
        )
        .call()
        .await?
        .amountOut;
    let wbtc = format_units(one, wbtc_decimals)?;
    let usdc = format_units(amount, usdc_decimals)?;
    println!("{wbtc} WBTC = {usdc} USDC");

    let one = parse_units("1", weth_decimals)?;
    let amount = quoter
        .quoteExactInputSingle(
            ETH_WETH_ADDRESS,
            ETH_USDC_ADDRESS,
            U24::from(FEE),
            one.into(),
            U160::ZERO,
        )
        .call()
        .await?
        .amountOut;
    let weth = format_units(one, weth_decimals)?;
    let usdc = format_units(amount, usdc_decimals)?;
    println!("{weth} WETH = {usdc} USDC");

    Ok(())
}
