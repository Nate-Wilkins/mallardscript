extern crate anyhow;
extern crate pest_duckyscript;

use anyhow::{anyhow, Context, Result};
use pest_duckyscript::mallardscript;
use std::{
    collections::HashMap,
    io::{Seek, Write},
    path::PathBuf,
};

static INDENTATION_SIZE: usize = 2;

/// Compile MallardScript input path to DuckyScript output file.
pub fn compile(
    current_directory: PathBuf,
    input_path: &str,
    output_file: &std::fs::File,
    indentation: usize,
    imports_visited: &mut HashMap<String, bool>,
) -> Result<()> {
    log::info!("Compiling '{}'.", input_path);

    // Expand our input path.
    let input_path_expanded = std::fs::canonicalize(current_directory.join(input_path))
        .with_context(|| {
            format!(
                "Unable to find file input '{}' from '{}'.",
                input_path,
                current_directory.display()
            )
        })?;

    // Handle Circular Dependencies.
    // Do not compile input, if we've already compiled it before.
    if imports_visited.contains_key(input_path) {
        return Err(anyhow!("Circular dependency detected."));
    } else {
        // Mark import as visited.
        imports_visited.insert(
            input_path_expanded
                .clone()
                .into_os_string()
                .into_string()
                .unwrap(),
            true,
        );
    }

    // Load input contents.
    let input_contents = std::fs::read_to_string(&input_path_expanded).with_context(|| {
        format!(
            "Unable to load file input '{}' from '{}'.",
            input_path_expanded.display(),
            current_directory.display()
        )
    })?;

    // Parse input contents into AST.
    let program_ast = mallardscript::parser::parse_document(input_contents)
        .with_context(|| ("Unable to parse input."))?;

    // Process AST.
    for statement in program_ast {
        compile_statement(
            input_path,
            input_path_expanded.clone(),
            statement,
            output_file,
            indentation,
            imports_visited,
        )?;
    }

    Ok(())
}

/// Compile MallardScript simple command to DuckyScript output file.
fn compile_simple_statement(
    output_file: &std::fs::File,
    indentation: usize,
    command_name: String,
    command_value: Option<String>,
) -> Result<()> {
    if let Some(value) = command_value {
        log::info!("Processing '{} {}'.", command_name, value);

        write_statement(
            output_file,
            indentation,
            format!("{} {}\n", command_name, value),
        )?;
    } else {
        // Process all other statement commands.
        write_statement(output_file, indentation, format!("{}\n", command_name))?;
    }

    Ok(())
}

