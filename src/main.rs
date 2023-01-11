extern crate anyhow;
extern crate pest_duckyscript;

use anyhow::{anyhow, Context, Result};
use pest_duckyscript::{duckyscript, mallardscript};
use std::io::Write;
use std::path::PathBuf;
use std::{collections::HashMap, io::Seek};

/// Entry point for mallardscript.
fn main() -> Result<()> {
    // Command line interface.
    let args = create_application()?.get_matches();

    // Initialize logger.
    initialize_logger();

    // Run the application.
    if let Err(e) = run(args) {
        eprintln!("{:?}", e);
        std::process::exit(2);
    }
    std::process::exit(0);
}

/// Create the application command line interface.
fn create_application() -> Result<clap::App<'static, 'static>> {
    return Ok(clap::App::new(clap::crate_name!())
        .bin_name(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .subcommand(
            clap::SubCommand::with_name("completions")
                .about("completions")
                .arg(
                    clap::Arg::with_name("type")
                        .short("t")
                        .long("type")
                        .required(true)
                        .takes_value(true)
                        .possible_values(&["Bash", "Elvish", "Fish", "PowerShell", "Zsh"])
                        .case_insensitive(true),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("build")
                .about("build mallardscript input")
                .arg(
                    clap::Arg::with_name("input")
                        .short("in")
                        .long("input")
                        .required(false)
                        .takes_value(true)
                        .default_value("index.ducky")
                        .help("entry file to compile"),
                )
                .arg(
                    clap::Arg::with_name("output")
                        .short("out")
                        .long("output")
                        .required(false)
                        .takes_value(true)
                        .default_value("output")
                        .help("out directory to build to"),
                ),
        ));
}

/// Initializes the application logger.
fn initialize_logger() {
    // TODO: Support `--verbosity`.
    let env = env_logger::Env::default();

    return env_logger::Builder::from_env(env)
        .target(env_logger::Target::Stdout)
        .init();
}

/// Run application according to command line interface arguments.
fn run(args: clap::ArgMatches) -> Result<()> {
    if args.subcommand_matches("completions").is_some() {
        return command_completions(args);
    } else if args.subcommand_matches("build").is_some() {
        return command_build(args);
    }

    return Err(anyhow!("No supported command provided."));
}

/// Command to output completions of a specific type to STDOUT.
fn command_completions(args: clap::ArgMatches) -> Result<()> {
    // Parse arguments.
    let args_completions = args.subcommand_matches("completions").unwrap();
    let completion_type = args_completions.value_of("type").unwrap();

    // Generate completion.
    if completion_type == "bash" {
        create_application()?.gen_completions_to(
            create_application()?.get_bin_name().unwrap(),
            clap::Shell::Bash,
            &mut std::io::stdout(),
        );
    } else if completion_type == "elvish" {
        create_application()?.gen_completions_to(
            create_application()?.get_bin_name().unwrap(),
            clap::Shell::Elvish,
            &mut std::io::stdout(),
        );
    } else if completion_type == "fish" {
        create_application()?.gen_completions_to(
            create_application()?.get_bin_name().unwrap(),
            clap::Shell::Fish,
            &mut std::io::stdout(),
        );
    } else if completion_type == "powershell" {
        create_application()?.gen_completions_to(
            create_application()?.get_bin_name().unwrap(),
            clap::Shell::PowerShell,
            &mut std::io::stdout(),
        );
    } else if completion_type == "zsh" {
        create_application()?.gen_completions_to(
            create_application()?.get_bin_name().unwrap(),
            clap::Shell::Zsh,
            &mut std::io::stdout(),
        );
    } else {
        return Err(anyhow!(
            "Completion type '{}' not supported.",
            completion_type
        ));
    }

    Ok(())
}

/// Command to build MallardScript.
fn command_build(args: clap::ArgMatches) -> Result<()> {
    // Parse arguments.
    let args_build = args.subcommand_matches("build").unwrap();
    let input = args_build.value_of("input").unwrap();
    let output = args_build.value_of("output").unwrap();
    let mut output_path = PathBuf::from(shellexpand::tilde(output).into_owned());
    output_path.push("index.ducky");
    let output_file_path = &output_path.clone();

    let current_directory = &std::env::current_dir().unwrap();

    // Build.
    println!("Build MallardScript.");
    println!("  Current Directory: '{}'", current_directory.display());
    println!("  Input: '{}'", input);
    println!("  Output: '{}'", output);

    // Setup.
    let output_file = std::fs::File::create(output_file_path).context(format!(
        "Failed to create output file '{}'.",
        output_file_path.display()
    ))?;

    // Compile.
    compile_input(
        current_directory.clone(),
        input,
        &output_file,
        &mut HashMap::new(),
    )
    .context(format!(
        "Failed to compile to output file '{}'.",
        output_file_path.display()
    ))?;

    // Validate DuckyScript.
    let output_contents = std::fs::read_to_string(output_file_path).with_context(|| {
        format!(
            "Unable load compiled output '{}'.",
            output_file_path.display()
        )
    })?;
    duckyscript::parser::parse_document(output_contents).with_context(|| {
        format!(
            "Unable to validate compiled output '{}'.",
            output_file_path.display(),
        )
    })?;

    println!("Done.");

    Ok(())
}

/// Compile MallardScript simple command to DuckyScript output file.
fn compile_simple_command(
    mut output_file: &std::fs::File,
    command_name: String,
    command_value: Option<String>,
) -> Result<()> {
    if let Some(value) = command_value {
        log::info!("Processing '{} {}'.", command_name, value);

        output_file
            .write_all(format!("{} {}\n", command_name, value).as_bytes())
            .context("Unable to write to output file.")?;
    } else {
        // Process all other statement commands.
        output_file
            .write_all(format!("{}\n", command_name).as_bytes())
            .context("Unable to write to output file.")?;
    }

    Ok(())
}

/// Compile MallardScript input path to DuckyScript output file.
fn compile_input(
    current_directory: PathBuf,
    input_path: &str,
    mut output_file: &std::fs::File,
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
        match statement {
            mallardscript::ast::Statement::CommandDefaultDelay(command) => {
                compile_simple_command(
                    output_file,
                    String::from("DEFAULTDELAY"),
                    command.value.into(),
                )?;
            }
            mallardscript::ast::Statement::CommandDefine(command) => {
                compile_simple_command(output_file, String::from("DEFINE"), command.value.into())?;
            }
            mallardscript::ast::Statement::CommandDelay(command) => {
                compile_simple_command(output_file, String::from("DELAY"), command.value.into())?;
            }
            mallardscript::ast::Statement::CommandExfil(command) => {
                compile_simple_command(output_file, String::from("EXFIL"), command.name.into())?;
            }
            mallardscript::ast::Statement::CommandKey(command) => {
                fn collect_command_key_values(
                    command_key: mallardscript::ast::StatementCommandKey,
                ) -> String {
                    // Collect all command key statement command key values.
                    let command_key_statements_reduced = command_key.statements.into_iter().fold(
                        String::from(""),
                        |accumulation, statement| {
                            if let mallardscript::ast::Statement::CommandKey(
                                statement_command_key,
                            ) = statement
                            {
                                return accumulation
                                    + &collect_command_key_values(statement_command_key);
                            } else if let mallardscript::ast::Statement::CommandKeyValue(
                                statement_command_key_value,
                            ) = statement
                            {
                                return accumulation + &statement_command_key_value.name;
                            }

                            accumulation
                        },
                    );

                    let command_key_remaining_keys = format!(" {}", command_key.remaining_keys);

                    command_key_statements_reduced
                        + if command_key.remaining_keys.is_empty() {
                            ""
                        } else {
                            &command_key_remaining_keys
                        }
                }

                let command_reduced = collect_command_key_values(command);
                compile_simple_command(output_file, command_reduced, None)?;
            }
            mallardscript::ast::Statement::CommandRem(command) => {
                compile_simple_command(output_file, String::from("REM"), command.value.into())?;
            }
            mallardscript::ast::Statement::CommandString(command) => {
                compile_simple_command(output_file, String::from("STRING"), command.value.into())?;
            }
            mallardscript::ast::Statement::CommandStringln(command) => {
                compile_simple_command(
                    output_file,
                    String::from("STRINGLN"),
                    command.value.into(),
                )?;
            }
            mallardscript::ast::Statement::SingleCommand(command) => {
                compile_simple_command(output_file, command.name, None)?;
            }
            mallardscript::ast::Statement::VariableDeclaration(variable) => {
                log::info!("Processing '${} = {}'.", variable.name, variable.assignment);

                // Process all variable statements.
                output_file
                    .write_all(
                        (format!("VAR ${} = {}\n", variable.name, variable.assignment)).as_bytes(),
                    )
                    .context("Unable to write to output file.")?;
            }
            mallardscript::ast::Statement::VariableAssignment(variable) => {
                log::info!("Processing '${} = {}'.", variable.name, variable.assignment);

                // Process all variable statements.
                output_file
                    .write_all(
                        (format!("${} = {}\n", variable.name, variable.assignment)).as_bytes(),
                    )
                    .context("Unable to write to output file.")?;
            }
            mallardscript::ast::Statement::CommandImport(command) => {
                // Compile import file.
                // Make sure to get the current working directory so imports can resolve locally.
                let mut new_current_directory = input_path_expanded.clone();
                new_current_directory.pop();
                compile_input(
                    new_current_directory,
                    &command.value,
                    output_file,
                    imports_visited,
                )
                .context(format!(
                    "Unable to import file '{}' from '{}'.",
                    command.value, input_path
                ))?;

                // Add a new line after import file compilation.
                output_file
                    .write_all(String::from("\n").as_bytes())
                    .context("Unable to write to output file.")?;
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
    }

    Ok(())
}
