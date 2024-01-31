use std::{
    collections::HashSet,
    fs::File,
    io::{self, Cursor, Write},
    path::PathBuf,
    sync::{Mutex, OnceLock},
};

use clap::Command;
use clap_complete::{generate, Generator, Shell};

use crate::cli::command;

const ZSH_FUNCTIONS: &str = include_str!("auto_complete_helpers/zshell/functions.zsh");
const POWERSHELL_FUNCTIONS: &str = include_str!("auto_complete_helpers/powershell/functions.ps1");

const BASH_FUNCTIONS: &str = include_str!("auto_complete_helpers/bash/functions.sh");
const BASH_DATASTORES: &str = include_str!("auto_complete_helpers/bash/datastores_complete.sh");
const BASH_DATABASES: &str = include_str!("auto_complete_helpers/bash/databases_complete.sh");

const SATORI_RUN_PREFIX_POWER_SHELL: &str = "satori;run;";
const SATORI_RUN_PREFIX_BASH: &str = "satori__run__";

static AUTO_COMPLETE_DATABASES_TOOLS: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();
static AUTO_COMPLETE_TOOLS: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

pub fn add_database_tool_autocomplete(tool_name: String) {
    let mut tools = AUTO_COMPLETE_DATABASES_TOOLS
        .get_or_init(|| Mutex::new(HashSet::new()))
        .lock()
        .unwrap();
    tools.insert(tool_name);
}

pub fn add_tool_autocomplete(tool_name: String) {
    let mut tools = AUTO_COMPLETE_TOOLS
        .get_or_init(|| Mutex::new(HashSet::new()))
        .lock()
        .unwrap();
    tools.insert(tool_name);
}

fn get_database_tools() -> HashSet<String> {
    AUTO_COMPLETE_DATABASES_TOOLS
        .get()
        .unwrap()
        .lock()
        .unwrap()
        .clone()
}

fn get_tools() -> HashSet<String> {
    AUTO_COMPLETE_TOOLS.get().unwrap().lock().unwrap().clone()
}
pub fn auto_complete(shell: Shell, out: PathBuf) {
    eprintln!("Generating completion file for {shell}...");
    let auto_complete_script = match generate_autocomplete_text(shell) {
        Ok(val) => val,
        Err(err) => {
            log::error!("Failed to generate auto-complete script: {}", err);
            return;
        }
    };
    let mut output_file = File::create(out).unwrap();
    output_file
        .write_all(auto_complete_script.as_bytes())
        .unwrap();
}

/// Public for testing only
pub(super) fn generate_autocomplete_text(shell: Shell) -> Result<String, String> {
    let mut cmd = command::get();
    let mut buffer: Vec<u8> = Vec::new();
    let mut cursor = io::Cursor::new(&mut buffer);
    generate_autocomplete_buf(shell, &mut cmd, &mut cursor);
    let auto_complete_script = String::from_utf8(buffer).unwrap();
    make_dynamic_completion_script(shell, &auto_complete_script)
}

fn generate_autocomplete_buf<G: Generator>(
    gen: G,
    cmd: &mut Command,
    buf: &mut Cursor<&mut Vec<u8>>,
) {
    generate(gen, cmd, cmd.get_name().to_string(), buf);
}

fn make_dynamic_completion_script(
    shell: Shell,
    auto_complete_script: &str,
) -> Result<String, String> {
    match shell {
        Shell::Bash => Ok(handle_bash(auto_complete_script.to_string())),
        Shell::PowerShell => Ok(handle_power_shell(auto_complete_script)),
        Shell::Zsh => Ok(handle_zsh(auto_complete_script)),
        _ => Err(format!(
            "Unsupported shell: {shell:?}. Supported shells are: Bash, PowerShell, Zsh",
        )),
    }
}

fn handle_zsh(auto_complete_script: &str) -> String {
    let mut auto_complete_static = auto_complete_script
        .replace(
            "datastore_name -- The name as defined in Satori data portal:",
            "datastore_name -- The name as defined in Satori data portal:_datastores",
        )
        .replace(
            "database -- Database name:",
            "database -- Database name:_databases",
        );
    auto_complete_static.push_str(ZSH_FUNCTIONS);
    auto_complete_static
}

fn handle_bash(mut auto_complete_script: String) -> String {
    auto_complete_script.push_str(BASH_FUNCTIONS);
    for tool in get_tools() {
        let full_command_line = format!("{SATORI_RUN_PREFIX_BASH}{tool})");
        let (start_index, end_index) =
            match get_indexes(&auto_complete_script, &full_command_line, "esac") {
                Ok(val) => val,
                Err(_) => {
                    // The last tool is not closed with satori__run, so we need to handle it separately
                    get_indexes(&auto_complete_script, &full_command_line, "esac\n}").unwrap()
                }
            };
        // The end index should include the esac word
        let end_index = end_index + 4;
        let curr_complete = &auto_complete_script[start_index..end_index].to_string();
        let mut new_s = BASH_DATASTORES.to_string();
        if get_database_tools().contains(&tool) {
            new_s.push_str(BASH_DATABASES);
        }
        new_s.push_str(&format!("else\n {curr_complete}\nfi\n"));
        auto_complete_script.replace_range(start_index..end_index, &new_s);
    }

    auto_complete_script
}

