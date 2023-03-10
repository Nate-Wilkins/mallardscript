extern crate assert_cmd;
extern crate predicates;
extern crate pretty_assertions;
extern crate tempfile;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use std::{io::Write, process::Command};
use tempfile::tempdir;
use tempfile::NamedTempFile;

#[test]
fn test_command_completions_type_zsh() -> Result<(), Box<dyn std::error::Error>> {
    // Given the CLI.
    let mut cmd = Command::cargo_bin("mallardscript")?;

    // When the user generates completions for zsh.
    let result = cmd.arg("completions").arg("--type").arg("zsh").assert();

    result
        // Then no errors occurred.
        .success()
        .stderr(predicate::str::is_empty())
        // Then completions for zsh were outputted.
        .stdout(predicate::str::contains("#compdef mallardscript"));

    Ok(())
}

#[test]
fn test_command_build_duckyscript_valid_rem_only() -> Result<(), Box<dyn std::error::Error>> {
    // Given the CLI.
    let mut cmd = Command::cargo_bin("mallardscript")?;

    // And DuckyScript file with REM commands only.
    let mut input_file = NamedTempFile::new()?;
    input_file.write_all(
        String::from(
            r#"
REM Hello, Friend.
REM This is it.
REM Good luck.
"#,
        )
        .as_bytes(),
    )?;

    // And an output directory.
    let temp_output_path = tempdir().unwrap();
    let output_path = temp_output_path.path().as_os_str().to_str().unwrap();

    // When the user builds the script.
    let result = cmd
        .arg("build")
        .arg("--input")
        .arg(input_file.path())
        .arg("--output")
        .arg(output_path)
        .assert();

    result
        // Then no errors occurred.
        .success()
        .stderr(predicate::str::is_empty())
        // Then the build completed successfully.
        .stdout(predicate::str::contains(format!(
            r#"Build MallardScript.
  Current Directory: '{}'
  Input: '{}'
  Output: '{}'
Done."#,
            std::env::current_dir().unwrap().display(),
            input_file.path().display(),
            String::from(output_path)
        )));

    // Then the build output is correct.
    let mut output_file_path =
        std::path::PathBuf::from(shellexpand::tilde(output_path).into_owned());
    output_file_path.push("index.ducky");

    let output_contents = std::fs::read_to_string(output_file_path).unwrap();
    assert_eq!(
        output_contents,
        r#"REM Hello, Friend.
REM This is it.
REM Good luck."#,
    );

    Ok(())
}

#[test]
fn test_command_build_duckyscript_valid_rem_string_only() -> Result<(), Box<dyn std::error::Error>>
{
    // Given the CLI.
    let mut cmd = Command::cargo_bin("mallardscript")?;

    // And DuckyScript file with REM and STRING commands only.
    let mut input_file = NamedTempFile::new()?;
    input_file.write_all(
        String::from(
            r#"
REM Hello, Friend.
STRING Typing Typing Typing...
"#,
        )
        .as_bytes(),
    )?;

    // And an output directory.
    let temp_output_path = tempdir().unwrap();
    let output_path = temp_output_path.path().as_os_str().to_str().unwrap();

    // When the user builds the script.
    let result = cmd
        .arg("build")
        .arg("--input")
        .arg(input_file.path())
        .arg("--output")
        .arg(output_path)
        .assert();

    result
        // Then no errors occurred.
        .success()
        .stderr(predicate::str::is_empty())
        // Then the build completed successfully.
        .stdout(predicate::str::contains(format!(
            r#"Build MallardScript.
  Current Directory: '{}'
  Input: '{}'
  Output: '{}'
Done."#,
            std::env::current_dir().unwrap().display(),
            input_file.path().display(),
            String::from(output_path)
        )));

    // Then the build output is correct.
    let mut output_file_path =
        std::path::PathBuf::from(shellexpand::tilde(output_path).into_owned());
    output_file_path.push("index.ducky");

    let output_contents = std::fs::read_to_string(output_file_path).unwrap();
    assert_eq!(
        output_contents,
        r#"REM Hello, Friend.
STRING Typing Typing Typing..."#,
    );

    Ok(())
}

