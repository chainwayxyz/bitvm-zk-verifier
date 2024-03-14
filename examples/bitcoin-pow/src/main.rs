use bitcoin_pow::calculate_pow;
use bitcoin_pow_methods::CALCULATE_POW_ID;

fn main() {
    // Pick two numbers
    let (receipt, _) = calculate_pow();

    // Here is where one would send 'receipt' over the network...

    // Verify receipt, panic if it's wrong
    receipt.verify(CALCULATE_POW_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct image ID?",
    );
}
