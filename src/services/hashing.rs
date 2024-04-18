use sha2::{Digest, Sha256};

pub fn generate_hashed_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    let hashed_password = hasher.finalize();

    // Convert the hashed password to hexadecimal string
    let hashed_password_hex = format!("{:x}", hashed_password);
    hashed_password_hex
}
