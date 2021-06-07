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
            assert_eq!(output.stdout, "true\ntrue\ntrue\ntrue\n".as_bytes())
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

    #[test]
    fn function_return() {
        if let Ok(_) = Command::new("cargo").args(&["build", "--release"]).output() {
            let output = Command::new("target/release/ptbri")
                .arg("tests/function_return.ptbr")
                .output()
                .expect("Failed to run ptbri");
            assert_eq!(output.stdout, "soma de  2  e  8\n10\n0\n10\n".as_bytes())
        } else {
            panic!("Cargo build failed");
        }
    }

    #[test]
    fn fibonacci_iterative() {
        if let Ok(_) = Command::new("cargo").args(&["build", "--release"]).output() {
            let output = Command::new("target/release/ptbri")
                .arg("tests/fibonacci_iterative.ptbr")
                .output()
                .expect("Failed to run ptbri");
            assert_eq!(output.stdout, "55\n".as_bytes())
        } else {
            panic!("Cargo build failed");
        }
    }

    #[test]
    fn function_recursion() {
        if let Ok(_) = Command::new("cargo").args(&["build", "--release"]).output() {
            let output = Command::new("target/release/ptbri")
                .arg("tests/function_recursion.ptbr")
                .output()
                .expect("Failed to run ptbri");
            assert_eq!(output.stdout, "2\n".as_bytes())
        } else {
            panic!("Cargo build failed");
        }
    }

    #[test]
    fn hanoi_towers_recursion() {
        if let Ok(_) = Command::new("cargo").args(&["build", "--release"]).output() {
            let output = Command::new("target/release/ptbri")
                .arg("tests/hanoi_towers_recursion.ptbr")
                .output()
                .expect("Failed to run ptbri");
            assert_eq!(output.stdout, "move the disk 1 from S to D\nmove the disk 2 from S to A\nmove the disk 1 from D to A\nmove the disk 3 from S to D\nmove the disk 1 from A to S\nmove the disk 2 from A to D\nmove the disk 1 from S to D\n".as_bytes())
        } else {
            panic!("Cargo build failed");
        }
    }

    #[test]
    fn ackermann() {
        if let Ok(_) = Command::new("cargo").args(&["build", "--release"]).output() {
            let output = Command::new("target/release/ptbri")
                .arg("tests/ackermann.ptbr")
                .output()
                .expect("Failed to run ptbri");
            assert_eq!(output.stdout, "509\n".as_bytes())
        } else {
            panic!("Cargo build failed");
        }
    }

    #[test]
    fn comments() {
        if let Ok(_) = Command::new("cargo").args(&["build", "--release"]).output() {
            let output = Command::new("target/release/ptbri")
                .arg("tests/comments.ptbr")
                .output()
                .expect("Failed to run ptbri");
            assert_eq!(output.stdout, "a é 5 e b é 10\n".as_bytes())
        } else {
            panic!("Cargo build failed");
        }
    }

    #[test]
    fn nested_function() {
        if let Ok(_) = Command::new("cargo").args(&["build", "--release"]).output() {
            let output = Command::new("target/release/ptbri")
                .arg("tests/nested_function.ptbr")
                .output()
                .expect("Failed to run ptbri");
            assert_eq!(output.stdout, "1 + 2: 3\n".as_bytes())
        } else {
            panic!("Cargo build failed");
        }
    }
}

mod errors {
    use std::process::Command;

    #[test]
    fn undefined_var() {
        if let Ok(_) = Command::new("cargo").args(&["build", "--release"]).output() {
            let output = Command::new("target/release/ptbri")
                .arg("tests/errors/undefined_var.ptbr")
                .output()
                .expect("Failed to run ptbri");
            assert_eq!(
                output.stderr,
                "Error: Variable \"a\" not defined\n".as_bytes()
            )
        } else {
            panic!("Cargo build failed");
        }
    }

    #[test]
    fn undefined_function() {
        if let Ok(_) = Command::new("cargo").args(&["build", "--release"]).output() {
            let output = Command::new("target/release/ptbri")
                .arg("tests/errors/undefined_function.ptbr")
                .output()
                .expect("Failed to run ptbri");
            assert_eq!(
                output.stderr,
                "Error: Function \"teste\" not defined\n".as_bytes()
            )
        } else {
            panic!("Cargo build failed");
        }
    }

    #[test]
    fn not_enough_args() {
        if let Ok(_) = Command::new("cargo").args(&["build", "--release"]).output() {
            let output = Command::new("target/release/ptbri")
                .arg("tests/errors/not_enough_args.ptbr")
                .output()
                .expect("Failed to run ptbri");
            assert_eq!(
                output.stderr,
                "Error: Function teste expected 2 arguments but 1 was supplied\n".as_bytes()
            )
        } else {
            panic!("Cargo build failed");
        }
    }

    #[test]
    fn illegal_operation() {
        if let Ok(_) = Command::new("cargo").args(&["build", "--release"]).output() {
            let output = Command::new("target/release/ptbri")
                .arg("tests/errors/illegal_operation.ptbr")
                .output()
                .expect("Failed to run ptbri");
            assert_eq!(
                output.stderr,
                "Error: Cannot perform addition, between types any and bool\n".as_bytes()
            )
        } else {
            panic!("Cargo build failed");
        }
    }

    #[test]
    fn pest_parse_error() {
        if let Ok(_) = Command::new("cargo").args(&["build", "--release"]).output() {
            let output = Command::new("target/release/ptbri")
                .arg("tests/errors/pest_parse_error.ptbr")
                .output()
                .expect("Failed to run ptbri");
            assert_eq!(
                output.stderr,
                "Error: Parsing failed, reason:\n --> 1:1\n  |\n1 | mostr a mais a\n  | ^---\n  |\n  = expected EOI or line\n".as_bytes()
            )
        } else {
            panic!("Cargo build failed");
        }
    }

    #[test]
    fn incomplete_expression() {
        if let Ok(_) = Command::new("cargo").args(&["build", "--release"]).output() {
            let output = Command::new("target/release/ptbri")
                .arg("tests/errors/incomplete_expression.ptbr")
                .output()
                .expect("Failed to run ptbri");
            assert_eq!(
                output.stderr,
                "Error: Parsing failed, reason:\n --> 1:15\n  |\n1 | mostre 1 mais \n  |               ^---\n  |\n  = expected ident, function_call, verdadeiro, falso, integer, float, or string\n".as_bytes()
            )
        } else {
            panic!("Cargo build failed");
        }
    }

    #[test]
    fn infinite_recursion() {
        if let Ok(_) = Command::new("cargo").args(&["build", "--release"]).output() {
            let output = Command::new("target/release/ptbri")
                .arg("tests/errors/infinite_recursion.ptbr")
                .output()
                .expect("Failed to run ptbri");
            assert_eq!(
                output.stderr,
                "Error: Reached recursion limit of 4000\n".as_bytes()
            )
        } else {
            panic!("Cargo build failed");
        }
    }
}
