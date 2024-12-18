use alloy::sol;

sol! {
    #[sol(rpc)]
    interface IQuoter {
        function quoteExactInputSingle(
            address tokenIn,
            address tokenOut,
            uint24 fee,
            uint256 amountIn,
            uint160 sqrtPriceLimitX96
          ) external returns (uint256 amountOut);
    }

    #[sol(rpc)]
    interface IUniswapV3Factory {
        function getPool(
            address tokenA,
            address tokenB,
            uint24 fee
          ) external view returns (address pool);
    }
}
