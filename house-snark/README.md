# HouseSnark
_Brace yourself, verifier is coming_

This repository contains a few simple R1CS relations.
Each of them can generate outputs useful for interacting with `snarcos` pallet in `aleph-node`.

## Generating outputs

Outputs are generated as binary files with extension `.bytes`.
The artifacts include:
 - verifying key
 - proving key
 - proof
 - public input

For this, run:
```shell
> cargo run --release -- generate-keys --relation <relation-id>
> cargo run --release -- generate-proof --relation <relation-id> --proving-key-file <relation-id>.pk.bytes
```

For now, default public input will be returned together with a proof.

Currently supported relations are:
 - `xor`
 - `linear-equation`

The files will be named according to the pattern: `<relation-id>.(vk|pk|proof|input).bytes`.
They can be directly sent to the pallet.

**Note:** Currently, only Groth16 SNARKs are supported and used.

## Cleaning

In order to clean your directory from all outputs, run:
```shell
> cargo run --release -- red-wedding
```
