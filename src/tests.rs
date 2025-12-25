use super::*;
use std::fs;
use std::path::Path;
use clap::Parser; // Needed for Args::parse_from

// Test CLI args parsing
#[test]
fn test_args_update_flag() {
    let args = crate::cli::Args::parse_from(["test-bin", "--update"]);
    assert!(args.update);
    assert!(!args.remove);
}

#[test]
fn test_args_no_update_flag() {
    // No flags: both update and remove should be false;
    // cli::run() enforces "at least one", but Args itself parses fine.
    let args = crate::cli::Args::parse_from(["test-bin"]);
    assert!(!args.update);
    assert!(!args.remove);
}

// Test file checking functions
#[test]
fn test_check_file_exists() {
    let temp_file = Path::new("test_file.txt");

    fs::write(temp_file, "Test data").unwrap();
    assert!(check_file(temp_file).is_ok());
    fs::remove_file(temp_file).unwrap();
}

#[test]
fn test_check_file_not_exists() {
    let missing_file = Path::new("non_existent_file.txt");

    let result = check_file(missing_file);
    assert!(result.is_err());

    if let Err(e) = result {
        assert!(e.to_string().contains("Missing file or dir"));
    }
}

