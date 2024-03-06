# Currently RISC0 to Groth16

```bash
RISC0_DEV_MODE=0 cargo run --release
```

```bash
sudo docker run --rm -v /home/ekrem/bridge/risc0tobitvm/risc0tobitvm/work_dir:/mnt risc0-groth16-prover
```

### Groth16 Verifier

edit `groth16-verifier/last_two_constants.h`

generate proof data

```sh
python3 generate_proof_bytes.py work_dir/proof.json
```

works for hardcoded

```sh
make groth16 && ./bin/groth16_verifier
```
