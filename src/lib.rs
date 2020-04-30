/// Simple Hasher module
pub mod simple_hasher {
    type _Data = /*&'static [u8]*/ Vec<u8>;
    use sha2::Digest;

    /// Performs the conversion from `u8` slice to `String`
    pub fn hex_string(input: _Data) -> String {
        input
            .iter()
            .map(|byte| format!("{:x}", byte))
            .collect()
    }

    /// Generic hashing function
    fn sha<T>(mut hasher: T, data: _Data) -> _Data where T: Digest {
        hasher.input(data.as_slice());
//        hex_string(&hasher.result())
        hasher
            .result()
            .to_vec()
    }

    /* Raw data hashing functions */

    /// SHA-224 raw data hashing function
    pub fn sha224(d: _Data) -> _Data {
        sha(sha2::Sha224::new(), d)
    }

    /// SHA-256 raw data hashing function
    pub fn sha256(d: _Data) -> _Data {
        sha(sha2::Sha256::new(), d)
    }

    /// SHA-384 raw data hashing function
    pub fn sha384(d: _Data) -> _Data {
        sha(sha2::Sha384::new(), d)
    }

    /// SHA-512 raw data hashing function
    pub fn sha512(d: _Data) -> _Data {
        sha(sha2::Sha512::new(), d)
    }

    /* String hashing functions */

    /// SHA-224 string hashing function
    pub fn string_sha224(s: String) -> _Data {
        sha224(s.as_bytes().to_vec())
    }

    /// SHA-256 string hashing function
    pub fn string_sha256(s: String) -> _Data {
        sha256(s.as_bytes().to_vec())
    }

    /// SHA-384 string hashing function
    pub fn string_sha384(s: String) -> _Data {
        sha384(s.as_bytes().to_vec())
    }

    /// SHA-512 string hashing function
    pub fn string_sha512(s: String) -> _Data {
        sha512(s.as_bytes().to_vec())
    }
}