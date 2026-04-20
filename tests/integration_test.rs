use overlayfs_purge::run;
use std::path::Path;

fn run_purger() {
    run(
        Path::new("test-fixtures/integration_test/sysupgrade.conf"),
        Path::new("test-fixtures/integration_test/keep.d"),
        Path::new("tmp/lowerdir"),
        Path::new("tmp/upperdir"),
    );
}

fn setup_fakeroot() {
    if nix::unistd::getuid().as_raw() != 0 {
        println!("Running test with fakeroot.");
        let args: Vec<_> = std::env::args().collect();
        let mut command_builder = std::process::Command::new("fakeroot");
        let status = command_builder.args(args).status().unwrap();
        std::process::exit(status.code().unwrap());
    }
}

fn setup_test_data() {
    let status = std::process::Command::new("sh")
        .arg("test-fixtures/integration_test/setup.sh")
        .status()
        .expect("Failed to setup test.");
    assert!(status.success());
}

fn verify_test_data() {
    let status = std::process::Command::new("sh")
        .arg("test-fixtures/integration_test/verify.sh")
        .status()
        .expect("Failed to verify test.");
    assert!(status.success());
}

#[test]
fn integration_test() {
    setup_fakeroot();
    setup_test_data();
    run_purger();
    verify_test_data();
}

#[test]
fn test_cli_no_force_flag() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_overlayfs-purge"))
        .output()
        .expect("Failed to run binary");
    assert!(!output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Aborting"),
        "Expected 'Aborting' in stdout, got: {stdout}"
    );
}

#[test]
fn test_cli_wrong_positional_count() {
    for args in [
        vec!["-f", "only_one"],
        vec!["-f", "a", "b", "c"],
        vec!["-f", "a", "b", "c", "d", "e"],
    ] {
        let output = std::process::Command::new(env!("CARGO_BIN_EXE_overlayfs-purge"))
            .args(&args)
            .output()
            .expect("Failed to run binary");
        assert!(
            !output.status.success(),
            "Expected non-zero exit for args: {args:?}"
        );
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("Usage:"),
            "Expected 'Usage:' in stderr for args {args:?}, got: {stderr}"
        );
    }
}

#[test]
fn test_cli_custom_paths() {
    let _ = std::fs::remove_dir_all("tmp/cli_test");
    std::fs::create_dir_all("tmp/cli_test/lowerdir").unwrap();
    std::fs::create_dir_all("tmp/cli_test/upperdir").unwrap();
    std::fs::File::create("tmp/cli_test/upperdir/file_keep").unwrap();
    std::fs::File::create("tmp/cli_test/upperdir/file_remove").unwrap();

    let status = std::process::Command::new(env!("CARGO_BIN_EXE_overlayfs-purge"))
        .args([
            "test-fixtures/integration_test/sysupgrade.conf",
            "test-fixtures/integration_test/keep.d",
            "tmp/cli_test/lowerdir",
            "tmp/cli_test/upperdir",
            "-f",
        ])
        .status()
        .expect("Failed to run binary");

    assert!(status.success());
    assert!(
        std::path::Path::new("tmp/cli_test/upperdir/file_keep").exists(),
        "file_keep should be kept"
    );
    assert!(
        !std::path::Path::new("tmp/cli_test/upperdir/file_remove").exists(),
        "file_remove should be purged"
    );
}
