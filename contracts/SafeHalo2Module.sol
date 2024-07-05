// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

import {Enum} from "safe-contracts/common/Enum.sol";
import {Halo2Verifier} from "./Verifier.sol";
import {Halo2VerifyingKey} from "./VerifyingKey.sol";

interface GnosisSafe {
    /// @dev Allows a Module to execute a Safe transaction without any further confirmations.
    /// @param to Destination address of module transaction.
    /// @param value Ether value of module transaction.
    /// @param data Data payload of module transaction.
    /// @param operation Operation type of module transaction.
    function execTransactionFromModule(
        address to,
        uint256 value,
        bytes calldata data,
        Enum.Operation operation
    ) external returns (bool success);
}

contract SafeHalo2Module {
    GnosisSafe public safe;
    Halo2Verifier public verifier;
    address public verifyingKey;

    constructor(address _safe, address _verifier, address _verifyingKey) {
        safe = GnosisSafe(_safe);
        verifier = Halo2Verifier(_verifier);
        verifyingKey = _verifyingKey;
    }

    function execAnyTx(
        bytes calldata proof,
        uint256[] calldata instances,
        address to,
        uint256 value,
        bytes calldata data
    ) public {
        require(verifyProof(proof, instances), "proof could not be verified");
        safe.execTransactionFromModule(to, value, data, Enum.Operation.Call);
    }

    function verifyProof(
        bytes calldata proof,
        uint256[] calldata instances
    ) public view returns (bool) {
        return verifier.verifyProof(verifyingKey, proof, instances);
    }
}
