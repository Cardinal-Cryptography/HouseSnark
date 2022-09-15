# HouseSnark
_Brace yourself, verifier is coming_

This repository contains a few simple R1CS relations.
Each of them can generate outputs useful for interacting with `snarcos` pallet in `aleph-node`.

## Generating outputs

Outputs are generated as binary files with extension `.bytes`.
The artifacts include:
 - verifying key
 - proof
 - public input

For this, run:
```shell
> ./target/release/house-snark generate --relation <relation-id>
```

Currently supported relations are:
 - `xor`
 - `merkle-tree`

You should receive three files: `<relation-id>.(vk|proof|input).bytes`, which then can be directly sent to the pallet.

**Note:** Currently, only Groth16 SNARKs are supported and used.
However, it should be easy to add e.g. GM17.

## Cleaning

In order to clean your directory from all outputs, run:
```shell
> ./target/release/house-snark red-wedding
```