fn handle_power_shell(auto_complete_script: &str) -> String {
    // We need to count also non bareword as elements, in order to support matching the datastore name which is single quoted
    let mut auto_complete_string = auto_complete_script.replace(
        "$element.StringConstantType -ne [StringConstantType]::BareWord -or",
        "",
    );

    // Add the Get-Datastore and Get-Databases functions
    auto_complete_string.push_str(POWERSHELL_FUNCTIONS);

    let mut moved_curr_complete = String::new();

    for tool in get_tools() {
        // will result in something like: satori;run;mongosh' {
        // We need to replace it to the datastore auto-complete
        let full_command_line = format!("{SATORI_RUN_PREFIX_POWER_SHELL}{tool}' {{");

        // Get the indexes of the current tool auto-complete and extract them
        let (start_index_cur_compl, end_index_cur_compl) =
            get_indexes(&auto_complete_string, &full_command_line, "break").unwrap();
        let curr_complete =
            &auto_complete_string[start_index_cur_compl..end_index_cur_compl].to_string();

        // Replace the current tool auto-complete with datastore auto-complete
        auto_complete_string.replace_range(
            start_index_cur_compl..end_index_cur_compl,
            r"
            $completionResults += Get-CompletionDatastores
            $completionResults
        ",
        );

        // Return the original auto-complete to a new section, also add the database auto-complete if needed
        let original_auto_complete = if get_database_tools().contains(&tool) {
            format!(
                r#"
            if($command.StartsWith('{SATORI_RUN_PREFIX_POWER_SHELL}{tool}')) {{
                if ($commandElements.Count -eq 4) {{
                    $datastoreName = $commandElements[3]
                    $completions = Get-CompletionDatabases -DatastoreName $datastoreName
                
                }} else {{
                    $completions=@({curr_complete})
                }}
            }}
            "#
            )
        } else {
            format!(
                r#"
            if($command.StartsWith('{SATORI_RUN_PREFIX_POWER_SHELL}{tool}')) {{
                $completions=@({curr_complete})
            }}
            "#
            )
        };
        moved_curr_complete.push_str(&original_auto_complete);
    }
    let new_s = format!(
        r#"
    if ($completions.Count -eq 0) {{
        {moved_curr_complete}
    
    }}

    $completions.Where"#
    );
    auto_complete_string.replace("$completions.Where", &new_s)
}

/// Get the indexes of the start and end of two strings
fn get_indexes(
    string_to_search: &str,
    start_text: &str,
    end_text: &str,
) -> Result<(usize, usize), IndexNotFound> {
    let start_index = string_to_search
        .find(start_text)
        .map(|start_index| start_index + start_text.len())
        .ok_or(IndexNotFound::StartIndexNotFound)?;
    let end_index = string_to_search[start_index..]
        .find(end_text)
        .map(|end_index| start_index + end_index)
        .ok_or(IndexNotFound::EndIndexNotFound)?;

    Ok((start_index, end_index))
}

#[derive(Debug)]
enum IndexNotFound {
    StartIndexNotFound,
    EndIndexNotFound,
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     /// This test will break each time a change to the cli is made.
//     /// It is expected, it just to make sure the auto-complete still work.
//     /// Each time you make a change make sure the auto-complete still works.
//     const EXPECTED_BASH: &str = include_str!("tests_helpers/auto_complete/expected_bash.sh");
//     const EXPECTED_ZSH: &str = include_str!("tests_helpers/auto_complete/expected_zsh.zsh");
//     const EXPECTED_POWER_SHELL: &str =
//         include_str!("tests_helpers/auto_complete/expected_powershell.ps1");
//     #[test]
//     fn test_auto_complete_bash() {
//         assert_eq!(
//             generate_autocomplete_text(Shell::Bash).unwrap(),
//             EXPECTED_BASH
//         );
//     }

//     #[test]
//     fn test_auto_complete_zsh() {
//         assert_eq!(
//             generate_autocomplete_text(Shell::Zsh).unwrap(),
//             EXPECTED_ZSH
//         );
//     }

//     #[test]
//     fn test_auto_complete_powershell() {
//         assert_eq!(
//             generate_autocomplete_text(Shell::PowerShell).unwrap(),
//             EXPECTED_POWER_SHELL
//         );
//     }
// }
