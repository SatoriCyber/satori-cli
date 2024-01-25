use std::{
    collections::HashSet, fs::File, io::{self, Cursor, Write}, path::PathBuf, sync::{Mutex, OnceLock}
};

use clap::Command;
use clap_complete::{generate, Generator, Shell};


use crate::cli::command;

const AUTO_COMPLETE_FUNCTIONS_ZSH: &str = include_str!("autocomplete_functions/zshell.zsh");
const AUTO_COMPLETE_FUNCTIONS_POWERSHELL: &str =
    include_str!("autocomplete_functions/powershell.ps1");

const SATORI_RUN_SUFFIX: &str = "satori;run;";

static AUTO_COMPLETE_DATABASES_TOOLS: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();
static AUTO_COMPLETE_TOOLS: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();


pub fn add_database_tool_autocomplete(tool_name: String) {
    let mut tools = AUTO_COMPLETE_DATABASES_TOOLS.get_or_init(|| Mutex::new(HashSet::new())).lock().unwrap();
    tools.insert(tool_name);
}

pub fn add_tool_autocomplete(tool_name: String) {
    let mut tools = AUTO_COMPLETE_TOOLS.get_or_init(|| Mutex::new(HashSet::new())).lock().unwrap();
    tools.insert(tool_name);
}

fn get_database_tools() -> HashSet<String> {
    AUTO_COMPLETE_DATABASES_TOOLS.get().unwrap().lock().unwrap().clone()
}

fn get_tools() -> HashSet<String> {
    AUTO_COMPLETE_TOOLS.get().unwrap().lock().unwrap().clone()
}
pub fn auto_complete(shell: Shell, out: PathBuf) {
    let mut cmd = command::get();
    eprintln!("Generating completion file for {shell}...");
    let mut buffer: Vec<u8> = Vec::new();
    let mut cursor = io::Cursor::new(&mut buffer);

    generate_autocomplete_buf(shell, &mut cmd, &mut cursor);
    let auto_complete_script = String::from_utf8(buffer).unwrap();
    let auto_complete_script = make_dynamic_completion_script(shell, &auto_complete_script);
    let mut output_file = File::create(out).unwrap();
    output_file
        .write_all(auto_complete_script.as_bytes())
        .unwrap();
}

fn generate_autocomplete_buf<G: Generator>(
    gen: G,
    cmd: &mut Command,
    buf: &mut Cursor<&mut Vec<u8>>,
) {
    generate(gen, cmd, cmd.get_name().to_string(), buf);
}

fn make_dynamic_completion_script(shell: Shell, auto_complete_script: &str) -> String {
    match shell {
        Shell::Bash => todo!(),
        Shell::Elvish => unimplemented!(),
        Shell::Fish => unimplemented!(),
        Shell::PowerShell => handle_power_shell(auto_complete_script),
        Shell::Zsh => handle_zsh(auto_complete_script),
        _ => todo!(),
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
    auto_complete_static.push_str(AUTO_COMPLETE_FUNCTIONS_ZSH);
    auto_complete_static
}

fn handle_power_shell(auto_complete_script: &str) -> String {
    // We need to count also non bareword as elements, in order to support matching the datastore name which is single quoted
    let mut auto_complete_string = auto_complete_script.replace("$element.StringConstantType -ne [StringConstantType]::BareWord -or", "");
    
    // Add the Get-Datastore and Get-Databases functions
    auto_complete_string.push_str(AUTO_COMPLETE_FUNCTIONS_POWERSHELL);
    
    let mut moved_curr_complete = "".to_string();
    
    for tool in get_tools() {
        // will result in something like: satori;run;mongosh' { 
        // We need to replace it to the datastore auto-complete
        let full_command_line = format!("{SATORI_RUN_SUFFIX}{tool}' {{");
        
        // Get the indexes of the current tool auto-complete and extract them
        let (start_index_cur_compl, end_index_cur_compl) = get_pwsh_tool_completion_indexes(&auto_complete_string, &full_command_line);
        let curr_complete = &auto_complete_string[start_index_cur_compl..end_index_cur_compl].to_string();
        
        // Replace the current tool auto-complete with datastore auto-complete
        auto_complete_string.replace_range(start_index_cur_compl..end_index_cur_compl, r#"
            $completionResults += Get-CompletionDatastores
            $completionResults
        "#);

        // Return the original auto-complete to a new section, also add the database auto-complete if needed
        let original_auto_complete = if get_database_tools().contains(&tool)  {
            format!(r#"
            if($command.StartsWith('{SATORI_RUN_SUFFIX}{tool}')) {{
                if ($commandElements.Count -eq 4) {{
                    $datastoreName = $commandElements[3]
                    $completions = Get-CompletionDatabases -DatastoreName $datastoreName
                
                }} else {{
                    $completions=@({curr_complete})
                }}
            }}
            "#)
          
        } else {
            format!(r#"
            if($command.StartsWith('{SATORI_RUN_SUFFIX}{tool}')) {{
                $completions=@({curr_complete})
            }}
            "#)
        };
        moved_curr_complete.push_str(&original_auto_complete);
    }
    let new_s = format!(r#"
    if ($completions.Count -eq 0) {{
        {moved_curr_complete}
    
    }}

    $completions.Where"#);
    auto_complete_string.replace("$completions.Where", &new_s)
}

fn get_pwsh_tool_completion_indexes(auto_complete_string: &str, full_command_name_with_suffix: &str) -> (usize, usize) {
    let start_index = auto_complete_string.find(full_command_name_with_suffix).and_then(|start_index| Some(start_index + full_command_name_with_suffix.len())).unwrap();
    let end_index = auto_complete_string[start_index..].find("break").and_then(|end_index| Some(start_index + end_index)).unwrap();

    (start_index, end_index)
}