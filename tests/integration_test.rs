use std::process::Command;

#[test]
fn test_valid_conversion() {
    // Test a valid px to em conversion using the CLI
    let output = Command::new("cargo")
        .args(&["run", "--", "16", "px", "-r", "16"])
        .output()
        .expect("Failed to execute process");

    // Verify the output contains the expected results
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("px: 16px"));
    assert!(stdout.contains("em: 1em"));
    assert!(stdout.contains("rem: 1rem"));
}

#[test]
fn test_valid_conversion_with_custom_root_font_size() {
    // Test a valid px to em conversion with a custom root font size (10)
    let output = Command::new("cargo")
        .args(&["run", "--", "20", "px", "-r", "10"])
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("px: 20px")); // 20px is unchanged
    assert!(stdout.contains("em: 2em")); // 20px = 2em at root font size of 10px
    assert!(stdout.contains("rem: 2rem")); // Same as em
}

#[test]
fn test_invalid_unit() {
    // Test an invalid unit using the CLI
    let output = Command::new("cargo")
        .args(&["run", "--", "24", "invalid_unit", "-r", "16"])
        .output()
        .expect("Failed to execute process");

    // Verify the error message is as expected
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Error: Unsupported unit: 'invalid_unit'."));
}

#[test]
fn test_list_units() {
    // Test listing available units
    let output = Command::new("cargo")
        .args(&["run", "--", "0", "px", "-r", "16"])
        .output()
        .expect("Failed to execute process");

    // Verify that all expected units are listed
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("px:"));
    assert!(stdout.contains("em:"));
    assert!(stdout.contains("rem:"));
}
