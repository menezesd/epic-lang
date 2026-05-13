use std::fs;
use std::path::Path;
use std::process::Command;

#[test]
fn rust_tree_walking_interpreter_matches_golden_outputs() {
    run_golden_suite(&[]);
}

#[test]
fn rust_analyzing_interpreter_matches_golden_outputs() {
    run_golden_suite(&["--analyze"]);
}

fn run_golden_suite(extra_args: &[&str]) {
    let exe = env!("CARGO_BIN_EXE_epic-lang-rs");
    let mut total = 0usize;

    for entry in fs::read_dir("tests").expect("read tests directory") {
        let path = entry.expect("read test entry").path();
        if path.extension().and_then(|s| s.to_str()) != Some("epic") {
            continue;
        }

        total += 1;
        let expected_path = path.with_extension("out");
        let expected = fs::read_to_string(&expected_path)
            .unwrap_or_else(|_| panic!("read {}", expected_path.display()));
        let output = Command::new(exe)
            .args(extra_args)
            .arg(&path)
            .output()
            .unwrap_or_else(|_| panic!("run {}", path.display()));

        let mut actual = String::from_utf8_lossy(&output.stdout).into_owned();
        actual.push_str(&String::from_utf8_lossy(&output.stderr));

        assert_eq!(
            expected.trim_end(),
            actual.trim_end(),
            "output mismatch for {}",
            display_path(&path)
        );
    }

    assert!(total > 0, "no .epic tests found");
}

fn display_path(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}
