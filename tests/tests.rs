#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn basic() {
        if let Ok(_) = Command::new("cargo").args(&["build", "--release"]).output() {
            let output = Command::new("target/release/ptbri")
                .arg("tests/basic.ptbr")
                .output()
                .expect("Failed to run ptbri");
            assert_eq!(output.stdout, "11\n5\n5 1\n6 84\n".as_bytes())
        } else {
            panic!("Cargo build failed");
        }
    }

    #[test]
    fn if_statement() {
        if let Ok(_) = Command::new("cargo").args(&["build", "--release"]).output() {
            let output = Command::new("target/release/ptbri")
                .arg("tests/if_statement.ptbr")
                .output()
                .expect("Failed to run ptbri");
            assert_eq!(output.stdout, "true\ntrue\ntrue\n".as_bytes())
        } else {
            panic!("Cargo build failed");
        }
    }

    #[test]
    fn senao() {
        if let Ok(_) = Command::new("cargo").args(&["build", "--release"]).output() {
            let output = Command::new("target/release/ptbri")
                .arg("tests/senao.ptbr")
                .output()
                .expect("Failed to run ptbri");
            assert_eq!(output.stdout, "senão\n2\n".as_bytes())
        } else {
            panic!("Cargo build failed");
        }
    }

    #[test]
    fn while_statement() {
        if let Ok(_) = Command::new("cargo").args(&["build", "--release"]).output() {
            let output = Command::new("target/release/ptbri")
                .arg("tests/while_statement.ptbr")
                .output()
                .expect("Failed to run ptbri");
            assert_eq!(output.stdout, "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n".as_bytes())
        } else {
            panic!("Cargo build failed");
        }
    }

    #[test]
    fn math() {
        if let Ok(_) = Command::new("cargo").args(&["build", "--release"]).output() {
            let output = Command::new("target/release/ptbri")
                .arg("tests/math.ptbr")
                .output()
                .expect("Failed to run ptbri");
            assert_eq!(output.stdout, "a = 10, b = 2\na mais b =  12\na menos b =  8\na vezes b =  20\na dividido por b =  5\na = 10, b = 2.5\na mais b =  12.5\na menos b =  7.5\na vezes b =  25\na dividido por b =  4\n".as_bytes())
        } else {
            panic!("Cargo build failed");
        }
    }

    #[test]
    fn boolean() {
        if let Ok(_) = Command::new("cargo").args(&["build", "--release"]).output() {
            let output = Command::new("target/release/ptbri")
                .arg("tests/boolean.ptbr")
                .output()
                .expect("Failed to run ptbri");
            assert_eq!(
                output.stdout,
                "true\nfalse\nverdadeiro não é falso\n".as_bytes()
            )
        } else {
            panic!("Cargo build failed");
        }
    }

    #[test]
    fn function() {
        if let Ok(_) = Command::new("cargo").args(&["build", "--release"]).output() {
            let output = Command::new("target/release/ptbri")
                .arg("tests/function.ptbr")
                .output()
                .expect("Failed to run ptbri");
            assert_eq!(output.stdout, "2 4\n6\n".as_bytes())
        } else {
            panic!("Cargo build failed");
        }
    }
}
