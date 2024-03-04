# Currently RISC0 to Groth16

```bash
RISC0_DEV_MODE=0 cargo run --release
```

```bash
sudo docker run --rm -v /home/ekrem/bridge/risc0tobitvm/risc0tobitvm/work_dir:/mnt risc0-groth16-prover
```

```bash
gcc groth16-verifier/main.c groth16-verifier/sha256.c -I groth16-verifier/ -oa && ./a
```