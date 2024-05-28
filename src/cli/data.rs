use std::path::PathBuf;

use clap_complete::Shell;

use crate::{list::data::List, login::Login, pwd::Pwd, run::Run, tools::Tools};

#[derive(Debug)]
pub enum Flow {
    Login(Login),
    Run(Run),
    AutoComplete(Shell, PathBuf),
    List(List),
    Tools(Tools),
    Pwd(Pwd),
}
