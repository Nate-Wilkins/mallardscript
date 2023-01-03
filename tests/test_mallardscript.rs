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

    return Ok(());
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
  Input: '{}'
  Output: '{}'

Done."#,
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

    return Ok(());
}

#[test]
fn test_command_build_duckyscript_valid_rem_string_only() -> Result<(), Box<dyn std::error::Error>>
{
    // Given the CLI.
    let mut cmd = Command::cargo_bin("mallardscript")?;

    // And DuckyScript file with REM commands only.
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
  Input: '{}'
  Output: '{}'

Done."#,
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

    return Ok(());
}

#[test]
fn test_command_build_duckyscript_valid_rem_string_variable_only(
) -> Result<(), Box<dyn std::error::Error>> {
    // Given the CLI.
    let mut cmd = Command::cargo_bin("mallardscript")?;

    // And DuckyScript file with REM commands only.
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
  Input: '{}'
  Output: '{}'

Done."#,
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

    return Ok(());
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
        String::from(format!(
            r#"
REM Hello, Friend.
IMPORT {}
"#,
            input_file_dependency.path().as_os_str().to_str().unwrap()
        ))
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
  Input: '{}'
  Output: '{}'

Done."#,
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

    return Ok(());
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
        String::from(format!(
            r#"
IMPORT {}
STRING Typing From B...
"#,
            input_file_dependency_a.path().as_os_str().to_str().unwrap()
        ))
        .as_bytes(),
    )?;

    // And DuckyScript file with REM and IMPORT commands.
    let mut input_file = NamedTempFile::new()?;
    input_file.write_all(
        String::from(format!(
            r#"
REM Hello, Friend.
IMPORT {}
"#,
            input_file_dependency_b.path().as_os_str().to_str().unwrap()
        ))
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
  Input: '{}'
  Output: '{}'

Done."#,
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

    return Ok(());
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
        String::from(format!(
            r#"
STRING Typing From A...
IMPORT {}
"#,
            input_file_dependency_c.path().as_os_str().to_str().unwrap()
        ))
        .as_bytes(),
    )?;

    // And DuckyScript file dependency B with STRING and IMPORT A.
    input_file_dependency_b.write_all(
        String::from(format!(
            r#"
IMPORT {}
STRING Typing From B...
"#,
            input_file_dependency_a.path().as_os_str().to_str().unwrap()
        ))
        .as_bytes(),
    )?;

    // And DuckyScript file dependency C with STRING and IMPORT A.
    input_file_dependency_c.write_all(
        String::from(format!(
            r#"
IMPORT {}
STRING Typing From C...
"#,
            input_file_dependency_a.path().as_os_str().to_str().unwrap()
        ))
        .as_bytes(),
    )?;

    // And DuckyScript file with REM and IMPORT commands.
    let mut input_file = NamedTempFile::new()?;
    input_file.write_all(
        String::from(format!(
            r#"
REM Hello, Friend.
IMPORT {}
"#,
            input_file_dependency_b.path().as_os_str().to_str().unwrap()
        ))
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
  Input: '{}'
  Output: '{}'
"#,
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

    return Ok(());
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
  Input: '{}'
  Output: '{}'
"#,
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

    return Ok(());
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
IMPORT ./__non_existant.ducky
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
  Input: '{}'
  Output: '{}'
"#,
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
                "1: Unable to load file input '\\./__non_existant\\.ducky'\\.",
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

    return Ok(());
}
