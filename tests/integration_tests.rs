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

    assert!(
        !output.status.success(),
        "Expected program to fail but it succeeded"
    );
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
    let out = run_rts(
        r#"let x = "hello";
print x;"#,
    );
    assert_eq!(out, "hello");
}

#[test]
fn test_string_variable_concat() {
    let out = run_rts(
        r#"let a = "foo";
let b = "bar";
print a + b;"#,
    );
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
    let out = run_rts(
        r#"let x = 5;
let n = "nope" + x;
print n;"#,
    );
    assert_eq!(out, "nope5");
}

#[test]
fn test_two_string_numbers_concat() {
    let out = run_rts(r#"print "10" + "10";"#);
    assert_eq!(out, "1010");
}

// ========== Functions ==========

#[test]
fn test_basic_function() {
    let code = r#"
function add(a, b) {
    return a + b;
}
print add(2, 3);
"#;
    let out = run_rts(code);
    assert_eq!(out, "5");
}

#[test]
fn test_function_single_param() {
    let code = r#"
function double(x) {
    return x + x;
}
print double(7);
"#;
    let out = run_rts(code);
    assert_eq!(out, "14");
}

#[test]
fn test_function_no_params() {
    let code = r#"
function greet() {
    return "hello";
}
print greet();
"#;
    let out = run_rts(code);
    assert_eq!(out, "hello");
}

#[test]
fn test_function_return_value_in_variable() {
    let code = r#"
function add(a, b) {
    return a + b;
}
let result = add(10, 20);
print result;
"#;
    let out = run_rts(code);
    assert_eq!(out, "30");
}

#[test]
fn test_function_with_expressions_as_args() {
    let code = r#"
function add(a, b) {
    return a + b;
}
print add(2 + 3, 4 * 2);
"#;
    let out = run_rts(code);
    assert_eq!(out, "13");
}

#[test]
fn test_nested_function_calls() {
    let code = r#"
function add(a, b) {
    return a + b;
}
print add(add(1, 2), add(3, 4));
"#;
    let out = run_rts(code);
    assert_eq!(out, "10");
}

#[test]
fn test_nested_call_as_inner_arg() {
    let code = r#"
function add(a, b) {
    return a + b;
}
let x = 5;
let m = 100;
print add(add(x, 10), m);
"#;
    let out = run_rts(code);
    assert_eq!(out, "115");
}

#[test]
fn test_function_with_if_else() {
    let code = r#"
function max(a, b) {
    if (a > b) {
        return a;
    } else {
        return b;
    }
}
print max(10, 20);
print max(30, 5);
"#;
    let out = run_rts(code);
    assert_eq!(out, "20\n30");
}

#[test]
fn test_function_with_loop() {
    let code = r#"
function sum(n) {
    let total = 0;
    let i = 1;
    while (i < n + 1) {
        total = total + i;
        i++;
    }
    return total;
}
print sum(5);
"#;
    let out = run_rts(code);
    assert_eq!(out, "15");
}

#[test]
fn test_function_local_scope() {
    let code = r#"
let x = 100;
function getFive() {
    let x = 5;
    return x;
}
print getFive();
print x;
"#;
    let out = run_rts(code);
    assert_eq!(out, "5\n100");
}

#[test]
fn test_function_reads_outer_variable() {
    let code = r#"
let x = 42;
function getX() {
    return x;
}
print getX();
"#;
    let out = run_rts(code);
    assert_eq!(out, "42");
}

#[test]
fn test_function_string_return() {
    let code = r#"
function greet(name) {
    return "hello " + name;
}
print greet("world");
"#;
    let out = run_rts(code);
    assert_eq!(out, "hello world");
}

#[test]
fn test_function_string_concat_args() {
    let code = r#"
function concat(a, b) {
    return a + b;
}
print concat("foo", "bar");
"#;
    let out = run_rts(code);
    assert_eq!(out, "foobar");
}

#[test]
fn test_multiple_functions() {
    let code = r#"
function add(a, b) {
    return a + b;
}
function mul(a, b) {
    return a * b;
}
print add(2, 3);
print mul(4, 5);
"#;
    let out = run_rts(code);
    assert_eq!(out, "5\n20");
}

#[test]
fn test_function_calling_another_function() {
    let code = r#"
function double(x) {
    return x + x;
}
function quadruple(x) {
    return double(double(x));
}
print quadruple(3);
"#;
    let out = run_rts(code);
    assert_eq!(out, "12");
}

#[test]
fn test_function_result_in_expression() {
    let code = r#"
function add(a, b) {
    return a + b;
}
let x = add(3, 4) + 10;
print x;
"#;
    let out = run_rts(code);
    assert_eq!(out, "17");
}

#[test]
fn test_function_result_in_condition() {
    let code = r#"
function isBig(x) {
    if (x > 10) {
        return 1;
    } else {
        return 0;
    }
}
if (isBig(15)) {
    print "big";
} else {
    print "small";
}
"#;
    let out = run_rts(code);
    assert_eq!(out, "big");
}

#[test]
fn test_function_called_multiple_times() {
    let code = r#"
function inc(x) {
    return x + 1;
}
let a = inc(0);
let b = inc(a);
let c = inc(b);
print c;
"#;
    let out = run_rts(code);
    assert_eq!(out, "3");
}

#[test]
fn test_function_with_break_in_loop() {
    let code = r#"
function findOver(limit) {
    let i = 0;
    while (i < 100) {
        if (i > limit) {
            return i;
        }
        i++;
    }
    return 0;
}
print findOver(5);
"#;
    let out = run_rts(code);
    assert_eq!(out, "6");
}

#[test]
fn test_recursive_function() {
    let code = r#"
function factorial(n) {
    if (n < 2) {
        return 1;
    }
    return n * factorial(n - 1);
}
print factorial(5);
"#;
    let out = run_rts(code);
    assert_eq!(out, "120");
}

#[test]
fn test_recursive_fibonacci() {
    let code = r#"
function fib(n) {
    if (n < 2) {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}
print fib(10);
"#;
    let out = run_rts(code);
    assert_eq!(out, "55");
}

#[test]
fn test_function_call_as_statement() {
    let code = r#"
let x = 0;
function doStuff() {
    print "called";
    return 0;
}
doStuff();
print "done";
"#;
    let out = run_rts(code);
    assert_eq!(out, "called\ndone");
}

// ========== Arrays ==========

#[test]
fn test_array_literal_print() {
    let out = run_rts("let arr = [1, 2, 3];\nprint arr;");
    assert_eq!(out, "[1, 2, 3]");
}

#[test]
fn test_array_empty() {
    let out = run_rts("let arr = [];\nprint arr;");
    assert_eq!(out, "[]");
}

#[test]
fn test_array_index_read() {
    let out = run_rts("let arr = [10, 20, 30];\nprint arr[0];\nprint arr[1];\nprint arr[2];");
    assert_eq!(out, "10\n20\n30");
}

#[test]
fn test_array_index_write() {
    let out = run_rts("let arr = [1, 2, 3];\narr[1] = 99;\nprint arr;");
    assert_eq!(out, "[1, 99, 3]");
}

#[test]
fn test_array_index_write_persists() {
    let out =
        run_rts("let arr = [1, 2, 3];\narr[0] = 10;\narr[2] = 30;\nprint arr[0];\nprint arr[2];");
    assert_eq!(out, "10\n30");
}

#[test]
fn test_array_index_with_expression() {
    let code = r#"
let arr = [10, 20, 30, 40];
let i = 1;
print arr[i + 1];
"#;
    let out = run_rts(code);
    assert_eq!(out, "30");
}

#[test]
fn test_array_index_write_with_expression() {
    let code = r#"
let arr = [0, 0, 0];
let i = 1;
arr[i] = 42;
print arr;
"#;
    let out = run_rts(code);
    assert_eq!(out, "[0, 42, 0]");
}

#[test]
fn test_array_reassignment() {
    let code = r#"
let arr = [1, 2, 3];
arr = [4, 5, 6];
print arr;
"#;
    let out = run_rts(code);
    assert_eq!(out, "[4, 5, 6]");
}

#[test]
fn test_array_in_loop() {
    let code = r#"
let arr = [10, 20, 30];
for (let i = 0; i < 3; i++) {
    print arr[i];
}
"#;
    let out = run_rts(code);
    assert_eq!(out, "10\n20\n30");
}

#[test]
fn test_array_modify_in_loop() {
    let code = r#"
let arr = [1, 2, 3];
for (let i = 0; i < 3; i++) {
    arr[i] = arr[i] * 10;
}
print arr;
"#;
    let out = run_rts(code);
    assert_eq!(out, "[10, 20, 30]");
}

#[test]
fn test_array_with_strings() {
    let out = run_rts(
        r#"let arr = ["a", "b", "c"];
print arr[1];"#,
    );
    assert_eq!(out, "b");
}

#[test]
fn test_array_passed_to_function() {
    let code = r#"
function first(arr) {
    return arr[0];
}
let a = [42, 99];
print first(a);
"#;
    let out = run_rts(code);
    assert_eq!(out, "42");
}

#[test]
fn test_array_returned_from_function() {
    let code = r#"
function swap(arr) {
    let temp = arr[0];
    arr[0] = arr[1];
    arr[1] = temp;
    return arr;
}
let a = [1, 2];
a = swap(a);
print a;
"#;
    let out = run_rts(code);
    assert_eq!(out, "[2, 1]");
}

#[test]
fn test_array_index_out_of_bounds_fails() {
    let stderr = run_rts_should_fail("let arr = [1, 2];\nprint arr[5];");
    assert!(stderr.contains("out of bounds"));
}

#[test]
fn test_array_negative_index_fails() {
    let stderr = run_rts_should_fail("let arr = [1, 2];\nprint arr[-1];");
    assert!(stderr.contains("out of bounds"));
}

#[test]
fn test_array_bubble_sort() {
    let code = r#"
let arr = [3, 1, 2];
let n = 3;
for (let i = 0; i < n; i++) {
    for (let j = 0; j < n - 1; j++) {
        if (arr[j] > arr[j + 1]) {
            let temp = arr[j];
            arr[j] = arr[j + 1];
            arr[j + 1] = temp;
        }
    }
}
print arr;
"#;
    let out = run_rts(code);
    assert_eq!(out, "[1, 2, 3]");
}

// ========== Nested Arrays ==========

#[test]
fn test_nested_array_literal() {
    let out = run_rts("let arr = [1, [2, 3]];\nprint arr[1];");
    assert_eq!(out, "[2, 3]");
}

#[test]
fn test_nested_array_index_depth_2() {
    let out = run_rts("let arr = [[10, 20], [30, 40]];\nprint arr[1][0];");
    assert_eq!(out, "30");
}

#[test]
fn test_nested_array_index_depth_3() {
    let out = run_rts("let x = [10, [3, [6, 11]], 5];\nprint x[1][1][0];");
    assert_eq!(out, "6");
}

#[test]
fn test_nested_array_index_first_element() {
    let out = run_rts("let arr = [[1, 2], [3, 4]];\nprint arr[0][0];");
    assert_eq!(out, "1");
}

#[test]
fn test_nested_array_index_with_expression() {
    let code = r#"
let arr = [[10, 20], [30, 40]];
let i = 1;
print arr[i][i - 1];
"#;
    let out = run_rts(code);
    assert_eq!(out, "30");
}

#[test]
fn test_nested_array_print_whole() {
    let out = run_rts("let arr = [[1, 2], [3, 4]];\nprint arr;");
    assert_eq!(out, "[[1, 2], [3, 4]]");
}

// ========== Nested Array Assignment ==========

#[test]
fn test_nested_array_write_depth_2() {
    let code = r#"
let x = [[1, 2], [3, 4]];
x[0][1] = 99;
print x;
"#;
    let out = run_rts(code);
    assert_eq!(out, "[[1, 99], [3, 4]]");
}

#[test]
fn test_nested_array_write_depth_3() {
    let code = r#"
let x = [10, [3, [6, 11]], 5];
x[1][1][0] = 99;
print x;
"#;
    let out = run_rts(code);
    assert_eq!(out, "[10, [3, [99, 11]], 5]");
}

#[test]
fn test_nested_array_write_preserves_siblings() {
    let code = r#"
let x = [[1, 2], [3, 4]];
x[1][0] = 99;
print x[0];
print x[1];
"#;
    let out = run_rts(code);
    assert_eq!(out, "[1, 2]\n[99, 4]");
}

#[test]
fn test_nested_array_write_then_read() {
    let code = r#"
let x = [[10, 20], [30, 40]];
x[0][0] = 5;
print x[0][0];
print x[0][1];
"#;
    let out = run_rts(code);
    assert_eq!(out, "5\n20");
}

#[test]
fn test_nested_array_write_with_expression_index() {
    let code = r#"
let x = [[1, 2], [3, 4]];
let i = 1;
x[i][i - 1] = 50;
print x;
"#;
    let out = run_rts(code);
    assert_eq!(out, "[[1, 2], [50, 4]]");
}

#[test]
fn test_nested_array_write_in_loop() {
    let code = r#"
let x = [[0, 0], [0, 0]];
for (let i = 0; i < 2; i++) {
    for (let j = 0; j < 2; j++) {
        x[i][j] = i * 2 + j;
    }
}
print x;
"#;
    let out = run_rts(code);
    assert_eq!(out, "[[0, 1], [2, 3]]");
}

#[test]
fn test_nested_array_write_via_function() {
    let code = r#"
function setCell(grid, r, c, val) {
    grid[r][c] = val;
    return 0;
}
let g = [[1, 2], [3, 4]];
setCell(g, 0, 1, 99);
print g;
"#;
    let out = run_rts(code);
    assert_eq!(out, "[[1, 99], [3, 4]]");
}

// ========== Objects ==========

#[test]
fn test_object_property_read() {
    let code = r#"
let obj = {x: 10, y: 20};
print obj.x;
print obj.y;
"#;
    let out = run_rts(code);
    assert_eq!(out, "10\n20");
}

#[test]
fn test_object_property_write() {
    let code = r#"
let obj = {a: 1, b: 2};
obj.a = 99;
print obj.a;
"#;
    let out = run_rts(code);
    assert_eq!(out, "99");
}

#[test]
fn test_object_property_write_preserves_others() {
    let code = r#"
let obj = {a: 1, b: 2};
obj.a = 99;
print obj.b;
"#;
    let out = run_rts(code);
    assert_eq!(out, "2");
}

#[test]
fn test_object_add_new_property() {
    let code = r#"
let obj = {x: 1};
obj.y = 2;
print obj.x;
print obj.y;
"#;
    let out = run_rts(code);
    assert_eq!(out, "1\n2");
}

#[test]
fn test_object_string_values() {
    let code = r#"
let obj = {greeting: "hello"};
print obj.greeting + " world";
"#;
    let out = run_rts(code);
    assert_eq!(out, "hello world");
}

#[test]
fn test_object_in_expression() {
    let code = r#"
let obj = {a: 10, b: 20};
print obj.a + obj.b;
"#;
    let out = run_rts(code);
    assert_eq!(out, "30");
}

#[test]
fn test_object_in_variable() {
    let code = r#"
let obj = {val: 42};
let x = obj.val + 8;
print x;
"#;
    let out = run_rts(code);
    assert_eq!(out, "50");
}

#[test]
fn test_object_nested() {
    let code = r#"
let obj = {inner: {x: 5}};
print obj.inner.x;
"#;
    let out = run_rts(code);
    assert_eq!(out, "5");
}

#[test]
fn test_object_nested_write() {
    let code = r#"
let obj = {inner: {x: 5}};
obj.inner.x = 99;
print obj.inner.x;
"#;
    let out = run_rts(code);
    assert_eq!(out, "99");
}

#[test]
fn test_object_passed_to_function() {
    let code = r#"
function getX(obj) {
    return obj.x;
}
let p = {x: 42, y: 10};
print getX(p);
"#;
    let out = run_rts(code);
    assert_eq!(out, "42");
}

#[test]
fn test_object_mutated_in_function() {
    let code = r#"
function setX(obj, val) {
    obj.x = val;
    return 0;
}
let p = {x: 1, y: 2};
setX(p, 99);
print p.x;
"#;
    let out = run_rts(code);
    assert_eq!(out, "99");
}

#[test]
fn test_object_with_array_value() {
    let code = r#"
let obj = {items: [10, 20, 30]};
print obj.items[1];
"#;
    let out = run_rts(code);
    assert_eq!(out, "20");
}

#[test]
fn test_object_property_in_condition() {
    let code = r#"
let obj = {score: 85};
if (obj.score > 50) {
    print "pass";
} else {
    print "fail";
}
"#;
    let out = run_rts(code);
    assert_eq!(out, "pass");
}

#[test]
fn test_object_property_in_loop() {
    let code = r#"
let obj = {count: 3};
while (obj.count > 0) {
    print obj.count;
    obj.count = obj.count - 1;
}
"#;
    let out = run_rts(code);
    assert_eq!(out, "3\n2\n1");
}

#[test]
fn test_object_underscore_property() {
    let code = r#"
let obj = {my_value: 42};
print obj.my_value;
"#;
    let out = run_rts(code);
    assert_eq!(out, "42");
}

#[test]
fn test_object_undefined_property_fails() {
    let stderr = run_rts_should_fail(
        r#"let obj = {a: 1};
print obj.b;"#,
    );
    assert!(stderr.contains("not found"));
}

// ========== Object Bracket Access ==========

#[test]
fn test_object_bracket_read() {
    let code = r#"
let obj = {a: 10, b: 20};
print obj["a"];
print obj["b"];
"#;
    let out = run_rts(code);
    assert_eq!(out, "10\n20");
}

#[test]
fn test_object_bracket_write() {
    let code = r#"
let obj = {a: 1};
obj["a"] = 99;
print obj.a;
"#;
    let out = run_rts(code);
    assert_eq!(out, "99");
}

#[test]
fn test_object_bracket_dynamic_key() {
    let code = r#"
let obj = {x: 42};
let key = "x";
print obj[key];
"#;
    let out = run_rts(code);
    assert_eq!(out, "42");
}

#[test]
fn test_object_bracket_add_new_property() {
    let code = r#"
let obj = {a: 1};
obj["b"] = 2;
print obj.b;
"#;
    let out = run_rts(code);
    assert_eq!(out, "2");
}

#[test]
fn test_object_bracket_nested() {
    let code = r#"
let obj = {inner: {val: 5}};
print obj["inner"]["val"];
"#;
    let out = run_rts(code);
    assert_eq!(out, "5");
}

#[test]
fn test_object_bracket_mixed_with_dot() {
    let code = r#"
let obj = {inner: {val: 5}};
print obj.inner["val"];
"#;
    let out = run_rts(code);
    assert_eq!(out, "5");
}
