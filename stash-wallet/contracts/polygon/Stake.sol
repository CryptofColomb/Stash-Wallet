pragma solidity ^0.8.0;

contract Stake {
    struct StakeInfo {
        uint256 amount;
        uint256 startTime;
        bool released;
    }

    mapping(address => StakeInfo) public stakes;

    function stake(uint256 amount) external payable {
        require(amount >= 50 * 1e18, "Minimum 50 stUSD/stEUR/stGBP stake edilmeli");
        require(amount <= 100000 * 1e18, "Maksimum 100,000 stake edilebilir");

        stakes[msg.sender] = StakeInfo({
            amount: amount,
            startTime: block.timestamp,
            released: false
        });
    }

    function release(address account) external {
        require(block.timestamp >= stakes[account].startTime + 30 days, "30 gün geçmedi");
        stakes[account].released = true;
    }
}