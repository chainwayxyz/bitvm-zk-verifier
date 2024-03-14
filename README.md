# BitVM ZK Verifier

To clone this repo with submodules:

```
git clone --recurse-submodules https://github.com/chainwayxyz/bitvm-zk-verifier.git
```


## To Transpile to BitVM instruction sets

We have our groth16 verifier here [`groth16-verifier/main.c`](groth16-verifier/main.c)

To transpile it to BitVM, first you need to install rv32i-to-bitvm

```
cd rv32i-to-bitvm
yarn
cd ..
```

Build the mcl library for pairing operations:
```
make mcl
```

To compile the groth16 verifier you will need the clang and riscv32 toolchain, follow the instructions in https://github.com/sifive/riscv-llvm

```
make zkverifier
```

Transpile to BitVM instruction set and save the program:
```
npx ts-node --files rv32i-to-bitvm/main.ts bin/zkverifier
```

Run the BitVM simulation:
```
cargo run --bin bitvm --release
```


## To run the RISC0 prover:

```bash
RISC0_DEV_MODE=0 cargo run --bin risc0tobitvm --release
```

This will generate a risc0 proof and save the input.json file to work_dir.

We already placed a valid proof for Bitcoin proof of work in work_dir/proof.json

In order to generate groth16 proofs, follow steps in [https://github.com/risc0/risc0/blob/main/compact_proof/README.md](https://github.com/risc0/risc0/blob/main/compact_proof/README.md).




Expected output:
```
step count: 17087788659
result: 1, 0
```


# TODO

- Skip initializations to reduce the step count
