pub mod simple_hasher {
    use sha2::Digest;

    fn hex_string(input: &[u8]) -> String {
        input
            .as_ref()
            .iter()
            .map(|b| format!("{:x}", b))
            .collect()
    }

    fn sha<T>(mut hasher: T, s: String) -> String where T: Digest {
        hasher.input(s.as_bytes());

        hex_string(&hasher.result())
    }

    pub fn sha224(s: String) -> String {
        sha(sha2::Sha224::new(), s)
    }

    pub fn sha256(s: String) -> String {
        sha(sha2::Sha256::new(), s)
    }

    pub fn sha384(s: String) -> String {
        sha(sha2::Sha384::new(), s)
    }

    pub fn sha5124(s: String) -> String {
        sha(sha2::Sha512::new(), s)
    }
}
