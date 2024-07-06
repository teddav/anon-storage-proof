// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.23;

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

contract SafeBasicModule {
    GnosisSafe safe;

    constructor(address _safe) {
        safe = GnosisSafe(_safe);
    }

    function execAnyTx(address to, uint256 value, bytes calldata data) public {
        safe.execTransactionFromModule(to, value, data, Enum.Operation.Call);
    }
}
