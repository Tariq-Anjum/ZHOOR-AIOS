use std::process::Command;

#[test]
fn zhoor_binary_runs_and_exits_cleanly() {
    let output = Command::new(env!("CARGO_BIN_EXE_zhoor"))
        .output()
        .expect("zhoor binary should execute");
    assert!(output.status.success());
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());
}
