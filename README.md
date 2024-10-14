# Rust Assignment

This is a rust based smart contract which is deployed to ICP local test network which performs the functionalities as asked.
The functions are:
1. Send Tokens:
○ Users is able to send tokens to other addresses.
2. Receive Tokens:
○ The wallet is able to receive tokens and update the balance
accordingly.
3. Balance Display:
○ Shows the current token balance of the wallet


## Running the project locally

If you want to test the working of this project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --clean --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy

```

Lets Run a simple test case where we
1) Initialize a token
2) Check the balance of the account to check if the tokens are minted in the account or not
3) Transfer some tokens from one address to another address
4) Check the balance of both the accounts
```bash
#Minting 1000000 custom tokens 
dfx canister call rust_hello_backend init_token '( "TEST", "Test Token", 1000000, 8 )'

# Checking if the number of tokens are minted
dfx canister call rust_hello_backend balance_of '(principal "5efnn-telku-74vvo-k7d22-adlyf-p4kkz-ehcns-3dxsp-s5x5m-okk6e-yae")'

# Using the transfer function to transfer 50000 tokens to another account
dfx canister call rust_hello_backend transfer '(principal "pslmo-vbi5f-lrz6s-m5ocl-a75r3-7vszq-kn64o-4ucje-hfud5-aeiaf-kae", 50000)'

# Checking the balance of the senders account
dfx canister call rust_hello_backend balance_of '(principal "5efnn-telku-74vvo-k7d22-adlyf-p4kkz-ehcns-3dxsp-s5x5m-okk6e-yae")'

# Checking the balance of the recipient account
dfx canister call rust_hello_backend balance_of '(principal "pslmo-vbi5f-lrz6s-m5ocl-a75r3-7vszq-kn64o-4ucje-hfud5-aeiaf-kae")'

```
