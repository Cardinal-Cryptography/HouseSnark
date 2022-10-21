# `snarkxt` - your client for `pallet_snarcos` 🌈

## What is it?

This is a `subxt`-based CLI tool for interacting with `pallet_snarcos`.

## How to use it? - Example scenario

```shell
# Most likely, the first thing you need to do is to generate some snarkish data using `house-snark` tool.
 
# Let's assume that we start in the root directory of this repo.
# Generate data:
pushd house-snark/
cargo run --release -- generate-keys --relation xor
cargo run --release -- generate-proof --relation xor --proving-key-file xor.groth.pk.bytes

# Go to the `snarkxt` directory.
pushd ../snarkxt/

# Firstly, store the key in pallet's storage.
cargo run --release -- store-key \
  --identifier yeah \
  --vk-file ../house-snark/xor.groth.vk.bytes

# Secondly, run proof verification.
cargo run --release -- verify \
  --identifier yeah \
  --proof-file ../house-snark/xor.groth.proof.bytes \
  --input-file ../house-snark/xor.groth.public_input.bytes \
  --system groth16
```

## How to use it? - Full manual
```shell
USAGE:
    snarkxt [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -h, --help               Print help information
        --node <NODE>        WS endpoint address of the node to connect to [default:
                             ws://127.0.0.1:9944]
        --signer <SIGNER>    Seed of the submitting account [default: //Alice]

SUBCOMMANDS:
    help         Print this message or the help of the given subcommand(s)
    store-key    Store a verification key under an identifier in the pallet's storage
    verify       Verify a proof against public input with a stored verification key

------------------------------------------------------------------------------------------

snarkxt-store-key 
Store a verification key under an identifier in the pallet's storage

USAGE:
    snarkxt store-key --identifier <IDENTIFIER> --vk-file <VK_FILE>

OPTIONS:
    -h, --help                       Print help information
        --identifier <IDENTIFIER>    
        --vk-file <VK_FILE>          Path to a file containing the verification key

------------------------------------------------------------------------------------------

snarkxt-verify 
Verify a proof against public input with a stored verification key

USAGE:
    snarkxt verify --identifier <IDENTIFIER> --proof-file <PROOF_FILE> --input-file <INPUT_FILE> --system <SYSTEM>

OPTIONS:
    -h, --help                       Print help information
        --identifier <IDENTIFIER>    
        --input-file <INPUT_FILE>    Path to a file containing the public input
        --proof-file <PROOF_FILE>    Path to a file containing the proof
        --system <SYSTEM>            Which proving system should be used
```

## What to do if the compilation process fails?

Most likely, the runtime API has changed.
In that case, you need to regenerate the [`aleph_api` module](src/aleph_api.rs).
Follow the instructions from https://docs.substrate.io/reference/command-line-tools/subxt/:
```shell
# Let's assume that we have `subxt` already installed and a running node on `http://localhost:9933`.
subxt metadata > /tmp/metadata.scale
subxt codegen --derive Clone Eq PartialEq Debug -f /tmp/metadata.scale > ./src/aleph_api.rs
cargo fmt
```
