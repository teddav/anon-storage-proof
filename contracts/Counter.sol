// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

contract Counter {
    address safeAdmin;
    uint256 public number;

    modifier onlyAdmin() {
        require(msg.sender == safeAdmin, "not admin");
        _;
    }

    constructor(address _safeAdmin) {
        safeAdmin = _safeAdmin;
    }

    function setNumber(uint256 newNumber) public onlyAdmin {
        number = newNumber;
    }
}
