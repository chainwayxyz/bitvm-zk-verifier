# Currently RISC0 to Groth16

```bash
RISC0_DEV_MODE=0 cargo run --release
```

```bash
sudo docker run --rm -v /home/ekrem/bridge/risc0tobitvm/risc0tobitvm/work_dir:/mnt risc0-groth16-prover
```

```bash
make expected && ./bin/expected
```

### Groth16 Verifier

works for hardcoded

```sh
make groth16 && ./bin/groth16_verifier
```
