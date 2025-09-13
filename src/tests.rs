use super::*;

#[test]
fn test_args_update_flag() {
  // Simulate the case where update flag is passed.
  let args = Args {
      update: true,
  };
  // Test if the update flag is correctly passed
  assert!(args.update);
}

#[test]
fn test_args_no_update_flag() {
  // Simulate the case where no update flag is passed
  let args = Args {
      update: false,
  };
  // Test if the update flag is correctly set to false
  assert!(!args.update);
}

#[test]
fn test_check_file_exists() {
    let temp_file = Path::new("test_file.txt");

    // Create the file for testing
    fs::write(temp_file, "Test data").expect("Unable to write to file");

    // Assert that check_file succeeds
    assert!(check_file(temp_file).is_ok());

    // Clean up the file after test
    fs::remove_file(temp_file).expect("Unable to remove file");
}

#[test]
fn test_check_file_not_exists() {
    let missing_file = Path::new("non_existent_file.txt");

    // Assert that check_file returns an error
    let result = check_file(missing_file);
    assert!(result.is_err());

    // You can also check the error message contents if you want
    if let Err(e) = result {
        assert!(e.to_string().contains("Missing file or dir"));
    }
}


