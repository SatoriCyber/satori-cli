use std::{path::PathBuf, fs::File, io::BufWriter};

use clap::Command;
use clap_complete::{Shell, Generator, generate};

use super::{Flow, command, parsers};

pub fn run() -> Result<Flow, super::CliError> {
    let command = command::get();
    parsers::parse(command)
}

pub fn auto_complete(shell: Shell, out: PathBuf) {
    let mut cmd = command::get();
    eprintln!("Generating completion file for {shell}...");
    let file = File::create(out).unwrap();
    let mut buf_writer = BufWriter::new(file);
    completions_to_file(shell, &mut cmd, &mut buf_writer);
}

fn completions_to_file<G: Generator>(gen: G, cmd: &mut Command, file: &mut BufWriter<File>) {
    generate(gen, cmd, cmd.get_name().to_string(), file);
}
