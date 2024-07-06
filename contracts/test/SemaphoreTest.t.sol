// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.23;

import {console} from "forge-std/Test.sol";
import {Safe} from "safe-contracts/Safe.sol";
import {ModuleManager} from "safe-contracts/base/ModuleManager.sol";

import {TestUtils} from "./utils.sol";
import {ISemaphore} from "../src/ISemaphore.sol";
import {SemaphoreMasterModule} from "../src/SemaphoreVerifierModule.sol";

contract SemaphoreTest is TestUtils {
    ISemaphore semaphore;

    function setUp() public {
        vm.createSelectFork("https://rpc.ankr.com/eth_sepolia");
        semaphore = ISemaphore(0x42C0e6780B60E18E44B3AB031B216B6360009baB);
    }

    function test_SemaphoreValidateProof() public {
        uint256 groupId = semaphore.createGroup();
        console.log("groupId", groupId);

        semaphore.addMember(
            groupId,
            1772263923816844738218801185744046056658685953373862850595384902837154819354
        );
        semaphore.addMember(
            groupId,
            18332492625122532256147860717901555793783137999081466661877342842509167197884
        );
        semaphore.addMember(
            groupId,
            14666505891533126344042109479207383749941956403874345698933763060880398309605
        );

        ISemaphore.SemaphoreProof memory proof = ISemaphore.SemaphoreProof({
            merkleTreeDepth: 2,
            merkleTreeRoot: 9501249152673732179233955257213499012687906844097516139405623068204843435416,
            nullifier: 9582533315389292433808407312629026681536829446312722967027066035428001646761,
            message: 47685659316963314470182056101991554889330944767214389596285906550081655406592,
            scope: 9501249152673732179233955257213499012687906844097516139405623068204843435416,
            points: [
                21473312563566601488854077220969640323486914488050523484412547489288271382331,
                5473843783496297063103463173283166643107238194703059336924521198147974811717,
                18562648816247348357866903681966289223799778479370720570808990601592150738837,
                17935461727259458549107343629637863731717496073386473899559240855955942612025,
                18984690666677572187175990617044797145590723400784307522299224039929281350438,
                1121867103105004466499809311430530807629297219632808380427437673982185798085,
                7861989758471127548859484200749311818228580874401693358933375276146280910903,
                10902964393937992207857159788083102478139535975014401899794527718214199374908
            ]
        });

        require(semaphore.verifyProof(groupId, proof), "invalid proof");
        semaphore.validateProof(groupId, proof);

        vm.expectRevert(
            ISemaphore.Semaphore__YouAreUsingTheSameNullifierTwice.selector
        );
        semaphore.validateProof(groupId, proof);
    }

    function test_SemaphoreModuleAddMember() public {
        Safe safe = deployAndSetupSafe();
        SemaphoreMasterModule module = new SemaphoreMasterModule(
            address(safe),
            semaphore
        );

        bytes memory txData_enableModule = abi.encodeWithSelector(
            ModuleManager.enableModule.selector,
            address(module)
        );
        execTransaction(safe, address(safe), txData_enableModule);

        uint256 commitment = 1772263923816844738218801185744046056658685953373862850595384902837154819354;
        bytes memory data = abi.encode(commitment);
        bytes32 dataHash = keccak256(data);
        bytes memory signatures = getSignature(dataHash);

        module.addRemoveMember(
            SemaphoreMasterModule.MemberAction.AddMember,
            dataHash,
            data,
            signatures
        );

        require(semaphore.hasMember(module.groupId(), commitment));
    }

    function test_SemaphoreModuleRemoveMember() public {
        Safe safe = deployAndSetupSafe();
        SemaphoreMasterModule module = new SemaphoreMasterModule(
            address(safe),
            semaphore
        );

        bytes memory txData_enableModule = abi.encodeWithSelector(
            ModuleManager.enableModule.selector,
            address(module)
        );
        execTransaction(safe, address(safe), txData_enableModule);

        uint256 commitment = 1772263923816844738218801185744046056658685953373862850595384902837154819354;
        bytes memory data = abi.encode(commitment);
        bytes32 dataHash = keccak256(data);
        bytes memory signatures = getSignature(dataHash);

        module.addRemoveMember(
            SemaphoreMasterModule.MemberAction.AddMember,
            dataHash,
            data,
            signatures
        );
        require(semaphore.hasMember(module.groupId(), commitment));

        uint256[] memory siblings = new uint256[](2);
        siblings[
            0
        ] = 18332492625122532256147860717901555793783137999081466661877342842509167197884;
        siblings[
            1
        ] = 14666505891533126344042109479207383749941956403874345698933763060880398309605;

        bytes memory data2 = abi.encode(commitment, siblings);
        bytes32 dataHash2 = keccak256(data2);
        bytes memory signatures2 = getSignature(dataHash2);

        module.addRemoveMember(
            SemaphoreMasterModule.MemberAction.RemoveMember,
            dataHash2,
            data2,
            signatures2
        );
        require(!semaphore.hasMember(module.groupId(), commitment));
    }
}
