pragma solidity ^0.8.0;

contract Referral {
    mapping(address => uint256) public referralCount;
    mapping(address => bool) public hasReferred;

    function refer(address referrer) external {
        require(referralCount[referrer] < 10, "Referans limitine ulaşıldı");
        require(!hasReferred[msg.sender], "Zaten referans kullandınız");

        referralCount[referrer] += 1;
        hasReferred[msg.sender] = true;
    }
}