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

### Example scenario (non-universal system)

```shell
> cargo run --release -- generate-keys   --system groth16 --relation <relation-id>
> cargo run --release -- generate-proof  --system groth16 --relation <relation-id> --proving-key-file <relation-id>.pk.bytes
```

### Example scenario (universal system)

```shell
> cargo run --release -- generate-srs             --system marlin
> cargo run --release -- generate-keys-from-srs   --system marlin --relation <relation-id> --srs-file marlin.srs.bytes
> cargo run --release -- generate-proof           --system marlin --relation <relation-id> --proving-key-file <relation-id>.pk.bytes
```

### Supported relations

Currently supported relations are:
 - `xor`
 - `linear-equation`

The files will be named according to the pattern: `<relation-id>.(vk|pk|proof|input).bytes`.
They can be directly sent to the pallet.

**Note:** Currently, only Groth16 and GM17 SNARKs are supported and used.

## Cleaning

In order to clean your directory from all outputs, run:
```shell
> cargo run --release -- red-wedding
```
