```bash
forge init arbitrage && cd arbitrage

forge install uniswap/v2-core
echo '@uniswap-v2=lib/v2-core >> remappings.txt

## Downgrade Solidity compiler to 0.5.16 (required by Uniswap V2)
echo 'solc_version = "0.5.16"' >> foundry.txt

## Get rid of forge-std and scripts (they require solc ^0.8.0)
rm -rf scripts
vim test/Counter.t.sol

forge build
forge test
```
