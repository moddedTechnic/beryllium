use std::{
    collections::HashMap,
    fs::create_dir,
    path::PathBuf,
    process::Command,
};


#[test]
fn valid_examples() {
    let examples = HashMap::from([
        ("empty_program.be", 0),
        ("exit_failure.be", 20),
        ("exit_simple.be", 0),
        ("exit_variable.be", 20),
        ("let_simple.be", 0),
        ("let_variable_value.be", 10),
        ("maths_add_simple.be", 3),
        ("maths_add_variables.be", 6),
        ("maths_div_remainder.be", 5),
        ("maths_div_simple.be", 5),
        ("maths_mod_simple.be", 0),
        ("maths_mul_simple.be", 6),
        ("maths_mul_variables.be", 8),
        ("maths_sub_simple.be", 1),
        ("maths_sub_variables.be", 2),
        ("if_simple_true.be", 1),
        ("if_simple_false.be", 0),
        ("if_else_true.be", 0),
        ("if_else_false.be", 1),
        ("variable_scoping.be", 5),
        ("variable_scoping_multiple.be", 3),
        ("block.be", 3),
        ("block_if.be", 3),
        ("comparison_equality_true.be", 0),
        ("comparison_equality_false.be", 1),
        ("comparison_nonequality_true.be", 0),
        ("comparison_nonequality_false.be", 1),
        ("comparison_lesser_true.be", 0),
        ("comparison_lesser_false_eq.be", 1),
        ("comparison_lesser_false.be", 1),
        ("comparison_lesser_equal_true.be", 0),
        ("comparison_lesser_equal_true_eq.be", 0),
        ("comparison_lesser_equal_false.be", 1),
        ("comparison_greater_true.be", 0),
        ("comparison_greater_false_eq.be", 1),
        ("comparison_greater_false.be", 1),
        ("comparison_greater_equal_true.be", 0),
        ("comparison_greater_equal_true_eq.be", 0),
        ("comparison_greater_equal_false.be", 1),
        ("variable_mutability_valid.be", 1),
        ("while.be", 10),
    ]);
    let examples_dir = PathBuf::from("examples");
    let build_dir = PathBuf::from("examples/build");
    if !build_dir.exists() {
        create_dir(&build_dir).expect("failed to create build dir");
    }

    for (example, expected_code) in examples.iter() {
        let example_file = examples_dir.join(example);
        let target_file = build_dir.join(example);
        println!("{example}");
        assert!(example_file.exists());
        let compile_args = beryllium::CompileArgs {
            source_file: example_file,
            target_file: Some(target_file.clone()),
        };
        let compile_result = beryllium::compile(&compile_args);
        println!("        {compile_result:?}");
        assert!(compile_result.is_ok());
        println!("    runnning");
        let output = Command::new(target_file).output().expect("executable runs correctly");
        let code = output.status.code();
        assert!(code.is_some());
        assert_eq!(code.unwrap(), *expected_code);
        println!("    SUCCESS\n");
    }
}


#[test]
fn invalid_variable_mutability_fails() {
    let examples_dir = PathBuf::from("examples");
    let build_dir = PathBuf::from("examples/build");
    if !build_dir.exists() {
        create_dir(&build_dir).expect("failed to create build dir");
    }

    let example = "variable_mutability_invalid.be";
    let example_file = examples_dir.join(example);
    let target_file = build_dir.join(example);
    println!("{example}");
    assert!(example_file.exists());
    let compile_args = beryllium::CompileArgs {
        source_file: example_file,
        target_file: Some(target_file),
    };
    let compile_result = beryllium::compile(&compile_args);
    println!("        {compile_result:?}");
    assert!(compile_result.is_err());
    let err = compile_result.unwrap_err();
    assert!(matches!(err, beryllium::CompileError::ChangedImmutableVariable(_)));
    println!("    SUCCESS\n");
}

