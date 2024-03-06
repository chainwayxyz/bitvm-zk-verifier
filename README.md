# Currently RISC0 to Groth16

```bash
RISC0_DEV_MODE=0 cargo run --release
```

```bash
sudo docker run --rm -v /home/ekrem/bridge/risc0tobitvm/risc0tobitvm/work_dir:/mnt risc0-groth16-prover
```

### Groth16 Verifier

```sh
git clone https://github.com/herumi/mcl
cd mcl
make -j4
cd ..
```

```sh
make groth16 && ./bin/groth16_verifier
```
