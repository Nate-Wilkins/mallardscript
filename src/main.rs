extern crate anyhow;
extern crate pest_duckyscript;

use anyhow::{anyhow, Context, Result};
use pest_duckyscript::{duckyscript, mallardscript};
use std::io::Write;
use std::{collections::HashMap, io::Seek};

/// Entry point for mallardscript.
fn main() -> Result<()> {
    // Command line interface.
    let args = create_application()?.get_matches();

    // Initialize logger.
    initialize_logger();

    // Run the application.
    if let Err(e) = run(args) {
        eprintln!("{}", e);
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
    if let Some(_) = args.subcommand_matches("completions") {
        return command_completions(args);
    } else if let Some(_) = args.subcommand_matches("build") {
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

    return Ok(());
}

/// Command to build MallardScript.
fn command_build(args: clap::ArgMatches) -> Result<()> {
    // Parse arguments.
    let args_build = args.subcommand_matches("build").unwrap();
    let input = args_build.value_of("input").unwrap();
    let output = args_build.value_of("output").unwrap();
    let mut output_path = std::path::PathBuf::from(shellexpand::tilde(output).into_owned());
    output_path.push("index.ducky");
    let output_file_path = &output_path.clone();

    // Build.
    println!("Build MallardScript.");
    println!("  Input: '{}'", input);
    println!("  Output: '{}'", output);
    println!("");

    // Setup.
    let output_file = std::fs::File::create(output_file_path).context(format!(
        "Failed to create output file '{}'.",
        output_file_path.display()
    ))?;

    // Compile.
    compile_input(input, &output_file, HashMap::new()).context(format!(
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

    return Ok(());
}

/// Compile MallardScript input path to DuckyScript output file.
fn compile_input(
    input_path: &str,
    mut output_file: &std::fs::File,
    mut imports_visited: HashMap<String, bool>,
) -> Result<()> {
    log::info!("Compiling '{}'.", input_path);

    // Expand our input path.
    let input_path_expanded =
        &std::path::PathBuf::from(shellexpand::tilde(input_path).into_owned());

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
    let input_contents = std::fs::read_to_string(input_path_expanded).with_context(|| {
        format!(
            "Unable to load file input '{}'.",
            input_path_expanded.display()
        )
    })?;

    // Parse input contents into AST.
    let program_ast = mallardscript::parser::parse_document(input_contents)
        .with_context(|| format!("Unable to parse input."))?;

    // Process AST.
    for statement in program_ast {
        match statement {
            mallardscript::ast::Statement::Command(command) => {
                log::info!("Processing '{} {}'.", command.name, command.value);

                // Process import statement commands.
                if command.name == "IMPORT" {
                    // Compile import file.
                    compile_input(&command.value, output_file, imports_visited.clone()).context(
                        format!(
                            "Unable to import file '{}' from '{}'.",
                            command.value, input_path
                        ),
                    )?;

                    // Add a new line after import file compilation.
                    output_file
                        .write_all(String::from("\n").as_bytes())
                        .context(format!("Unable to write to output file."))
                        .unwrap();
                } else {
                    // Process all other statement commands.
                    output_file
                        .write_all(
                            String::from(format!("{} {}\n", command.name, command.value))
                                .as_bytes(),
                        )
                        .context(format!("Unable to write to output file."))
                        .unwrap();
                }
            }

            mallardscript::ast::Statement::Variable(variable) => {
                log::info!("Processing '${} = {}'.", variable.name, variable.assignment);

                // Process all variable statements.
                output_file
                    .write_all(
                        String::from(format!("${} = {}\n", variable.name, variable.assignment))
                            .as_bytes(),
                    )
                    .context(format!("Unable to write to output file."))
                    .unwrap();
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
        }
    }

    return Ok(());
}
