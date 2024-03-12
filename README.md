# Currently RISC0 to Groth16

## To run the RISC0 prover:

```bash
RISC0_DEV_MODE=0 cargo run --release
```

## To Transpile to BitVM instruction sets

First follow the instructions in the repository https://github.com/sifive/riscv-llvm to install RISCV toolchain

Then

```bash
make mcl
make transpiler
```

Expected output:
```
Step count =  12568999252
obj/zkverifier result code: 1 0
```
