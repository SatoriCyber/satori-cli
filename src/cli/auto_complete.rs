use std::{fs::File, io::BufWriter, path::PathBuf};

use clap::Command;
use clap_complete::{Shell, Generator, generate};

use crate::cli::parser::get_cmd;

pub fn run(shell: Shell, out: PathBuf) {
    let mut cmd = get_cmd();
    eprintln!("Generating completion file for {shell}...");
    let file = File::create(out).unwrap();
    let mut buf_writer = BufWriter::new(file);
    completions_to_file(shell, &mut cmd, &mut buf_writer);
}


fn completions_to_file<G: Generator>(gen: G, cmd: &mut Command, file: &mut BufWriter<File>) {
    generate(gen, cmd, cmd.get_name().to_string(), file);
}
