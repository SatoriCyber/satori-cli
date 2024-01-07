use std::path::PathBuf;

use clap_complete::Shell;

use crate::{login::Login, run::Run, list::List, tools::Tools};

#[derive(Debug)]
pub enum Flow {
    Login(Login),
    Run(Run),
    AutoComplete(Shell, PathBuf),
    List(List),
    Tools(Tools),
}
