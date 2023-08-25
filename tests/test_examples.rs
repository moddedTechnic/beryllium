use std::{
    collections::HashMap,
    fs::create_dir,
    path::PathBuf,
    process::Command,
};


#[test]
fn examples() {
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
    ]);
    let examples_dir = PathBuf::from("examples");
    let build_dir = PathBuf::from("examples/build");
    if !build_dir.exists() {
        create_dir(&build_dir).expect("failed to create build dir");
    }

    for (example, expected_code) in examples.iter() {
        let example_file = examples_dir.join(example);
        let target_file = build_dir.join(example);
        println!("{example_file:?} => {target_file:?}");
        assert!(example_file.exists());
        let compile_args = beryllium::CompileArgs {
            source_file: example_file,
            target_file: Some(target_file.clone()),
        };
        assert!(beryllium::compile(&compile_args).is_ok());
        let output = Command::new(target_file).output().expect("executable runs correctly");
        let code = output.status.code();
        assert!(code.is_some());
        assert_eq!(code.unwrap(), *expected_code);
    }
}

