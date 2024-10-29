// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {ZKChess} from "../src/ZKChess.sol";

contract CounterScript is Script {

    function setUp() public {}

    function run() public {
        vm.startBroadcast();

        address ALIGNED_SERVICE_MANAGER_ADDRESS = 0x58F280BeBE9B34c9939C3C39e0890C81f163B623;
        address BATCHER_PAYMENT_SERVICE_ADDRESS = 0x815aeCA64a974297942D2Bbf034ABEe22a38A003;
    
        ZKChess deployed = new ZKChess(ALIGNED_SERVICE_MANAGER_ADDRESS, BATCHER_PAYMENT_SERVICE_ADDRESS);

        vm.stopBroadcast();
    }
}
