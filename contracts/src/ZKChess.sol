// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {ERC721} from "../lib/solmate/src/tokens/ERC721.sol";

contract ZKChess is ERC721 {
    uint256 public totalSupply;

    address public alignedServiceManager;
    address public paymentServiceAddr;

    bytes32 public elfCommitment = 0x866626c7b7feda5c3734f4f22aa3724df5245b902b52a4dda54ef2324095e193;
        
    error InvalidElf(bytes32 submittedElf); // c6d95066

    // map to check if proof has already been submitted
    mapping(bytes32 => bool) public mintedProofs;

    constructor(
        address _alignedServiceManager,
        address _paymentServiceAddr
    ) ERC721("ZKChess", unicode"ZK♕") {
        alignedServiceManager = _alignedServiceManager;
        paymentServiceAddr = _paymentServiceAddr;
    }

    function verifyBatchInclusion(
        bytes32 proofCommitment,
        bytes32 pubInputCommitment,
        bytes32 provingSystemAuxDataCommitment,
        bytes20 proofGeneratorAddr,
        bytes32 batchMerkleRoot,
        bytes memory merkleProof,
        uint256 verificationDataBatchIndex
    ) external returns (uint256) {
        if (elfCommitment != provingSystemAuxDataCommitment) {
            revert InvalidElf(provingSystemAuxDataCommitment);
        }
        require(
            address(proofGeneratorAddr) == msg.sender,
            "proofGeneratorAddr does not match"
        );

        bytes32 fullHash = keccak256(
            abi.encodePacked(
                proofCommitment,
                pubInputCommitment,
                provingSystemAuxDataCommitment,
                proofGeneratorAddr
            )
        );
        require(!mintedProofs[fullHash], "proof already minted");

        (
            bool callWasSuccessfull,
            bytes memory proofIsIncluded
        ) = alignedServiceManager.staticcall(
                abi.encodeWithSignature(
                    "verifyBatchInclusion(bytes32,bytes32,bytes32,bytes20,bytes32,bytes,uint256,address)",
                    proofCommitment,
                    pubInputCommitment,
                    provingSystemAuxDataCommitment,
                    proofGeneratorAddr,
                    batchMerkleRoot,
                    merkleProof,
                    verificationDataBatchIndex,
                    paymentServiceAddr
                )
            );

        require(callWasSuccessfull, "static_call failed");

        bool proofIsIncludedBool = abi.decode(proofIsIncluded, (bool));
        require(proofIsIncludedBool, "proof not included in batch");

        mintedProofs[fullHash] = true;

        uint256 tokenId = totalSupply++;
        _mint(msg.sender, tokenId);

        return tokenId;
    }

    function tokenURI(
        uint256
    ) public view virtual override returns (string memory) {
        return "ipfs://QmUKviny9x2oQUegyJFFBAUU2q5rvu5CsPzrUaBSDukpHQ";
    }
}
