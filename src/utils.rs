use sha2::{Digest, Sha256};

pub fn sha256_hash(input: &str) -> String {
    // Create a Sha256 object
    let mut sha256 = Sha256::new();

    // Update the hasher with the input string
    sha256.update(input.as_bytes());

    // Finalize the hash and get the result as a byte array
    let hash_result = sha256.finalize();

    // Convert the byte array to a hexadecimal string
    let hash_string = hash_result
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>();

    hash_string
}
