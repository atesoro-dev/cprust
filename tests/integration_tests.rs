use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_cprust_some_patterns() {
    let cases = vec![
        (vec!["a.txt", "b.txt"], true),
        (vec!["a", "b"], true),
        
        (vec!["a.txt", "b.txt", "c.txt"], false),
    ];

    for (args, should_succeed) in cases {
        let temp_dir = tempdir().unwrap();
        
        for arg in &args {
            let file_path = temp_dir.path().join(arg);
            if arg.ends_with('/') {
                fs::create_dir_all(&file_path).unwrap();
            } else {
                fs::write(&file_path, "test data").unwrap();
            }
        }

        let mut cmd = Command::cargo_bin("cprust").unwrap();
        cmd.current_dir(temp_dir.path()).args(&args);

        let assert = cmd.assert();
        if should_succeed {
            assert.success();
            let last_arg = args.last().unwrap();
            let dest_path = temp_dir.path().join(last_arg);
            assert!(dest_path.exists(), "Dest file {:?} should exist", dest_path);
        } else {
            assert.failure();
        }
    }
}