/// Compile MallardScript statement.
fn compile_statement(
    input_path: &str,
    input_path_expanded: PathBuf,
    statement: mallardscript::ast::Statement,
    mut output_file: &std::fs::File,
    indentation: usize,
    imports_visited: &mut HashMap<String, bool>,
) -> Result<()> {
    match statement {
        mallardscript::ast::Statement::CommandDefaultDelay(command) => {
            compile_simple_statement(
                output_file,
                indentation,
                String::from("DEFAULTDELAY"),
                command.value.into(),
            )?;
        }
        mallardscript::ast::Statement::CommandDefine(command) => {
            compile_simple_statement(
                output_file,
                indentation,
                String::from("DEFINE"),
                command.value.into(),
            )?;
        }
        mallardscript::ast::Statement::CommandDelay(command) => {
            compile_simple_statement(
                output_file,
                indentation,
                String::from("DELAY"),
                command.value.into(),
            )?;
        }
        mallardscript::ast::Statement::CommandExfil(command) => {
            compile_simple_statement(
                output_file,
                indentation,
                String::from("EXFIL"),
                command.name.into(),
            )?;
        }
        mallardscript::ast::Statement::CommandKey(command) => {
            fn collect_command_key_values(
                command_key: mallardscript::ast::StatementCommandKey,
            ) -> Vec<String> {
                // Collect all command key statement command key values.
                let mut command_key_statements_reduced = command_key.statements.into_iter().fold(
                    vec![] as Vec<String>,
                    |mut accumulation, statement| {
                        if let mallardscript::ast::Statement::CommandKey(statement_command_key) =
                            statement
                        {
                            accumulation.extend(collect_command_key_values(statement_command_key));
                        } else if let mallardscript::ast::Statement::CommandKeyValue(
                            statement_command_key_value,
                        ) = statement
                        {
                            accumulation.push(statement_command_key_value.name);
                        }

                        accumulation
                    },
                );

                if !command_key.remaining_keys.is_empty() {
                    command_key_statements_reduced.push(command_key.remaining_keys);
                }

                command_key_statements_reduced
            }

            let command_reduced = collect_command_key_values(command).join(" ");
            compile_simple_statement(output_file, indentation, command_reduced, None)?;
        }
        mallardscript::ast::Statement::CommandRem(command) => {
            compile_simple_statement(
                output_file,
                indentation,
                String::from("REM"),
                command.value.into(),
            )?;
        }
        mallardscript::ast::Statement::CommandString(command) => {
            compile_simple_statement(
                output_file,
                indentation,
                String::from("STRING"),
                command.value.into(),
            )?;
        }
        mallardscript::ast::Statement::CommandStringln(command) => {
            compile_simple_statement(
                output_file,
                indentation,
                String::from("STRINGLN"),
                command.value.into(),
            )?;
        }
        mallardscript::ast::Statement::SingleCommand(command) => {
            compile_simple_statement(output_file, indentation, command.name, None)?;
        }
        mallardscript::ast::Statement::VariableDeclaration(variable) => {
            log::info!("Processing '${} = {}'.", variable.name, variable.assignment);

            // Process all variable statements.
            write_statement(
                output_file,
                indentation,
                format!("VAR ${} = {}\n", variable.name, variable.assignment),
            )?;
        }
        mallardscript::ast::Statement::VariableAssignment(variable) => {
            log::info!("Processing '${} = {}'.", variable.name, variable.assignment);

            // Process all variable statements.
            write_statement(
                output_file,
                indentation,
                format!("${} = {}\n", variable.name, variable.assignment),
            )?;
        }
        mallardscript::ast::Statement::CommandImport(command) => {
            // Compile import file.
            // Make sure to get the current working directory so imports can resolve locally.
            let mut new_current_directory = input_path_expanded;
            new_current_directory.pop();
            compile(
                new_current_directory,
                &command.value,
                output_file,
                indentation,
                imports_visited,
            )
            .context(format!(
                "Unable to import file '{}' from '{}'.",
                command.value, input_path
            ))?;

            // Add a new line after import file compilation.
            write_statement(output_file, indentation, String::from("\n"))?;
        }
        mallardscript::ast::Statement::BlockIf(block) => {
            // Process block if statement.
            write_statement(
                output_file,
                indentation,
                format!("IF {} THEN\n", block.expression),
            )?;

            // Process block if statement, true case statements.
            for statement in block.statements_true {
                compile_statement(
                    input_path,
                    input_path_expanded.clone(),
                    statement,
                    output_file,
                    indentation + INDENTATION_SIZE,
                    imports_visited,
                )?;
            }

            // Add ELSE statement.
            if !block.statements_false.is_empty() {
                write_statement(output_file, indentation, String::from("ELSE\n"))?;

                // Process block if statement, false case statements.
                for statement in block.statements_false {
                    compile_statement(
                        input_path,
                        input_path_expanded.clone(),
                        statement,
                        output_file,
                        indentation + INDENTATION_SIZE,
                        imports_visited,
                    )?;
                }
            }

            // Add the END_IF statement.
            write_statement(output_file, indentation, String::from("END_IF\n"))?;
        }
        mallardscript::ast::Statement::BlockWhile(block) => {
            // Process block while statement.
            write_statement(
                output_file,
                indentation,
                format!("WHILE {}\n", block.expression),
            )?;

            // Process block while statement statements.
            for statement in block.statements {
                compile_statement(
                    input_path,
                    input_path_expanded.clone(),
                    statement,
                    output_file,
                    indentation + INDENTATION_SIZE,
                    imports_visited,
                )?;
            }

            // Add the END_WHILE statement.
            write_statement(output_file, indentation, String::from("END_WHILE\n"))?;
        }
        mallardscript::ast::Statement::End { .. } => {
            log::info!("Processing End.");

            // Remove statement end line from end of file.
            output_file
                .set_len(
                    output_file
                        .metadata()
                        .unwrap()
                        .len()
                        .checked_sub("\n".len() as u64)
                        .unwrap(),
                )
                .unwrap();
            output_file.seek(std::io::SeekFrom::End(0))?;
        }
        mallardscript::ast::Statement::CommandKeyValue { .. } => {
            return Err(anyhow!("Provided statement CommandKeyValue not supported at top level commands. These should be nested under CommandKey statements."));
        }
    }

    Ok(())
}

/// Write a statement line to the output file.
/// This also adds indentation for the statement line.
fn write_statement(
    mut output_file: &std::fs::File,
    indentation: usize,
    line: String,
) -> Result<()> {
    output_file
        .write_all(format!("{}{}", " ".repeat(indentation), line).as_bytes())
        .context("Unable to write to output file.")?;

    Ok(())
}
