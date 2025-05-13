pragma solidity ^0.8.0;

contract StableUSD {
    string public name = "Stable USD";
    string public symbol = "stUSD";
    uint8 public decimals = 18;
    uint256 public totalSupply = 100000000 * 10 ** 18;

    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    constructor() {
        balanceOf[msg.sender] = totalSupply;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        require(balanceOf[msg.sender] >= amount, "Yetersiz bakiye");
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
        emit Transfer(msg.sender, to, amount);
        return true;
    }
}