use sha1::{Digest, Sha1};

/// Compute deterministic error hash using SHA1 over cmd + stderr signature + env hash
pub fn compute_error_hash(cmd: &str, stderr: &str, env_signature: &str) -> String {
    let stderr_sig = stderr_signature(stderr);
    let mut hasher = Sha1::new();
    hasher.update(cmd.as_bytes());
    hasher.update(stderr_sig.as_bytes());
    hasher.update(env_signature.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

/// Reduce stderr to a deterministic signature (first N chars + lines hashed)
fn stderr_signature(stderr: &str) -> String {
    const N: usize = 256;
    let prefix = &stderr.chars().take(N).collect::<String>();
    // simple hash of lines to avoid over-sensitivity to long traces
    let mut line_hasher = Sha1::new();
    for line in stderr.lines().take(50) { // cap to first 50 lines
        line_hasher.update(line.as_bytes());
    }
    let lines_hash = hex::encode(line_hasher.finalize());
    format!("{}:{}", prefix, lines_hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_determinism() {
        let h1 = compute_error_hash("cmd", "err", "env");
        let h2 = compute_error_hash("cmd", "err", "env");
        assert_eq!(h1, h2);
    }
}
