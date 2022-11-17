# blender-cli
Blending assets with SNARKs from your CLI

## Dev instructions
1. Checkout `aleph-node` to `snarkeling` branch and build the node (`cargo build --release`).
2. Comment out invocation to `verify_deposit()` in `Blender` contract (`contracts/blender/contract.rs`).
3. Set up environment (run `contracts/setup_blending.sh`).
4. Copy the `Blender` contract address (the script above will be print it).
5. Go to this repo.
6. Setup your application data with the copied address like this:
```shell
cargo run --release -- set-contract-address 5GSoq77tJzcpUPXvJ1Bv48ZTwurtqoQKC9pou61Yk7kLBt8p
```
7. Deposit your money:
```shell
cargo run --release -- deposit 0 50
```
8. Hurray!
