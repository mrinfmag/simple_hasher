/// Simple Hasher module
pub mod simple_hasher {
    use sha2::Digest;

    fn hex_string(input: &[u8]) -> String {
        input
            .as_ref()
            .iter()
            .map(|b| format!("{:x}", b))
            .collect()
    }

    /// Generic hashing function
    fn sha<T>(mut hasher: T, str: String) -> String where T: Digest {
        hasher.input(str.as_bytes());
        hex_string(&hasher.result())
    }

    /// SHA-224 hashing function
    pub fn sha224(s: String) -> String {
        sha(sha2::Sha224::new(), s)
    }

    /// SHA-256 hashing function
    pub fn sha256(s: String) -> String {
        sha(sha2::Sha256::new(), s)
    }

    /// SHA-384 hashing function
    pub fn sha384(s: String) -> String {
        sha(sha2::Sha384::new(), s)
    }

    /// SHA-512 hashing function
    pub fn sha512(s: String) -> String {
        sha(sha2::Sha512::new(), s)
    }
}
