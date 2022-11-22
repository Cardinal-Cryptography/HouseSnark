# House Snark
_Brace yourselves, verifier is coming_

This repository contains three tools:
 - [`house-snark`](house-snark/) - It contains a collection of a few R1CS relations.
You can pick out one of them, choose your favorite SNARK system and generate outputs useful for interacting with `snarcos` pallet in `aleph-node`.
This includes verifying and proving keys, serialized public input and of course a proof.
 - [`snarkxt`](snarkxt/) - A CLI tool that enables interaction with `snarcos` pallet - you can directly register a verifying key or pass a proof to verify.
 - [`blender-cli`](blender-cli/) - A CLI tool that allows you to interact with the Blender contract.
It will keep your secrets in a local encrypted file and generate the proofs for you as well.

Checkout inner `README.md` files for more details.
