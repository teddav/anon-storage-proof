// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.23;

import {console} from "forge-std/Test.sol";

import {TestUtils} from "./utils.sol";
import {ISemaphore} from "../src/ISemaphore.sol";

contract SemaphoreTest is TestUtils {
    ISemaphore semaphore;

    function setUp() public {
        vm.createSelectFork("https://rpc.ankr.com/eth_sepolia");
        semaphore = ISemaphore(0x42C0e6780B60E18E44B3AB031B216B6360009baB);
    }

    function test_SemaphoreCreateGroup() public {
        uint256 groupId = semaphore.createGroup();
        console.log("groupId", groupId);
    }
}
