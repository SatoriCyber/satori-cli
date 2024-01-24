use std::{
    fs::File,
    io::{self, Cursor, Write},
    path::PathBuf,
};

use clap::Command;
use clap_complete::{generate, Generator, Shell};

use crate::cli::command;

const AUTO_COMPLETE_FUNCTIONS_ZSH: &str = include_str!("autocomplete_functions/zshell.zsh");

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

fn make_dynamic_completion_script(shell: Shell, auto_complete_static: &str) -> String {
    match shell {
        Shell::Bash => todo!(),
        Shell::Elvish => unimplemented!(),
        Shell::Fish => unimplemented!(),
        Shell::PowerShell => todo!(),
        Shell::Zsh => handle_zsh(auto_complete_static),
        _ => todo!(),
    }
}

fn handle_zsh(auto_complete_static: &str) -> String {
    let mut auto_complete_static = auto_complete_static
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
