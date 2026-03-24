/// E2E tests untuk binary `awsx`
/// Menjalankan binary langsung dan memvalidasi output/exit code
use std::process::Command;
use std::env;
use std::fs;
use std::path::PathBuf;

fn awsx_with_config(config_path: Option<PathBuf>) -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_awsx"));
    if let Some(p) = config_path {
        cmd.env("AWSX_CONFIG_PATH", p);
    }
    cmd
}

// ─── Unit tests: pure logic ───────────────────────────────────────────────────

#[test]
fn test_match_score_exact() {
    assert_eq!(awsx::matching::match_score("my-profile", "my-profile"), 100);
}

#[test]
fn test_match_score_case_insensitive() {
    assert_eq!(awsx::matching::match_score("MyProfile", "myprofile"), 100);
}

#[test]
fn test_match_score_partial_tokens() {
    let score = awsx::matching::match_score("dev-api", "dev-api-staging");
    assert!(score > 0 && score < 100, "Expected score antara 0-100, Actual: {score}");
}

#[test]
fn test_match_score_no_match() {
    assert_eq!(awsx::matching::match_score("production", "dev-cluster"), 0);
}

#[test]
fn test_detect_environment_production() {
    assert_eq!(awsx::matching::detect_environment("my-prod-profile"), Some("production".to_string()));
    assert_eq!(awsx::matching::detect_environment("prd-service"), Some("production".to_string()));
}

#[test]
fn test_detect_environment_staging() {
    assert_eq!(awsx::matching::detect_environment("stg-api"), Some("staging".to_string()));
    assert_eq!(awsx::matching::detect_environment("staging-cluster"), Some("staging".to_string()));
}

#[test]
fn test_detect_environment_development() {
    assert_eq!(awsx::matching::detect_environment("dev-backend"), Some("development".to_string()));
}

#[test]
fn test_detect_environment_none() {
    assert_eq!(awsx::matching::detect_environment("my-service"), None);
}

// ─── E2E tests: binary CLI ────────────────────────────────────────────────────

fn temp_config_file(test_name: &str) -> PathBuf {
    let temp_dir = env::temp_dir().join(format!("awsx-test-{}-{}", std::process::id(), test_name));
    fs::create_dir_all(&temp_dir).ok();
    temp_dir.join("config.toml")
}

#[test]
fn e2e_help_flag() {
    let out = awsx_with_config(None).arg("--help").output().expect("gagal jalankan awsx");
    assert!(out.status.success(), "Expected exit 0, Actual: {}", out.status);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("awsx"), "Expected stdout mengandung 'awsx', Actual:\n{stdout}");
}

#[test]
fn e2e_version_flag() {
    let out = awsx_with_config(None).arg("--version").output().expect("gagal jalankan awsx");
    assert!(out.status.success(), "Expected exit 0, Actual: {}", out.status);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("awsx"), "Expected stdout mengandung nama app, Actual:\n{stdout}");
}

#[test]
fn e2e_list_subcommand() {
    let config_file = temp_config_file("list");
    let out = awsx_with_config(Some(config_file.clone())).arg("list").output().expect("gagal jalankan awsx list");
    assert!(out.status.success(), "Expected exit 0, Actual: {}", out.status);
    fs::remove_dir_all(config_file.parent().unwrap()).ok();
}

#[test]
fn e2e_save_and_list_context() {
    let config_file = temp_config_file("save_and_list");
    let save = awsx_with_config(Some(config_file.clone()))
        .args(["save", "e2e-test", "--aws-profile", "default", "--region", "ap-southeast-1"])
        .output()
        .expect("gagal jalankan awsx save");
    assert!(save.status.success(), "Expected save exit 0, Actual: {}", save.status);

    let list = awsx_with_config(Some(config_file.clone())).arg("list").output().expect("gagal jalankan awsx list");
    let stdout = String::from_utf8_lossy(&list.stdout);
    assert!(
        stdout.contains("e2e-test"),
        "Expected list mengandung 'e2e-test', Actual:\n{stdout}"
    );

    fs::remove_dir_all(config_file.parent().unwrap()).ok();
}

#[test]
fn e2e_delete_context() {
    let config_file = temp_config_file("delete_context");
    awsx_with_config(Some(config_file.clone()))
        .args(["save", "e2e-delete-me", "--aws-profile", "default"])
        .output()
        .expect("gagal save");

    let del = awsx_with_config(Some(config_file.clone()))
        .args(["delete", "e2e-delete-me"])
        .output()
        .expect("gagal jalankan awsx delete");
    assert!(del.status.success(), "Expected delete exit 0, Actual: {}", del.status);

    let list = awsx_with_config(Some(config_file.clone())).arg("list").output().expect("gagal list");
    let stdout = String::from_utf8_lossy(&list.stdout);
    assert!(
        !stdout.contains("e2e-delete-me"),
        "Expected 'e2e-delete-me' sudah terhapus, Actual:\n{stdout}"
    );

    fs::remove_dir_all(config_file.parent().unwrap()).ok();
}

#[test]
fn e2e_delete_nonexistent_context() {
    let config_file = temp_config_file("delete_nonexistent");
    let out = awsx_with_config(Some(config_file.clone()))
        .args(["delete", "context-yang-tidak-ada-xyz"])
        .output()
        .expect("gagal jalankan awsx delete");
    assert!(!out.status.success(), "Expected exit non-0 untuk context tidak ada, Actual: exit 0");

    fs::remove_dir_all(config_file.parent().unwrap()).ok();
}

#[test]
fn e2e_shell_hook_zsh() {
    let out = awsx_with_config(None).args(["shell-hook", "zsh"]).output().expect("gagal jalankan awsx shell-hook zsh");
    assert!(out.status.success(), "Expected exit 0, Actual: {}", out.status);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(!stdout.is_empty(), "Expected output tidak kosong, Actual: kosong");
}

#[test]
fn e2e_clear_outputs_unset() {
    let out = awsx_with_config(None).arg("clear").output().expect("gagal jalankan awsx clear");
    assert!(out.status.success(), "Expected exit 0, Actual: {}", out.status);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("unset AWS_PROFILE"),
        "Expected stdout mengandung 'unset AWS_PROFILE', Actual:\n{stdout}"
    );
}

#[test]
fn e2e_use_nonexistent_context() {
    let config_file = temp_config_file("use_nonexistent");
    let out = awsx_with_config(Some(config_file.clone()))
        .args(["use", "context-tidak-ada-xyz"])
        .output()
        .expect("gagal jalankan awsx use");
    assert!(!out.status.success(), "Expected exit non-0, Actual: exit 0");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(!stderr.is_empty(), "Expected stderr ada pesan error, Actual: kosong");

    fs::remove_dir_all(config_file.parent().unwrap()).ok();
}
