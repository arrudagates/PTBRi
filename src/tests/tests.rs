#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn basic() {
        if let Ok(_) = Command::new("cargo")
            .args(&["build", "--release", "--target-dir", "src/tests"])
            .output()
        {
            let output = Command::new("src/tests/release/ptbri")
                .arg("src/tests/basic.ptbr")
                .output()
                .expect("Failed to run ptbri");
            assert_eq!(output.stdout, "11\n5\n5 1\n6 84\n".as_bytes())
        } else {
            panic!("Cargo build failed");
        }
    }

    #[test]
    fn if_statement() {
        if let Ok(_) = Command::new("cargo")
            .args(&["build", "--release", "--target-dir", "src/tests"])
            .output()
        {
            let output = Command::new("src/tests/release/ptbri")
                .arg("src/tests/if_statement.ptbr")
                .output()
                .expect("Failed to run ptbri");
            assert_eq!(output.stdout, "\"true\"\n\"true\"\n\"true\"\n".as_bytes())
        } else {
            panic!("Cargo build failed");
        }
    }
}
