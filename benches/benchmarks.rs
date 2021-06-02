use std::process::Command;

use criterion::{criterion_group, criterion_main, Criterion};

fn basic() {
    let output = Command::new("target/release/ptbri")
        .arg("tests/basic.ptbr")
        .output()
        .expect("Failed to run ptbri");
    assert_eq!(output.stdout, "11\n5\n5 1\n6 84\n".as_bytes())
}

fn if_statement() {
    let output = Command::new("target/release/ptbri")
        .arg("tests/if_statement.ptbr")
        .output()
        .expect("Failed to run ptbri");
    assert_eq!(output.stdout, "true\ntrue\ntrue\ntrue\n".as_bytes())
}

fn senao() {
    let output = Command::new("target/release/ptbri")
        .arg("tests/senao.ptbr")
        .output()
        .expect("Failed to run ptbri");
    assert_eq!(output.stdout, "senão\n2\n".as_bytes())
}

fn while_statement() {
    let output = Command::new("target/release/ptbri")
        .arg("tests/while_statement.ptbr")
        .output()
        .expect("Failed to run ptbri");
    assert_eq!(output.stdout, "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n".as_bytes())
}

fn math() {
    let output = Command::new("target/release/ptbri")
        .arg("tests/math.ptbr")
        .output()
        .expect("Failed to run ptbri");
    assert_eq!(output.stdout, "a = 10, b = 2\na mais b =  12\na menos b =  8\na vezes b =  20\na dividido por b =  5\na = 10, b = 2.5\na mais b =  12.5\na menos b =  7.5\na vezes b =  25\na dividido por b =  4\n".as_bytes())
}

fn boolean() {
    let output = Command::new("target/release/ptbri")
        .arg("tests/boolean.ptbr")
        .output()
        .expect("Failed to run ptbri");
    assert_eq!(
        output.stdout,
        "true\nfalse\nverdadeiro não é falso\n".as_bytes()
    )
}

fn function() {
    let output = Command::new("target/release/ptbri")
        .arg("tests/function.ptbr")
        .output()
        .expect("Failed to run ptbri");
    assert_eq!(output.stdout, "2 4\n6\n".as_bytes())
}

fn function_return() {
    let output = Command::new("target/release/ptbri")
        .arg("tests/function_return.ptbr")
        .output()
        .expect("Failed to run ptbri");
    assert_eq!(output.stdout, "soma de  2  e  8\n10\n0\n10\n".as_bytes())
}

fn fibonacci_iterative() {
    let output = Command::new("target/release/ptbri")
        .arg("tests/fibonacci_iterative.ptbr")
        .output()
        .expect("Failed to run ptbri");
    assert_eq!(output.stdout, "55\n".as_bytes())
}

fn function_recursion() {
    let output = Command::new("target/release/ptbri")
        .arg("tests/function_recursion.ptbr")
        .output()
        .expect("Failed to run ptbri");
    assert_eq!(output.stdout, "2\n".as_bytes())
}

fn hanoi_towers_recursion() {
    let output = Command::new("target/release/ptbri")
        .arg("tests/hanoi_towers_recursion.ptbr")
        .output()
        .expect("Failed to run ptbri");
    assert_eq!(output.stdout, "move the disk 1 from S to D\nmove the disk 2 from S to A\nmove the disk 1 from D to A\nmove the disk 3 from S to D\nmove the disk 1 from A to S\nmove the disk 2 from A to D\nmove the disk 1 from S to D\n".as_bytes())
}

fn ackermann() {
    let output = Command::new("target/release/ptbri")
        .arg("tests/ackermann.ptbr")
        .output()
        .expect("Failed to run ptbri");
    assert_eq!(output.stdout, "509\n".as_bytes())
}

fn nested_function() {
    let output = Command::new("target/release/ptbri")
        .arg("tests/nested_function.ptbr")
        .output()
        .expect("Failed to run ptbri");
    assert_eq!(output.stdout, "1 + 2: 3\n".as_bytes())
}

fn criterion_benchmark(c: &mut Criterion) {
    if let Ok(_) = Command::new("cargo").args(&["build", "--release"]).output() {
        c.bench_function("basic", |b| b.iter(|| basic()));
        c.bench_function("if_statement", |b| b.iter(|| if_statement()));
        c.bench_function("senao", |b| b.iter(|| senao()));
        c.bench_function("while_statement", |b| b.iter(|| while_statement()));
        c.bench_function("math", |b| b.iter(|| math()));
        c.bench_function("boolean", |b| b.iter(|| boolean()));
        c.bench_function("function", |b| b.iter(|| function()));
        c.bench_function("function_return", |b| b.iter(|| function_return()));
        c.bench_function("fibonacci_iterative", |b| b.iter(|| fibonacci_iterative()));
        c.bench_function("function_recursion", |b| b.iter(|| function_recursion()));
        c.bench_function("hanoi_towers_recursion", |b| {
            b.iter(|| hanoi_towers_recursion())
        });
        c.bench_function("ackermann", |b| b.iter(|| ackermann()));
        c.bench_function("nested_function", |b| b.iter(|| nested_function()));
    } else {
        panic!("Cargo build failed");
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
