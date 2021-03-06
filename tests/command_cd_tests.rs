mod helpers;

use helpers::in_directory as cwd;

#[test]
fn cd_directory_not_found() {
    nu_error!(output, cwd("tests/fixtures"), "cd dir_that_does_not_exist");

    assert!(output.contains("dir_that_does_not_exist"));
    assert!(output.contains("directory not found"));
}
