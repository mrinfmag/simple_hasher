/// Simple Hasher module
pub mod simple_hasher {
    use sha2::Digest;

    /// Performs the conversion from `u8` slice to `String`
    pub fn hex_string(input: &[u8]) -> String {
        input
            .as_ref()
            .iter()
            .map(|byte| format!("{:x}", byte))
            .collect()
    }

    /// Generic hashing function
    fn sha<T>(mut hasher: T, data: &[u8]) -> &[u8] where T: Digest {
        hasher.input(str.as_bytes());
//        hex_string(&hasher.result())
        &hasher.result()
    }

    /* Raw data hashing functions */

    /// SHA-224 raw data hashing function
    pub fn sha224(s: &[u8]) -> &[u8] {
        sha(sha2::Sha224::new(), d)
    }

    /// SHA-256 raw data hashing function
    pub fn sha256(s: &[u8]) -> &[u8] {
        sha(sha2::Sha256::new(), d)
    }

    /// SHA-384 raw data hashing function
    pub fn sha384(s: &[u8]) -> &[u8] {
        sha(sha2::Sha384::new(), d)
    }

    /// SHA-512 raw data hashing function
    pub fn sha512(s: &[u8]) -> &[u8] {
        sha(sha2::Sha512::new(), d)
    }

    /* String hashing functions */

    /// SHA-224 string hashing function
    pub fn string_sha224(s: String) -> &[u8] {
        sha224(s.as_bytes())
    }

    /// SHA-256 string hashing function
    pub fn string_sha256(s: String) -> &[u8] {
        sha256(s.as_bytes())
    }

    /// SHA-384 string hashing function
    pub fn string_sha384(s: String) -> &[u8] {
        sha384(s.as_bytes())
    }

    /// SHA-512 string hashing function
    pub fn string_sha512(s: String) -> &[u8] {
        sha512(s.as_bytes())
    }
}
