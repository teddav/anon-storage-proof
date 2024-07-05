// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

import {Test, console} from "forge-std/Test.sol";
import "safe-contracts/Safe.sol";
import "safe-contracts/proxies/SafeProxy.sol";
import {Counter} from "../contracts/Counter.sol";

contract CounterTest is Test {
    uint256[3] pks = [
        0x47e179ec197488593b187f80a00eb0da91f1b9d0b13f8733639f19c30a34926a,
        0x8b3a350cf5c34c9194ca85829a2df0ec3153be0318b5e2d3348e872092edffba,
        0x92db14e403b83dfe3df233f83dfa3a0d7096f21ca9b0d6d6b8d88b2b4ec1564e
    ];

    function test_Deploy() public {
        Counter counter = new Counter();
        counter.setNumber(123);

        address singleton = address(new Safe());
        Safe safe = Safe(payable(address(new SafeProxy(singleton))));

        address[] memory owners = new address[](3);
        owners[0] = vm.addr(pks[0]);
        owners[1] = vm.addr(pks[1]);
        owners[2] = vm.addr(pks[2]);

        safe.setup(
            owners,
            2,
            address(0),
            bytes(""),
            address(0),
            address(0),
            0,
            payable(address(0))
        );

        bytes memory tx_data = abi.encodeWithSelector(
            Counter.setNumber.selector,
            456
        );
        // console.logBytes(tx_data);

        bytes32 to_sign = safe.getTransactionHash(
            address(counter),
            0,
            tx_data,
            Enum.Operation(0),
            1_000_000,
            1_000_000,
            1_000_000,
            address(0),
            address(0),
            0
        );

        bytes memory signatures = getSignature(to_sign);

        assertEq(counter.number(), 123);

        safe.execTransaction(
            address(counter),
            0,
            tx_data,
            Enum.Operation(0),
            1_000_000,
            1_000_000,
            1_000_000,
            address(0),
            payable(address(0)),
            signatures
        );

        assertEq(counter.number(), 456);
    }

    function getSignature(
        bytes32 to_sign
    ) internal view returns (bytes memory) {
        (uint8 v1, bytes32 r1, bytes32 s1) = vm.sign(pks[0], to_sign);
        (uint8 v2, bytes32 r2, bytes32 s2) = vm.sign(pks[1], to_sign);
        bytes memory sig1 = abi.encodePacked(r1, s1, v1);
        bytes memory sig2 = abi.encodePacked(r2, s2, v2);
        bytes memory signatures = abi.encodePacked(sig1, sig2);

        // console.log("======");
        // bytes32 header;
        // uint256 recovered_r;
        // uint256 recovered_s;
        // uint256 recovered_v;

        // assembly {
        //     let signaturePos := mul(0x41, 1)
        //     header := mload(signatures)
        //     recovered_r := mload(add(signatures, add(signaturePos, 0x20)))
        //     recovered_s := mload(add(signatures, add(signaturePos, 0x40)))
        //     recovered_v := and(
        //         mload(add(signatures, add(signaturePos, 0x41))),
        //         0xff
        //     )
        // }
        // console.log("header");
        // console.logBytes32(bytes32(header));
        // console.log("r");
        // console.logBytes32(bytes32(recovered_r));
        // console.log("s");
        // console.logBytes32(bytes32(recovered_s));
        // console.log("v", recovered_v);
        // console.log("======");

        return signatures;
    }
}
