# HouseSnark
_Brace yourself, verifier is coming_

This repository contains a few simple R1CS relations.
Each of them can generate outputs useful for interacting with `snarcos` pallet in `aleph-node`.

## Generating outputs

Outputs are generated as binary files with extension `.bytes`.
The artifacts include:
 - SRS
 - verifying key
 - proving key
 - proof
 - public input

For now, default public input is returned together with a proof.

### Example scenarios (non-universal system)

XOR relation example

```shell
> cargo run --release -- generate-keys --system groth16 xor --public-xoree 2 --private-xoree 3 --result 1
> cargo run --release -- generate-proof  --system groth16 --proving-key-file xor.groth16.pk.bytes xor --public-xoree 2 --private-xoree 3 --result 1
```

Linear equation relation example

```shell
> cargo run --release -- generate-keys --system groth16 linear-equation --a 2 --x 7 --b 5 --y 19
> cargo run --release -- generate-proof  --system groth16 --proving-key-file linear_equation.groth16.pk.bytes linear-equation --a 2 --x 7 --b 5 --y 19
```

Merkle tree relation example

```
cargo run --release -- generate-keys --system groth16 merkle-tree --leaf 1 --leaves 0,1,2,3,4,5,6,7 --seed deadbeef
cargo run --release -- generate-proof  --system groth16 --proving-key-file merkle_tree.groth16.pk.bytes merkle-tree --leaf 1 --leaves 0,1,2,3,4,5,6,7 --seed deadbeef

```

### Example scenario (universal system)

```shell
> cargo run --release -- generate-srs             --system marlin
> cargo run --release -- generate-keys-from-srs   --system marlin --srs-file marlin.srs.bytes <relation-id> <arguments>
> cargo run --release -- generate-proof           --system marlin --proving-key-file <relation-id>.pk.bytes <relation-id> <arguments>
```

### Supported relations

Currently supported relations are:
 - `xor`
 - `linear-equation`
 - `merkle-tree`

The files will be named according to the pattern: `<relation-id>.(vk|pk|proof|input).bytes`.
They can be directly sent to the pallet.

**Note:** Currently, only Groth16 and GM17 SNARKs are supported and used.

## Cleaning

In order to clean your directory from all outputs, run:
```shell
> cargo run --release -- red-wedding
```
