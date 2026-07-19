use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

// type 关键词创建别名
type TestResult = Result<(), Box<dyn std::error::Error>>;
#[test]
fn dies_no_args() -> TestResult {
    // cargo_bin -> 去寻找"name"的二进制可执行文件
    //assert() 不带参数运行
    //.failure() 必须失败，不失败panic
    //.stderr()检查标准错误流
    //.predicate::str::contains
    // 断言错误信息包含"USAGE"
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello").assert().success();
    // arg()传入参数
    // assert运行返回程序返回值
    // success断言返回值为1
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}
#[test]
fn hello1_no_newline() -> TestResult {
    run(&["Hello there", "-n"], "tests/expected/hello1.n.txt")
}
#[test]
fn hello2_no_newline() -> TestResult {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}
