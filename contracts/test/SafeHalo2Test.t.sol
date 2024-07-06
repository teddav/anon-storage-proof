// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.23;

import {console, stdJson} from "forge-std/Test.sol";
import {Safe} from "safe-contracts/Safe.sol";
import {ModuleManager} from "safe-contracts/base/ModuleManager.sol";

import {Counter} from "../src/Counter.sol";
import {SafeHalo2Module} from "../src/SafeHalo2Module.sol";
import {TestUtils} from "./utils.sol";
import {Halo2Verifier} from "../src/Verifier.sol";
import {Halo2VerifyingKey} from "../src/VerifyingKey.sol";

contract SafeHalo2Test is TestUtils {
    using stdJson for string;

    function test_VerifyZkProof() public {
        Safe safe = deployAndSetupSafe();

        address verifier = address(new Halo2Verifier());
        address verifyingKey = address(new Halo2VerifyingKey());
        SafeHalo2Module module = new SafeHalo2Module(
            address(safe),
            verifier,
            verifyingKey
        );

        string memory proofFile = vm.readFile("zk_proof.json");
        bytes memory proof = proofFile.readBytes(".proof");

        uint256[] memory instances = new uint256[](1);
        instances[0] = 7;

        bool verification = module.verifyProof(proof, instances);
        assertEq(verification, true);
    }

    function test_VerifyProofAndExec() public {
        Safe safe = deployAndSetupSafe();

        address verifier = address(new Halo2Verifier());
        address verifyingKey = address(new Halo2VerifyingKey());
        SafeHalo2Module module = new SafeHalo2Module(
            address(safe),
            verifier,
            verifyingKey
        );

        string memory proofFile = vm.readFile("zk_proof.json");
        bytes memory proof = proofFile.readBytes(".proof");

        uint256[] memory instances = new uint256[](1);
        instances[0] = 7;

        // enable module
        bytes memory txData_enableModule = abi.encodeWithSelector(
            ModuleManager.enableModule.selector,
            address(module)
        );
        execTransaction(safe, address(safe), txData_enableModule);

        Counter counter = new Counter(address(safe));
        assertEq(counter.number(), 0);

        bytes memory txData_execModuleTx = abi.encodeWithSelector(
            Counter.setNumber.selector,
            12345
        );
        module.execAnyTx(
            proof,
            instances,
            address(counter),
            0,
            txData_execModuleTx
        );

        assertEq(counter.number(), 12345);
    }
}
