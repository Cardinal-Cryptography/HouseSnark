# blender-cli
Blending assets with SNARKs from your CLI

## Dev instructions
1. Checkout `aleph-node` to `snarkeling` branch and build the node (`cargo build --release`).
2. Comment out invocation to `verify_deposit()` in `Blender` contract (`contracts/blender/contract.rs`).
3. Build docker image `docker build --tag aleph-node:snarkeling -f ./docker/Dockerfile .`

## Run one-node `snarknode` chain

```bash
./contracts/run_snarknode.sh
```

## Deploy blender and PSP22 token contracts

```bash
cd contracts
./setup_blending.sh -r false -n ws://127.0.0.1:9943
```

Script will register the token with the blender contract at id 0 as well as give it the allowance to spend up to total_supply of the token on behalf of Alice.

## Interact with the blender contract

Use `//Alice` as account seed and issue cli commands from the tool directory:

```bash
cd blender-cli
```

### Set node RPC endpoint address

```bash
cargo run --release -- --seed //Alice set-node ws://127.0.0.1:9943
```

### Persist Blender contract address instance

```bash
cargo run --release -- --seed //Alice set-contract-address <blender-addrs>
```

### Register new PSP22 token contract with Blender instance

> This step is not required if you're working with local chain and had successfuly ran `./setup_blending.sh`

Before using Blender contract to deposit/withdraw tokens, we need to first "register" the PSP22 token contract in our Blender instance. Without this action, any transactions calling `deposit`/`withdraw` will fail.

**NOTE:** Token IDs in Blender instance have to be unique so if your transaction gets front-runned by someone else, trying to register under same `--token-id` you will have to retry the txn with a new token ID value.

```bash
cargo run --release -- --seed //Alice register-token --token-id 0 --token-address <PSP22_token_contract_address>
```

### Deposit a note

> Assumes that you've successfuly completed previous step of registering a PSP token under `--token-id 0` and that you had approved allowance of that PSP token to the Blender contract. Either manually or via `./setup_blending.sh`.

Deposits a note of 50 tokens of a PSP token registered with an id 0:

```bash
cargo run --release -- --seed //Alice deposit 0 100
```

### What notes do I have to spend?

```bash
cargo run --release -- --seed //Alice show-assets 0
```

### Withdraw a note

Withdraws a note of 50 tokens of a PSP22 token registered under an id 0:

```bash
cargo run --release -- --seed //Alice withdraw  --deposit-id 0 --amount 50
```
