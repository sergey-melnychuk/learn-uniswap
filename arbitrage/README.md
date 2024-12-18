```bash
forge init arbitrage && cd arbitrage
forge install uniswap/v2-core

## Add '@uniswap-v2=lib/v2-core' to remappings.txt
## Downgrade Solidity compiler to 0.5.16 (required by Uniswap V2)
## Get rid of forge-std and scripts (they require solc ^0.8.0)

forge build && forge test
```