#[test]
fn test_command_build_duckyscript_valid_rem_string_variable_only(
) -> Result<(), Box<dyn std::error::Error>> {
    // Given the CLI.
    let mut cmd = Command::cargo_bin("mallardscript")?;

    // And DuckyScript file with REM, STRING, and VAR commands only.
    let mut input_file = NamedTempFile::new()?;
    input_file.write_all(
        String::from(
            r#"
REM Hello, Friend.
STRING Typing Typing Typing...
VAR $MY_VARIABLE = 34
"#,
        )
        .as_bytes(),
    )?;

    // And an output directory.
    let temp_output_path = tempdir().unwrap();
    let output_path = temp_output_path.path().as_os_str().to_str().unwrap();

    // When the user builds the script.
    let result = cmd
        .arg("build")
        .arg("--input")
        .arg(input_file.path())
        .arg("--output")
        .arg(output_path)
        .assert();

    result
        // Then no errors occurred.
        .success()
        .stderr(predicate::str::is_empty())
        // Then the build completed successfully.
        .stdout(predicate::str::contains(format!(
            r#"Build MallardScript.
  Current Directory: '{}'
  Input: '{}'
  Output: '{}'
Done."#,
            std::env::current_dir().unwrap().display(),
            input_file.path().display(),
            String::from(output_path)
        )));

    // Then the build output is correct.
    let mut output_file_path =
        std::path::PathBuf::from(shellexpand::tilde(output_path).into_owned());
    output_file_path.push("index.ducky");

    let output_contents = std::fs::read_to_string(output_file_path).unwrap();
    assert_eq!(
        output_contents,
        r#"REM Hello, Friend.
STRING Typing Typing Typing...
VAR $MY_VARIABLE = 34"#,
    );

    Ok(())
}

