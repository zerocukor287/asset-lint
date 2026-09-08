use assert_cmd::Command;

// This file is for integration test.
// Test the validity of the calling arguments

#[test]
fn no_placeholders_accepts_multiple_patterns() {
    let mut cmd = Command::cargo_bin("asset-lint").unwrap();

    cmd.args(["--no-placeholders", ".*[Dd]ebug", ".*.psd"])
        .assert()
        .success();
}

#[test]
fn ignore_multiple_patterns() {
    let mut cmd = Command::cargo_bin("asset-lint").unwrap();

    cmd.args([
        "--ignore",
        ".*asset-lint.toml",
        ".*asset-lint.exe",
        "do-asset-lint.bat",
    ])
    .assert()
    .success();
}

#[test]
fn duplicates_different_calling_styles() {
    {
        let mut cmd = Command::cargo_bin("asset-lint").unwrap();

        cmd.args(["--assets-path", "./assets/"]).assert().success();
    }
    {
        let mut cmd = Command::cargo_bin("asset-lint").unwrap();
        cmd.args(["--assets-path", "./assets/", "--no-duplicates"])
            .assert()
            .failure(); // it finds a duplicate
    }
    {
        let mut cmd = Command::cargo_bin("asset-lint").unwrap();
        cmd.args(["--assets-path", "./assets/", "--no-duplicates", "true"])
            .assert()
            .failure(); // it finds a duplicate
    }
    {
        let mut cmd = Command::cargo_bin("asset-lint").unwrap();
        cmd.args(["--assets-path", "./assets/", "--no-duplicates", "false"])
            .assert()
            .success();
    }
}
