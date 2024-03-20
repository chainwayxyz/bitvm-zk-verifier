use groth16::verify_groth16;
use groth16_methods::VERIFY_GROTH16_ID;

fn main() {
    let (receipt, _) = verify_groth16();
    receipt.verify(VERIFY_GROTH16_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct image ID?",
    );
}