#[test]
fn test_command_build_duckyscript_valid_rem_string_single_import_only(
) -> Result<(), Box<dyn std::error::Error>> {
    // Given the CLI.
    let mut cmd = Command::cargo_bin("mallardscript")?;

    // And DuckyScript file dependency with STRING commands only.
    let mut input_file_dependency = NamedTempFile::new()?;
    input_file_dependency
        .write_all(String::from(r#"STRING Typing Typing Typing..."#).as_bytes())?;

    // And DuckyScript file with REM and IMPORT commands.
    let mut input_file = NamedTempFile::new()?;
    input_file.write_all(
        format!(
            r#"
REM Hello, Friend.
IMPORT "{}"
"#,
            input_file_dependency.path().as_os_str().to_str().unwrap()
        )
        .as_bytes(),
    )?;

    // And an output directory.
    let temp_output_path = tempdir().unwrap();
    let output_path = temp_output_path.path().as_os_str().to_str().unwrap();

    // When the user builds the script.
    let result = cmd
        .arg("build")
        .arg("--input")
        .arg(input_file.path())
        .arg("--output")
        .arg(output_path)
        .assert();

    result
        // Then no errors occurred.
        .success()
        .stderr(predicate::str::is_empty())
        // Then the build completed successfully.
        .stdout(predicate::str::contains(format!(
            r#"Build MallardScript.
  Current Directory: '{}'
  Input: '{}'
  Output: '{}'
Done."#,
            std::env::current_dir().unwrap().display(),
            input_file.path().display(),
            String::from(output_path)
        )));

    // Then the build output is correct.
    let mut output_file_path =
        std::path::PathBuf::from(shellexpand::tilde(output_path).into_owned());
    output_file_path.push("index.ducky");

    let output_contents = std::fs::read_to_string(output_file_path).unwrap();
    println!("{:?}", output_contents);
    assert_eq!(
        output_contents,
        r#"REM Hello, Friend.
STRING Typing Typing Typing..."#,
    );

    Ok(())
}

#[test]
fn test_command_build_duckyscript_valid_rem_string_multiple_import_only(
) -> Result<(), Box<dyn std::error::Error>> {
    // Given the CLI.
    let mut cmd = Command::cargo_bin("mallardscript")?;

    // And DuckyScript file dependency A with STRING commands only.
    let mut input_file_dependency_a = NamedTempFile::new()?;
    input_file_dependency_a.write_all(String::from(r#"STRING Typing From A..."#).as_bytes())?;

    // And DuckyScript file dependency B with STRING and IMPORT commands only.
    let mut input_file_dependency_b = NamedTempFile::new()?;
    input_file_dependency_b.write_all(
        format!(
            r#"
IMPORT "{}"
STRING Typing From B...
"#,
            input_file_dependency_a.path().as_os_str().to_str().unwrap()
        )
        .as_bytes(),
    )?;

    // And DuckyScript file with REM and IMPORT commands.
    let mut input_file = NamedTempFile::new()?;
    input_file.write_all(
        format!(
            r#"
REM Hello, Friend.
IMPORT "{}"
"#,
            input_file_dependency_b.path().as_os_str().to_str().unwrap()
        )
        .as_bytes(),
    )?;

    // And an output directory.
    let temp_output_path = tempdir().unwrap();
    let output_path = temp_output_path.path().as_os_str().to_str().unwrap();

    // When the user builds the script.
    let result = cmd
        .arg("build")
        .arg("--input")
        .arg(input_file.path())
        .arg("--output")
        .arg(output_path)
        .assert();

    result
        // Then no errors occurred.
        .success()
        .stderr(predicate::str::is_empty())
        // Then the build completed successfully.
        .stdout(predicate::str::contains(format!(
            r#"Build MallardScript.
  Current Directory: '{}'
  Input: '{}'
  Output: '{}'
Done."#,
            std::env::current_dir().unwrap().display(),
            input_file.path().display(),
            String::from(output_path)
        )));

    // Then the build output is correct.
    let mut output_file_path =
        std::path::PathBuf::from(shellexpand::tilde(output_path).into_owned());
    output_file_path.push("index.ducky");

    let output_contents = std::fs::read_to_string(output_file_path).unwrap();
    println!("{:?}", output_contents);
    assert_eq!(
        output_contents,
        r#"REM Hello, Friend.
STRING Typing From A...
STRING Typing From B..."#,
    );

    Ok(())
}

#[test]
fn test_command_build_duckyscript_valid_if() -> Result<(), Box<dyn std::error::Error>> {
    // Given the CLI.
    let mut cmd = Command::cargo_bin("mallardscript")?;

    // And DuckyScript file with IF commands.
    let mut input_file = NamedTempFile::new()?;
    input_file.write_all(
        String::from(
            r#"IF TRUE THEN
  REM Hello, Friend.
END_IF"#,
        )
        .as_bytes(),
    )?;

    // And an output directory.
    let temp_output_path = tempdir().unwrap();
    let output_path = temp_output_path.path().as_os_str().to_str().unwrap();

    // When the user builds the script.
    let result = cmd
        .arg("build")
        .arg("--input")
        .arg(input_file.path())
        .arg("--output")
        .arg(output_path)
        .assert();

    result
        // Then no errors occurred.
        .success()
        .stderr(predicate::str::is_empty())
        // Then the build completed successfully.
        .stdout(predicate::str::contains(format!(
            r#"Build MallardScript.
  Current Directory: '{}'
  Input: '{}'
  Output: '{}'
Done."#,
            std::env::current_dir().unwrap().display(),
            input_file.path().display(),
            String::from(output_path)
        )));

    // Then the build output is correct.
    let mut output_file_path =
        std::path::PathBuf::from(shellexpand::tilde(output_path).into_owned());
    output_file_path.push("index.ducky");

    let output_contents = std::fs::read_to_string(output_file_path).unwrap();
    assert_eq!(
        output_contents,
        r#"IF TRUE THEN
  REM Hello, Friend.
END_IF"#,
    );

    Ok(())
}

#[test]
fn test_command_build_duckyscript_valid_if_expression() -> Result<(), Box<dyn std::error::Error>> {
    // Given the CLI.
    let mut cmd = Command::cargo_bin("mallardscript")?;

    // And DuckyScript file with IF commands.
    let mut input_file = NamedTempFile::new()?;
    input_file.write_all(
        String::from(
            r#"IF $MY_VARIABLE > 0 && TRUE THEN
  REM Hello, Friend.
END_IF"#,
        )
        .as_bytes(),
    )?;

    // And an output directory.
    let temp_output_path = tempdir().unwrap();
    let output_path = temp_output_path.path().as_os_str().to_str().unwrap();

    // When the user builds the script.
    let result = cmd
        .arg("build")
        .arg("--input")
        .arg(input_file.path())
        .arg("--output")
        .arg(output_path)
        .assert();

    result
        // Then no errors occurred.
        .success()
        .stderr(predicate::str::is_empty())
        // Then the build completed successfully.
        .stdout(predicate::str::contains(format!(
            r#"Build MallardScript.
  Current Directory: '{}'
  Input: '{}'
  Output: '{}'
Done."#,
            std::env::current_dir().unwrap().display(),
            input_file.path().display(),
            String::from(output_path)
        )));

    // Then the build output is correct.
    let mut output_file_path =
        std::path::PathBuf::from(shellexpand::tilde(output_path).into_owned());
    output_file_path.push("index.ducky");

    let output_contents = std::fs::read_to_string(output_file_path).unwrap();
    assert_eq!(
        output_contents,
        r#"IF $MY_VARIABLE > 0 && TRUE THEN
  REM Hello, Friend.
END_IF"#,
    );

    Ok(())
}

#[test]
fn test_command_build_duckyscript_valid_if_and_else_expression(
) -> Result<(), Box<dyn std::error::Error>> {
    // Given the CLI.
    let mut cmd = Command::cargo_bin("mallardscript")?;

    // And DuckyScript file with IF commands.
    let mut input_file = NamedTempFile::new()?;
    input_file.write_all(
        String::from(
            r#"IF $MY_VARIABLE > 0 && TRUE THEN
  REM Hello, Friend.
ELSE
  REM Hello, Dog?
END_IF"#,
        )
        .as_bytes(),
    )?;

    // And an output directory.
    let temp_output_path = tempdir().unwrap();
    let output_path = temp_output_path.path().as_os_str().to_str().unwrap();

    // When the user builds the script.
    let result = cmd
        .arg("build")
        .arg("--input")
        .arg(input_file.path())
        .arg("--output")
        .arg(output_path)
        .assert();

    result
        // Then no errors occurred.
        .success()
        .stderr(predicate::str::is_empty())
        // Then the build completed successfully.
        .stdout(predicate::str::contains(format!(
            r#"Build MallardScript.
  Current Directory: '{}'
  Input: '{}'
  Output: '{}'
Done."#,
            std::env::current_dir().unwrap().display(),
            input_file.path().display(),
            String::from(output_path)
        )));

    // Then the build output is correct.
    let mut output_file_path =
        std::path::PathBuf::from(shellexpand::tilde(output_path).into_owned());
    output_file_path.push("index.ducky");

    let output_contents = std::fs::read_to_string(output_file_path).unwrap();
    assert_eq!(
        output_contents,
        r#"IF $MY_VARIABLE > 0 && TRUE THEN
  REM Hello, Friend.
ELSE
  REM Hello, Dog?
END_IF"#,
    );

    Ok(())
}

#[test]
fn test_command_build_duckyscript_valid_if_and_else_nested(
) -> Result<(), Box<dyn std::error::Error>> {
    // Given the CLI.
    let mut cmd = Command::cargo_bin("mallardscript")?;

    // And DuckyScript file with IF commands.
    let mut input_file = NamedTempFile::new()?;
    input_file.write_all(
        String::from(
            r#"IF $MY_VARIABLE > 0 && TRUE THEN
  IF TRUE THEN
    REM Hello, Friend.
  END_IF
ELSE
  IF TRUE THEN
    REM Hello, Dog?
  END_IF
END_IF"#,
        )
        .as_bytes(),
    )?;

    // And an output directory.
    let temp_output_path = tempdir().unwrap();
    let output_path = temp_output_path.path().as_os_str().to_str().unwrap();

    // When the user builds the script.
    let result = cmd
        .arg("build")
        .arg("--input")
        .arg(input_file.path())
        .arg("--output")
        .arg(output_path)
        .assert();

    result
        // Then no errors occurred.
        .success()
        .stderr(predicate::str::is_empty())
        // Then the build completed successfully.
        .stdout(predicate::str::contains(format!(
            r#"Build MallardScript.
  Current Directory: '{}'
  Input: '{}'
  Output: '{}'
Done."#,
            std::env::current_dir().unwrap().display(),
            input_file.path().display(),
            String::from(output_path)
        )));

    // Then the build output is correct.
    let mut output_file_path =
        std::path::PathBuf::from(shellexpand::tilde(output_path).into_owned());
    output_file_path.push("index.ducky");

    let output_contents = std::fs::read_to_string(output_file_path).unwrap();
    assert_eq!(
        output_contents,
        r#"IF $MY_VARIABLE > 0 && TRUE THEN
  IF TRUE THEN
    REM Hello, Friend.
  END_IF
ELSE
  IF TRUE THEN
    REM Hello, Dog?
  END_IF
END_IF"#,
    );

    Ok(())
}

#[test]
fn test_command_build_duckyscript_valid_while() -> Result<(), Box<dyn std::error::Error>> {
    // Given the CLI.
    let mut cmd = Command::cargo_bin("mallardscript")?;

    // And DuckyScript file with IF commands.
    let mut input_file = NamedTempFile::new()?;
    input_file.write_all(
        String::from(
            r#"WHILE TRUE
  REM Hello, Friend.
END_WHILE"#,
        )
        .as_bytes(),
    )?;

    // And an output directory.
    let temp_output_path = tempdir().unwrap();
    let output_path = temp_output_path.path().as_os_str().to_str().unwrap();

    // When the user builds the script.
    let result = cmd
        .arg("build")
        .arg("--input")
        .arg(input_file.path())
        .arg("--output")
        .arg(output_path)
        .assert();

    result
        // Then no errors occurred.
        .success()
        .stderr(predicate::str::is_empty())
        // Then the build completed successfully.
        .stdout(predicate::str::contains(format!(
            r#"Build MallardScript.
  Current Directory: '{}'
  Input: '{}'
  Output: '{}'
Done."#,
            std::env::current_dir().unwrap().display(),
            input_file.path().display(),
            String::from(output_path)
        )));

    // Then the build output is correct.
    let mut output_file_path =
        std::path::PathBuf::from(shellexpand::tilde(output_path).into_owned());
    output_file_path.push("index.ducky");

    let output_contents = std::fs::read_to_string(output_file_path).unwrap();
    assert_eq!(
        output_contents,
        r#"WHILE TRUE
  REM Hello, Friend.
END_WHILE"#,
    );

    Ok(())
}

#[test]
fn test_command_build_duckyscript_valid_while_nested() -> Result<(), Box<dyn std::error::Error>> {
    // Given the CLI.
    let mut cmd = Command::cargo_bin("mallardscript")?;

    // And DuckyScript file with IF commands.
    let mut input_file = NamedTempFile::new()?;
    input_file.write_all(
        String::from(
            r#"WHILE TRUE
  WHILE TRUE
    REM Hello, Friend.
  END_WHILE
END_WHILE"#,
        )
        .as_bytes(),
    )?;

    // And an output directory.
    let temp_output_path = tempdir().unwrap();
    let output_path = temp_output_path.path().as_os_str().to_str().unwrap();

    // When the user builds the script.
    let result = cmd
        .arg("build")
        .arg("--input")
        .arg(input_file.path())
        .arg("--output")
        .arg(output_path)
        .assert();

    result
        // Then no errors occurred.
        .success()
        .stderr(predicate::str::is_empty())
        // Then the build completed successfully.
        .stdout(predicate::str::contains(format!(
            r#"Build MallardScript.
  Current Directory: '{}'
  Input: '{}'
  Output: '{}'
Done."#,
            std::env::current_dir().unwrap().display(),
            input_file.path().display(),
            String::from(output_path)
        )));

    // Then the build output is correct.
    let mut output_file_path =
        std::path::PathBuf::from(shellexpand::tilde(output_path).into_owned());
    output_file_path.push("index.ducky");

    let output_contents = std::fs::read_to_string(output_file_path).unwrap();
    assert_eq!(
        output_contents,
        r#"WHILE TRUE
  WHILE TRUE
    REM Hello, Friend.
  END_WHILE
END_WHILE"#,
    );

    Ok(())
}

#[test]
fn test_command_build_duckyscript_valid_multi_command_key() -> Result<(), Box<dyn std::error::Error>>
{
    // Given the CLI.
    let mut cmd = Command::cargo_bin("mallardscript")?;

    // And DuckyScript file with IF commands.
    let mut input_file = NamedTempFile::new()?;
    input_file.write_all(String::from(r#"GUI SHIFT WINDOWS"#).as_bytes())?;

    // And an output directory.
    let temp_output_path = tempdir().unwrap();
    let output_path = temp_output_path.path().as_os_str().to_str().unwrap();

    // When the user builds the script.
    let result = cmd
        .arg("build")
        .arg("--input")
        .arg(input_file.path())
        .arg("--output")
        .arg(output_path)
        .assert();

    result
        // Then no errors occurred.
        .success()
        .stderr(predicate::str::is_empty())
        // Then the build completed successfully.
        .stdout(predicate::str::contains(format!(
            r#"Build MallardScript.
  Current Directory: '{}'
  Input: '{}'
  Output: '{}'
Done."#,
            std::env::current_dir().unwrap().display(),
            input_file.path().display(),
            String::from(output_path)
        )));

    // Then the build output is correct.
    let mut output_file_path =
        std::path::PathBuf::from(shellexpand::tilde(output_path).into_owned());
    output_file_path.push("index.ducky");

    let output_contents = std::fs::read_to_string(output_file_path).unwrap();
    assert_eq!(output_contents, r#"GUI SHIFT WINDOWS"#,);

    Ok(())
}

#[test]
fn test_command_build_duckyscript_invalid_circular_dependency_imports(
) -> Result<(), Box<dyn std::error::Error>> {
    // Given the CLI.
    let mut cmd = Command::cargo_bin("mallardscript")?;

    // And DuckyScript file dependencies.
    let mut input_file_dependency_a = NamedTempFile::new()?;
    let mut input_file_dependency_b = NamedTempFile::new()?;
    let mut input_file_dependency_c = NamedTempFile::new()?;

    // And DuckyScript file dependency A with STRING and IMPORT C.
    input_file_dependency_a.write_all(
        format!(
            r#"
STRING Typing From A...
IMPORT "{}"
"#,
            input_file_dependency_c.path().as_os_str().to_str().unwrap()
        )
        .as_bytes(),
    )?;

    // And DuckyScript file dependency B with STRING and IMPORT A.
    input_file_dependency_b.write_all(
        format!(
            r#"
IMPORT "{}"
STRING Typing From B...
"#,
            input_file_dependency_a.path().as_os_str().to_str().unwrap()
        )
        .as_bytes(),
    )?;

    // And DuckyScript file dependency C with STRING and IMPORT A.
    input_file_dependency_c.write_all(
        format!(
            r#"
IMPORT "{}"
STRING Typing From C...
"#,
            input_file_dependency_a.path().as_os_str().to_str().unwrap()
        )
        .as_bytes(),
    )?;

    // And DuckyScript file with REM and IMPORT commands.
    let mut input_file = NamedTempFile::new()?;
    input_file.write_all(
        format!(
            r#"
REM Hello, Friend.
IMPORT "{}"
"#,
            input_file_dependency_b.path().as_os_str().to_str().unwrap()
        )
        .as_bytes(),
    )?;

    // And an output directory.
    let temp_output_path = tempdir().unwrap();
    let output_path = temp_output_path.path().as_os_str().to_str().unwrap();

    // When the user builds the script.
    let result = cmd
        .arg("build")
        .arg("--input")
        .arg(input_file.path())
        .arg("--output")
        .arg(output_path)
        .assert();

    result
        // Then errors occurred.
        .failure()
        .stdout(predicate::str::contains(format!(
            r#"Build MallardScript.
  Current Directory: '{}'
  Input: '{}'
  Output: '{}'
"#,
            std::env::current_dir().unwrap().display(),
            input_file.path().display(),
            String::from(output_path)
        )))
        // Then the build failed.
        .stderr(
            predicate::str::is_match("Failed to compile to output file '.+index\\.ducky'\\.")
                .unwrap(),
        );

    // Then the build output is correct.
    let mut output_file_path =
        std::path::PathBuf::from(shellexpand::tilde(output_path).into_owned());
    output_file_path.push("index.ducky");

    let output_contents = std::fs::read_to_string(output_file_path).unwrap();
    println!("{:?}", output_contents);
    assert_eq!(
        output_contents,
        r#"REM Hello, Friend.
STRING Typing From A...
"#,
    );

    Ok(())
}

#[test]
fn test_command_build_duckyscript_valid_multiple_rem_string_import_relative_only(
) -> Result<(), Box<dyn std::error::Error>> {
    // Given the CLI.
    let mut cmd = Command::cargo_bin("mallardscript")?;

    // And DuckyScript file dependency A with STRING commands only.
    let mut input_file_dependency_a = NamedTempFile::new()?;
    input_file_dependency_a.write_all(String::from(r#"STRING Typing From A..."#).as_bytes())?;
    let input_file_path_dependency_a = input_file_dependency_a.path();
    let input_file_path_buffer_dependency_a = input_file_path_dependency_a.to_path_buf();
    let input_file_name_dependency_a = input_file_path_buffer_dependency_a
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    // And DuckyScript file dependency B with STRING and IMPORT commands only.
    let mut input_file_dependency_b = NamedTempFile::new()?;
    input_file_dependency_b.write_all(
        format!(
            r#"
IMPORT "./{}"
STRING Typing From B...
"#,
            input_file_name_dependency_a
        )
        .as_bytes(),
    )?;
    let input_file_path_dependency_b = input_file_dependency_b.path();
    let input_file_path_buffer_dependency_b = input_file_path_dependency_b.to_path_buf();
    let input_file_name_dependency_b = input_file_path_buffer_dependency_b
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    // And DuckyScript file with REM and IMPORT commands.
    let mut input_file = NamedTempFile::new()?;
    input_file.write_all(
        format!(
            r#"
REM Hello, Friend.
IMPORT "./{}"
"#,
            input_file_name_dependency_b
        )
        .as_bytes(),
    )?;

    // And an output directory.
    let temp_output_path = tempdir().unwrap();
    let output_path = temp_output_path.path().as_os_str().to_str().unwrap();

    // When the user builds the script.
    let result = cmd
        .arg("build")
        .arg("--input")
        .arg(input_file.path())
        .arg("--output")
        .arg(output_path)
        .assert();

    result
        // Then no errors occurred.
        .success()
        .stderr(predicate::str::is_empty())
        // Then the build completed successfully.
        .stdout(predicate::str::contains(format!(
            r#"Build MallardScript.
  Current Directory: '{}'
  Input: '{}'
  Output: '{}'
Done."#,
            std::env::current_dir().unwrap().display(),
            input_file.path().display(),
            String::from(output_path)
        )));

    // Then the build output is correct.
    let mut output_file_path =
        std::path::PathBuf::from(shellexpand::tilde(output_path).into_owned());
    output_file_path.push("index.ducky");

    let output_contents = std::fs::read_to_string(output_file_path).unwrap();
    println!("{:?}", output_contents);
    assert_eq!(
        output_contents,
        r#"REM Hello, Friend.
STRING Typing From A...
STRING Typing From B..."#,
    );

    Ok(())
}

#[test]
fn test_command_build_duckyscript_invalid_delay() -> Result<(), Box<dyn std::error::Error>> {
    // Given the CLI.
    let mut cmd = Command::cargo_bin("mallardscript")?;

    // And DuckyScript file with DELAY that is invalid.
    let mut input_file = NamedTempFile::new()?;
    input_file.write_all(
        String::from(
            r#"
DEALAY 3000
"#,
        )
        .as_bytes(),
    )?;

    // And an output directory.
    let temp_output_path = tempdir().unwrap();
    let output_path = temp_output_path.path().as_os_str().to_str().unwrap();

    // When the user builds the script.
    let result = cmd
        .arg("build")
        .arg("--input")
        .arg(input_file.path())
        .arg("--output")
        .arg(output_path)
        .assert();

    result
        // Then errors occurred.
        .failure()
        .stdout(predicate::str::contains(format!(
            r#"Build MallardScript.
  Current Directory: '{}'
  Input: '{}'
  Output: '{}'
"#,
            std::env::current_dir().unwrap().display(),
            input_file.path().display(),
            String::from(output_path)
        )))
        // Then the build failed.
        .stderr(
            predicate::str::is_match("Failed to compile to output file '.+index\\.ducky'\\.")
                .unwrap(),
        )
        .stderr(predicate::str::is_match("0: Unable to parse input\\.").unwrap())
        .stderr(predicate::str::is_match("1: Unable to parse provided document\\.").unwrap())
        .stderr(
            predicate::str::is_match(
                "2:  --> 2:1
     |
   2 | DEALAY 3000
     | ^---
     |
     = expected EOI, keyword_command, or statement_variable",
            )
            .unwrap(),
        );

    // Then the build output is correct.
    let mut output_file_path =
        std::path::PathBuf::from(shellexpand::tilde(output_path).into_owned());
    output_file_path.push("index.ducky");

    let output_contents = std::fs::read_to_string(output_file_path).unwrap();
    println!("{:?}", output_contents);
    assert_eq!(output_contents, r#""#,);

    Ok(())
}

#[test]
fn test_command_build_duckyscript_invalid_import_not_found(
) -> Result<(), Box<dyn std::error::Error>> {
    // Given the CLI.
    let mut cmd = Command::cargo_bin("mallardscript")?;

    // And DuckyScript file with REM and IMPORT command that is not found.
    let mut input_file = NamedTempFile::new()?;
    input_file.write_all(
        String::from(
            r#"
REM Hello, Friend.
IMPORT "./__non_existant.ducky"
"#,
        )
        .as_bytes(),
    )?;

    // And an output directory.
    let temp_output_path = tempdir().unwrap();
    let output_path = temp_output_path.path().as_os_str().to_str().unwrap();

    // When the user builds the script.
    let result = cmd
        .arg("build")
        .arg("--input")
        .arg(input_file.path())
        .arg("--output")
        .arg(output_path)
        .assert();

    result
        // Then errors occurred.
        .failure()
        .stdout(predicate::str::contains(format!(
            r#"Build MallardScript.
  Current Directory: '{}'
  Input: '{}'
  Output: '{}'
"#,
            std::env::current_dir().unwrap().display(),
            input_file.path().display(),
            String::from(output_path)
        )))
        // Then the build failed.
        .stderr(
            predicate::str::is_match("Failed to compile to output file '.+index\\.ducky'\\.")
                .unwrap(),
        )
        .stderr(
            predicate::str::is_match(
                "0: Unable to import file '\\./__non_existant\\.ducky' from '.+'\\.",
            )
            .unwrap(),
        )
        .stderr(
            predicate::str::is_match(
                "1: Unable to find file input '\\./__non_existant\\.ducky' from '.+'\\.",
            )
            .unwrap(),
        )
        .stderr(predicate::str::is_match("2: No such file or directory \\(os error 2\\)").unwrap());

    // Then the build output is correct.
    let mut output_file_path =
        std::path::PathBuf::from(shellexpand::tilde(output_path).into_owned());
    output_file_path.push("index.ducky");

    let output_contents = std::fs::read_to_string(output_file_path).unwrap();
    println!("{:?}", output_contents);
    assert_eq!(
        output_contents,
        r#"REM Hello, Friend.
"#,
    );

    Ok(())
}
