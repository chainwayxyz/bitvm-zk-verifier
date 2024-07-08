use risc0_zkvm::{get_prover_server, recursion::identity_p254, Groth16Receipt, ProverOpts, SuccinctReceipt};
use risc0tobitvm::{setup, prove};
use risc0_groth16::to_json;
use std::{fs::{read_to_string, write, File}, io::{BufReader, BufWriter, Cursor, Write}, path::Path};

// use hello_world::multiply;
use bitcoin_pow::{calculate_pow, CALCULATE_POW_ELF};

// fn main() {
//     let mut template = read_to_string("templates/constants_template.h").unwrap();
//     setup(CALCULATE_POW_ELF, &mut template);
//     // let (receipt, _) = multiply(101, 31);
//     let (receipt, _) = calculate_pow();
//     // println!("receipt: {:#?}", receipt);
//     // prove(&receipt, &mut template);
//     let opts = ProverOpts::default();
//     let prover = get_prover_server(&opts).unwrap();
//     let composite_receipt = receipt.inner.composite().unwrap();
//     let succinct_receipt = prover.composite_to_succinct(&composite_receipt).unwrap();
//     // println!("succinct_receipt: {:#?}", succinct_receipt);
//     // let ident_receipt = identity_p254(&succinct_receipt).unwrap();
//     // println!("ident_receipt: {:#?}", ident_receipt);
//     // let seal_bytes = ident_receipt.get_seal_bytes();
//     // let seal_json = File::create(Path::new("./work_dir").join("input.json")).unwrap();
//     // let mut seal_reader = Cursor::new(&seal_bytes);
//     // to_json(&mut seal_reader, &seal_json).unwrap();
//     let groth16_receipt = prover.succinct_to_groth16(&succinct_receipt).unwrap();
//     println!("groth16_receipt: {:#?}", groth16_receipt);
//     let res = groth16_receipt.verify_integrity();
//     println!("res: {:#?}", res);
//     // let groth16_proof = groth16_receipt.;
//     // let groth16_json = File::create(Path::new("./work_dir").join("groth16_input.json")).unwrap();
//     // let mut seal_reader = Cursor::new(&groth16_proof);
//     // to_json(&mut seal_reader, &groth16_json).unwrap();
//     // write("groth16-verifier/constants.h", template).unwrap();
// }

fn main() {
    let mut template = read_to_string("templates/constants_template.h").unwrap();
    setup(CALCULATE_POW_ELF, &mut template);
    // let (receipt, _) = multiply(101, 31);
    let (receipt, _) = calculate_pow();
    prove(&receipt, &mut template);
    write("groth16-verifier/constants.h", template).unwrap();
}