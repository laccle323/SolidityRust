pragma solidity ^0.8.0;

contract SimpleWallet {
    address public owner;
    mapping(address => uint256) public balances;

    // Custom event
    event Deposited(address indexed sender, uint256 amount);

    // Set the contract creator as the owner
    constructor() {
        owner = msg.sender;
    }

    // Accept ETH deposits and update the sender's balance
    function deposit() external payable {
        require(msg.value > 0, "Deposit must be greater than 0");
        balances[msg.sender] += msg.value;
        emit Deposited(msg.sender, msg.value);
    }

    // Allow only the owner to withdraw the entire contract balance
    function withdraw(uint256 amount) external {
        require(msg.sender == owner, "Only owner can withdraw");
        require(address(this).balance >= amount, "Insufficient contract balance");
        payable(owner).transfer(amount);
    }

    // Get the contract's ETH balance
    function getContractBalance() external view returns (uint256) {
        return address(this).balance;
    }
}
