use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn unique_temp_file() -> std::path::PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::SeqCst);
    let tid = std::thread::current().id();
    std::env::temp_dir().join(format!("test_rusty_{:?}_{}.rts", tid, id))
}

/// Helper: runs an RTS program string via a temp file and returns stdout
fn run_rts(code: &str) -> String {
    let file_path = unique_temp_file();
    std::fs::write(&file_path, code).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", "run", file_path.to_str().unwrap()])
        .output()
        .expect("Failed to execute");

    let _ = std::fs::remove_file(&file_path);

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("Program failed:\n{}", stderr);
    }

    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

/// Helper: runs an RTS program that should panic/fail
fn run_rts_should_fail(code: &str) -> String {
    let file_path = unique_temp_file();
    std::fs::write(&file_path, code).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", "run", file_path.to_str().unwrap()])
        .output()
        .expect("Failed to execute");

    let _ = std::fs::remove_file(&file_path);

    assert!(!output.status.success(), "Expected program to fail but it succeeded");
    String::from_utf8_lossy(&output.stderr).to_string()
}

// ========== Arithmetic ==========

#[test]
fn test_basic_arithmetic() {
    let out = run_rts("print 2 + 3;");
    assert_eq!(out, "5");
}

#[test]
fn test_subtraction() {
    let out = run_rts("print 10 - 3;");
    assert_eq!(out, "7");
}

#[test]
fn test_multiplication() {
    let out = run_rts("print 4 * 5;");
    assert_eq!(out, "20");
}

#[test]
fn test_division() {
    let out = run_rts("print 20 / 4;");
    assert_eq!(out, "5");
}

#[test]
fn test_precedence() {
    let out = run_rts("print 2 + 3 * 4;");
    assert_eq!(out, "14");
}

#[test]
fn test_parentheses() {
    let out = run_rts("print (2 + 3) * 4;");
    assert_eq!(out, "20");
}

#[test]
fn test_nested_parentheses() {
    let out = run_rts("print (2 + (3 * (4 + 2))) / 2;");
    assert_eq!(out, "10");
}

#[test]
fn test_unary_negation() {
    let out = run_rts("print -(2 + 3);");
    assert_eq!(out, "-5");
}

// ========== Variables ==========

#[test]
fn test_variable_declaration() {
    let out = run_rts("let x = 10;\nprint x;");
    assert_eq!(out, "10");
}

#[test]
fn test_variable_reassignment() {
    let out = run_rts("let x = 5;\nx = 10;\nprint x;");
    assert_eq!(out, "10");
}

#[test]
fn test_undeclared_variable_assignment_fails() {
    let stderr = run_rts_should_fail("x = 10;");
    assert!(stderr.contains("not defined") || stderr.contains("Unexpected"));
}

#[test]
fn test_increment() {
    let out = run_rts("let x = 5;\nx++;\nprint x;");
    assert_eq!(out, "6");
}

#[test]
fn test_decrement() {
    let out = run_rts("let x = 5;\nx--;\nprint x;");
    assert_eq!(out, "4");
}

// ========== Comparisons ==========

#[test]
fn test_greater_than_true() {
    let out = run_rts("print 10 > 5;");
    assert_eq!(out, "1");
}

#[test]
fn test_greater_than_false() {
    let out = run_rts("print 3 > 5;");
    assert_eq!(out, "0");
}

#[test]
fn test_less_than() {
    let out = run_rts("print 3 < 5;");
    assert_eq!(out, "1");
}

#[test]
fn test_equal() {
    let out = run_rts("print 5 == 5;");
    assert_eq!(out, "1");
}

#[test]
fn test_not_equal() {
    let out = run_rts("print 5 != 3;");
    assert_eq!(out, "1");
}

// ========== If / Else ==========

#[test]
fn test_if_true() {
    let out = run_rts("if (1 > 0) {\n    print 42;\n}");
    assert_eq!(out, "42");
}

#[test]
fn test_if_false() {
    let out = run_rts("if (0 > 1) {\n    print 42;\n}");
    assert_eq!(out, "");
}

#[test]
fn test_if_else_true_branch() {
    let out = run_rts("if (5 > 3) {\n    print 1;\n} else {\n    print 0;\n}");
    assert_eq!(out, "1");
}

#[test]
fn test_if_else_false_branch() {
    let out = run_rts("if (3 > 5) {\n    print 1;\n} else {\n    print 0;\n}");
    assert_eq!(out, "0");
}

// ========== Logical Operators ==========

#[test]
fn test_and_true() {
    let out = run_rts("print 1 > 0 && 2 > 1;");
    assert_eq!(out, "1");
}

#[test]
fn test_and_false() {
    let out = run_rts("print 1 > 0 && 0 > 1;");
    assert_eq!(out, "0");
}

#[test]
fn test_or_true() {
    let out = run_rts("print 0 > 1 || 2 > 1;");
    assert_eq!(out, "1");
}

#[test]
fn test_or_false() {
    let out = run_rts("print 0 > 1 || 0 > 2;");
    assert_eq!(out, "0");
}

#[test]
fn test_short_circuit_and() {
    // If && short-circuits, x stays 5 (never reaches the right side in a meaningful way)
    // We test by ensuring the result is correct
    let out = run_rts("print 0 && 1;");
    assert_eq!(out, "0");
}

#[test]
fn test_short_circuit_or() {
    let out = run_rts("print 1 || 0;");
    assert_eq!(out, "1");
}

