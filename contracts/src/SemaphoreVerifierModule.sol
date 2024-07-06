// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.23;

import {ISemaphore} from "./ISemaphore.sol";
import {Enum} from "safe-contracts/common/Enum.sol";

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

contract SemaphoreVerifierModule {
    GnosisSafe safe;
    ISemaphore public semaphore;
    uint256 public groupId;

    constructor(address _safe, ISemaphore _semaphore) {
        safe = GnosisSafe(_safe);
        semaphore = _semaphore;
        groupId = semaphore.createGroup();
    }

    function execAnyTx(
        ISemaphore.SemaphoreProof calldata proof,
        address to,
        uint256 value,
        bytes calldata data
    ) public {
        semaphore.validateProof(groupId, proof);
        safe.execTransactionFromModule(to, value, data, Enum.Operation.Call);
    }

    function addMember(uint256 identityCommitment) external {
        // we need to check if the identity corresponds to one of the signers
        semaphore.addMember(groupId, identityCommitment);
    }
}
