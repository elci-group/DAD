use anyhow::Result;

/// Compute context diff between current state and previous error state
pub fn compute_context_diff(
    _working_dir: &str,
    _prev_hash: Option<&str>,
) -> Result<Option<String>> {
    // TODO: Implement git diff or file-level diff
    // For now, return None
    Ok(None)
}

/// Extract relevant file changes from git
pub fn git_diff_summary(working_dir: &str) -> Result<Option<String>> {
    use std::process::Command;
    
    let output = Command::new("git")
        .args(&["diff", "--stat", "HEAD"])
        .current_dir(working_dir)
        .output()?;
    
    if output.status.success() {
        let diff = String::from_utf8_lossy(&output.stdout).to_string();
        if diff.is_empty() {
            Ok(None)
        } else {
            Ok(Some(diff))
        }
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_context_diff() {
        let result = compute_context_diff("/tmp", None);
        assert!(result.is_ok());
    }
}
