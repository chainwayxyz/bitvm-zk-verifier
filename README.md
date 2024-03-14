# BitVM ZK Verifier
Wit the goal of **being able to prove anything on Bitcoin**, this repository is a toolkit to generate a Groth16 verifier in BitVM. Full flow includes:

1. Creating STARK proof from a RISC Zero guest program (written in Rust)
2. Wrapping the STARK proof into a Groth16 proof
3. Crafting its corresponding Groth16 verifier in C
4. Compiling the verifier to rv32i instruction set
5. Transpiling the verifier from rv32i to BitVM instruction set
6. Running the BitVM simulation (To be run on-chain once BitVM is ready)

> [!WARNING] 
> BitVM ZK Verifier is still work-in-progress. It has not been audited and should not be used in production under any circumstances. It also requires a full BitVM implementation to be run fully on-chain.

## Instructions
To clone this repo with submodules:

```
git clone --recurse-submodules https://github.com/chainwayxyz/bitvm-zk-verifier.git
```


### Transpile The Verifier into BitVM Instruction Set

We have an example groth16 verifier here [`groth16-verifier/main.c`](groth16-verifier/main.c)

To transpile it to BitVM, you need to install rv32i-to-bitvm transpiler first:

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

Transpile it to the BitVM instruction set and save the program:
```
npx ts-node --files rv32i-to-bitvm/main.ts bin/zkverifier
```

Run the BitVM simulation:
```
cargo run --bin bitvm --release
```


Expected output:
```
step count: 17087788659
result: 1, 0
```

### To Generate Your Own Proof in RISC Zero:

```bash
RISC0_DEV_MODE=0 cargo run --bin risc0tobitvm --release
```

This will generate a RISCZero proof and save the input.json file to work_dir.

We already placed a valid proof for Bitcoin proof of work in `work_dir/proof.json`

In order to generate groth16 proofs, follow steps in [RISCZero repository](https://github.com/risc0/risc0/blob/main/compact_proof/README.md).


## TODOs

- Skip initializations to reduce the step count
- 
## License
This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details. By using, distributing, or contributing to this software, you agree to the terms and conditions of the GPLv3.