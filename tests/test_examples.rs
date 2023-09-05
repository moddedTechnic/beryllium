
macro_rules! valid_example {
    ($name:ident, $exit_code:tt) => {
        #[test]
        fn $name() {
            let examples_dir = PathBuf::from("examples");
            let build_dir = PathBuf::from("examples/build");
            if !build_dir.exists() {
                match create_dir(&build_dir) {
                    Ok(_) => (),
                    Err(err) => if !matches!(err.kind(), std::io::ErrorKind::AlreadyExists) {
                        Result::<(), std::io::Error>::Err(err).unwrap()
                    }
                }
            }

            let example = stringify!($name.be);
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
            assert_eq!(code.unwrap(), $exit_code);
            println!("    SUCCESS\n");
        }
    };
}


macro_rules! invalid_example {
    ($name:ident, $err:pat) => {
        #[test]
        fn invalid_variable_mutability_fails() {
            let examples_dir = PathBuf::from("examples");
            let build_dir = PathBuf::from("examples/build");
            if !build_dir.exists() {
                create_dir(&build_dir).expect("failed to create build dir");
            }

            let example = stringify!($name.be);
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
            assert!(matches!(err, $err));
            println!("    SUCCESS\n");
        }       
    };
}



mod example {
    use std::{
        fs::create_dir,
        path::PathBuf,
        process::Command,
    };

    valid_example!(exit_failure, 20);
    valid_example!(exit_simple, 0);
    valid_example!(exit_variable, 20);
    valid_example!(let_simple, 0);
    valid_example!(let_variable_value, 10);
    valid_example!(maths_add_simple, 3);
    valid_example!(maths_add_three_way, 6);
    valid_example!(maths_add_variables, 6);
    valid_example!(maths_div_remainder, 5);
    valid_example!(maths_div_simple, 5);
    valid_example!(maths_mod_simple, 0);
    valid_example!(maths_mul_simple, 6);
    valid_example!(maths_mul_variables, 8);
    valid_example!(maths_sub_simple, 1);
    valid_example!(maths_sub_three_way, 0);
    valid_example!(maths_sub_variables, 2);
    valid_example!(if_simple_true, 1);
    valid_example!(if_simple_false, 0);
    valid_example!(if_else_true, 0);
    valid_example!(if_else_false, 1);
    valid_example!(variable_scoping, 5);
    valid_example!(variable_scoping_multiple, 3);
    valid_example!(block, 3);
    valid_example!(block_if, 3);
    valid_example!(comparison_equality_true, 0);
    valid_example!(comparison_equality_false, 1);
    valid_example!(comparison_nonequality_true, 0);
    valid_example!(comparison_nonequality_false, 1);
    valid_example!(comparison_lesser_true, 0);
    valid_example!(comparison_lesser_false_eq, 1);
    valid_example!(comparison_lesser_false, 1);
    valid_example!(comparison_lesser_equal_true, 0);
    valid_example!(comparison_lesser_equal_true_eq, 0);
    valid_example!(comparison_lesser_equal_false, 1);
    valid_example!(comparison_greater_true, 0);
    valid_example!(comparison_greater_false_eq, 1);
    valid_example!(comparison_greater_false, 1);
    valid_example!(comparison_greater_equal_true, 0);
    valid_example!(comparison_greater_equal_true_eq, 0);
    valid_example!(comparison_greater_equal_false, 1);
    valid_example!(variable_mutability_valid, 1);
    valid_example!(iteration_while, 10);
    valid_example!(iteration_loop, 10);
    valid_example!(iteration_continue, 10);
    valid_example!(function_call, 1);

    invalid_example!(variable_mutability_invalid, beryllium::CompileError::ChangedImmutableVariable(_));
}