// ========== While Loop ==========

#[test]
fn test_while_loop() {
    let code = r#"
let x = 3;
while (x > 0) {
    print x;
    x = x - 1;
}
"#;
    let out = run_rts(code);
    assert_eq!(out, "3\n2\n1");
}

#[test]
fn test_while_false_condition() {
    let code = r#"
let x = 0;
while (x > 5) {
    print x;
}
print 99;
"#;
    let out = run_rts(code);
    assert_eq!(out, "99");
}

#[test]
fn test_while_break() {
    let code = r#"
let x = 5;
while (x > 0) {
    if (x == 3) {
        break;
    }
    print x;
    x = x - 1;
}
"#;
    let out = run_rts(code);
    assert_eq!(out, "5\n4");
}

#[test]
fn test_while_continue() {
    let code = r#"
let x = 5;
while (x > 0) {
    x = x - 1;
    if (x == 2) {
        continue;
    }
    print x;
}
"#;
    let out = run_rts(code);
    assert_eq!(out, "4\n3\n1\n0");
}

// ========== For Loop ==========

#[test]
fn test_for_loop() {
    let code = r#"
for (let i = 0; i < 4; i++) {
    print i;
}
"#;
    let out = run_rts(code);
    assert_eq!(out, "0\n1\n2\n3");
}

#[test]
fn test_for_loop_break() {
    let code = r#"
for (let i = 0; i < 10; i++) {
    if (i == 3) {
        break;
    }
    print i;
}
"#;
    let out = run_rts(code);
    assert_eq!(out, "0\n1\n2");
}

#[test]
fn test_for_loop_continue() {
    let code = r#"
for (let i = 0; i < 5; i++) {
    if (i == 2) {
        continue;
    }
    print i;
}
"#;
    let out = run_rts(code);
    assert_eq!(out, "0\n1\n3\n4");
}

#[test]
fn test_for_loop_continue_still_increments() {
    // This is the key test — continue in for loop must still run the update (i++)
    let code = r#"
for (let i = 0; i < 5; i++) {
    if (i == 1) {
        continue;
    }
    if (i == 3) {
        break;
    }
    print i;
}
"#;
    let out = run_rts(code);
    assert_eq!(out, "0\n2");
}

// ========== Combined / Integration ==========

#[test]
fn test_full_example() {
    let code = r#"
let x = (2 + (3 * (4 + 2))) / 2;
x++;
while (x > 5) {
    if (x == 8) {
        x = x - 1;
        continue;
    } else {
        if (x > 0 && 10 > 8) {
            print x;
        }
    }
    x--;
}
for (let i = 0; i < x; i++) {
    if (i == 1) {
        continue;
    }
    if (i == 2) {
        break;
    }
    print i;
}
"#;
    let out = run_rts(code);
    assert_eq!(out, "11\n10\n9\n7\n6\n0");
}

#[test]
fn test_division_by_zero_fails() {
    let stderr = run_rts_should_fail("print 10 / 0;");
    assert!(stderr.contains("Division by zero"));
}

#[test]
fn test_nested_if_in_loop() {
    let code = r#"
let x = 3;
while (x > 0) {
    if (x == 2) {
        print 99;
    } else {
        print x;
    }
    x = x - 1;
}
"#;
    let out = run_rts(code);
    assert_eq!(out, "3\n99\n1");
}

// ========== Strings ==========

#[test]
fn test_string_print() {
    let out = run_rts(r#"print "hello";"#);
    assert_eq!(out, "hello");
}

#[test]
fn test_string_concatenation() {
    let out = run_rts(r#"print "hello" + " world";"#);
    assert_eq!(out, "hello world");
}

#[test]
fn test_string_concat_with_number() {
    let out = run_rts(r#"print "score: " + 42;"#);
    assert_eq!(out, "score: 42");
}

#[test]
fn test_number_concat_with_string() {
    let out = run_rts(r#"print 10 + "px";"#);
    assert_eq!(out, "10px");
}

#[test]
fn test_string_variable() {
    let out = run_rts(r#"let x = "hello";
print x;"#);
    assert_eq!(out, "hello");
}

#[test]
fn test_string_variable_concat() {
    let out = run_rts(r#"let a = "foo";
let b = "bar";
print a + b;"#);
    assert_eq!(out, "foobar");
}

#[test]
fn test_string_in_if_condition() {
    let code = r#"
let x = 10;
if (x > 5) {
    print "big";
} else {
    print "small";
}
"#;
    let out = run_rts(code);
    assert_eq!(out, "big");
}

#[test]
fn test_string_equality() {
    let out = run_rts(r#"print "abc" == "abc";"#);
    assert_eq!(out, "1");
}

#[test]
fn test_string_inequality() {
    let out = run_rts(r#"print "abc" != "xyz";"#);
    assert_eq!(out, "1");
}

#[test]
fn test_string_number_equality_false() {
    let out = run_rts(r#"print "10" == 10;"#);
    assert_eq!(out, "0");
}

#[test]
fn test_string_concat_in_variable() {
    let out = run_rts(r#"let x = 5;
let n = "nope" + x;
print n;"#);
    assert_eq!(out, "nope5");
}

#[test]
fn test_two_string_numbers_concat() {
    let out = run_rts(r#"print "10" + "10";"#);
    assert_eq!(out, "1010");
}
