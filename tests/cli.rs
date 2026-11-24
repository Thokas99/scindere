use std::process::Command;

#[test]
fn cli_uses_scindere_name_and_exposes_required_commands() {
    let output = Command::new(env!("CARGO_BIN_EXE_scindere"))
        .arg("--help")
        .output()
        .expect("run Scindere help");
    assert!(output.status.success());
    let help = String::from_utf8(output.stdout).unwrap();
    assert!(help.contains("scindere"));
    assert!(help.contains("run"));
    assert!(help.contains("write-output"));
}

#[test]
fn cli_reports_initial_development_version() {
    let output = Command::new(env!("CARGO_BIN_EXE_scindere"))
        .arg("--version")
        .output()
        .expect("run Scindere version");
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap().trim(),
        "scindere 0.1.0"
    );
}
