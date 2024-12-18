// SPDX-License-Identifier: UNLICENSED
pragma solidity =0.5.16;

import {Counter} from "../src/Counter.sol";

contract CounterTest {
    Counter public counter;

    function setUp() public {
        counter = new Counter();
        counter.setNumber(0);
    }

    function test_Increment() public {
        counter.increment();
        assert(counter.number() == 1);
    }

    function testFuzz_SetNumber(uint256 x) public {
        counter.setNumber(x);
        assert(counter.number() == x);
    }
}
