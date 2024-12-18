use alloy::{
    primitives::{
        address,
        utils::{format_units, parse_units},
        Address,
    },
    providers::{Provider, ProviderBuilder},
};

use flashloans_course::contracts::{
    pancake::{IPancakeFactory, IPancakeRouter01},
    IERC20,
};

// https://docs.pancakeswap.finance/developers/smart-contracts/pancakeswap-exchange/v2-contracts/factory-v2
const BSC_PANCAKE_FACTORY_ADDRESS: Address = address!("cA143Ce32Fe78f1f7019d7d551a6402fC5350c73");

// https://bscscan.com/address/0x10ed43c718714eb63d5aa57b78b54704e256024e
const BSC_PANCAKE_ROUTER_ADDRESS: Address = address!("10ed43c718714eb63d5aa57b78b54704e256024e");

const BSC_BUSD_ADDRESS: Address = address!("e9e7CEA3DedcA5984780Bafc599bD69ADd087D56");
const BSC_WBNB_ADDRESS: Address = address!("bb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c");

const BSC_RPC_ENDPOINT: &str = "https://bsc-dataseed1.binance.org/";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let provider = ProviderBuilder::new().on_http(BSC_RPC_ENDPOINT.parse()?);

    let latest_block = provider.get_block_number().await?;
    println!("Latest block number: {latest_block}");

    let factory = IPancakeFactory::new(BSC_PANCAKE_FACTORY_ADDRESS, &provider);
    let pair_addr = factory
        .getPair(BSC_BUSD_ADDRESS, BSC_WBNB_ADDRESS)
        .call()
        .await?
        .pair;
    println!("Pair: {:?}", pair_addr);

    let busd_erc20 = IERC20::new(BSC_BUSD_ADDRESS, &provider);
    let busd_decimals = busd_erc20.decimals().call().await?._0;

    let wbnb_erc20 = IERC20::new(BSC_WBNB_ADDRESS, &provider);
    let wbnb_decimals = wbnb_erc20.decimals().call().await?._0;

    let one = parse_units("1", busd_decimals)?;

    let router = IPancakeRouter01::new(BSC_PANCAKE_ROUTER_ADDRESS, &provider);
    let amounts = router
        .getAmountsOut(one.into(), vec![BSC_BUSD_ADDRESS, BSC_WBNB_ADDRESS])
        .call()
        .await?
        .amounts;

    let busd_out = format_units(amounts[0], busd_decimals)?;
    let wbnb_out = format_units(amounts[1], wbnb_decimals)?;
    println!("{} BUSD = {} WBNB", busd_out, wbnb_out);

    Ok(())
}
