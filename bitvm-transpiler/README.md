# BitVM Transpiler

This is a **WIP** transpiler for converting other assembly languages to BitVM
assembly. This transpiler accepts different input file formats and ISA's.

## Running the Transpiler

```bash
cargo run --release $INPUTFILE
```

## Accepted Input File Formats

This transpiler can read and parse instructions only in executable and linkable
format (ELF).

## Supported Instruction Set Architectures and Extensions

Currently, BitVM tries to emulate rv32i instruction set. But this transpiler can
support more instruction sets or extension sets as input. Supported ISA's
resides in `src/*_parser`.

* rv32i + m extension

## References

* Carsten Munk's rv32i to BitVM transpiler - https://github.com/zippiehq/rv32i-to-bitvm/.

## License

This project is licensed under the GNU General Public License v3.0 - see the
[LICENSE](LICENSE) file for details. By using, distributing, or contributing to
this software, you agree to the terms and conditions of the GPLv3.
