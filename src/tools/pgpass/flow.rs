use core::fmt;
use std::hash::Hash;
use std::{
    collections::HashSet,
    fs::{File, OpenOptions},
    hash::Hasher,
    io::{BufRead, BufReader, Seek, SeekFrom, Write},
    os::unix::fs::OpenOptionsExt,
    path::{Path, PathBuf},
};

use crate::{
    helpers::{datastores::DatastoresInfo, satori_console::DatabaseCredentials},
    login,
    tools::errors,
};

use super::PgPass;

const PGPASS_FILE_NAME: &str = ".pgpass";

pub async fn run(params: PgPass) -> Result<(), errors::ToolsErrors> {
    // Need to catch where file not found and trigger login flow
    let datastores_info = crate::helpers::datastores::file::load()?;

    let credentials = login::get_creds_with_file(&params.login).await?;

    let satori_pgpass = pgpass_from_satori_db(datastores_info.clone(), credentials.clone());

    let pgpass_file = get_pgpass_file_path()?;
    if pgpass_file.exists() {
        let mut file = get_pgpass_file(pgpass_file)?;
        let existing_pgpass = pgpass_from_file(&file)?;

        let non_satori_entries = existing_pgpass
            .difference(&satori_pgpass)
            .collect::<HashSet<&PgPassEntry>>();
        let satori_entries = satori_pgpass
            .difference(&existing_pgpass)
            .collect::<HashSet<&PgPassEntry>>();
        let satori_exiting_entries = satori_pgpass
            .intersection(&existing_pgpass)
            .collect::<HashSet<&PgPassEntry>>();

        file.seek(SeekFrom::Start(0)).unwrap();
        file.set_len(0).unwrap();
        for entry in non_satori_entries {
            writeln!(file, "{}", entry).map_err(errors::ToolsErrors::FailedWritingToPgpassFile)?;
        }
        for entry in satori_entries {
            writeln!(file, "{}", entry).map_err(errors::ToolsErrors::FailedWritingToPgpassFile)?;
        }
        for entry in satori_exiting_entries {
            writeln!(file, "{}", entry).map_err(errors::ToolsErrors::FailedWritingToPgpassFile)?;
        }
    } else {
        let mut file = create_pgpass_file(pgpass_file)?;
        for entry in satori_pgpass {
            writeln!(file, "{}", entry).map_err(errors::ToolsErrors::FailedWritingToPgpassFile)?;
        }
    }

    Ok(())
}

fn get_pgpass_file_path() -> Result<PathBuf, errors::ToolsErrors> {
    Ok(homedir::get_my_home()?
        .ok_or_else(|| errors::ToolsErrors::HomeDirNotFound)?
        .join(Path::new(PGPASS_FILE_NAME)))
}

fn get_pgpass_file(pgpass_file: PathBuf) -> Result<File, errors::ToolsErrors> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .open(pgpass_file)
        .map_err(errors::ToolsErrors::FailedToOpenPgpassFile)
}

fn create_pgpass_file(pgpass_file: PathBuf) -> Result<File, errors::ToolsErrors> {
    let mut open_options = OpenOptions::new();
    open_options.write(true).create(true);

    match std::env::consts::OS {
        "windows" => {
            //TODO: Do we want to set the file permissions on windows?
            log::debug!("Need to implement the windows mode");
        }
        _ => {
            open_options.mode(0o600);
        }
    };
    open_options
        .open(pgpass_file)
        .map_err(errors::ToolsErrors::FailedToCreatePgpassFile)
}

#[derive(Eq)]
struct PgPassEntry {
    host: String,
    port: u16,
    database: String,
    username: String,
    password: String,
}

impl PartialEq for PgPassEntry {
    fn eq(&self, other: &Self) -> bool {
        self.host == other.host && self.port == other.port && self.database == other.database
    }
}

impl Hash for PgPassEntry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.host.hash(state);
        self.port.hash(state);
        self.database.hash(state);
    }
}
impl PgPassEntry {
    fn from_creds(
        credentials: DatabaseCredentials,
        port: u16,
        host: String,
        database: String,
    ) -> Self {
        PgPassEntry {
            host,
            port,
            database,
            username: credentials.username,
            password: credentials.password,
        }
    }
}

impl From<String> for PgPassEntry {
    fn from(entry: String) -> Self {
        let mut entry = entry.split(':');
        PgPassEntry {
            host: entry.next().unwrap().to_string(),
            port: entry.next().unwrap().parse::<u16>().unwrap(),
            database: entry.next().unwrap().to_string(),
            username: entry.next().unwrap().to_string(),
            password: entry.next().unwrap().to_string(),
        }
    }
}
impl fmt::Display for PgPassEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}:{}:{}",
            self.host, self.port, self.database, self.username, self.password
        )
    }
}

fn pgpass_from_satori_db(
    datastores_info: DatastoresInfo,
    credentials: DatabaseCredentials,
) -> HashSet<PgPassEntry> {
    datastores_info
        .value
        .values()
        .filter(|info| info.is_postgres_dialect())
        .flat_map(|datastore_info| {
            datastore_info
                .databases
                .iter()
                .map(|database| {
                    PgPassEntry::from_creds(
                        credentials.clone(),
                        datastore_info.port,
                        datastore_info.satori_host.clone(),
                        database.clone(),
                    )
                })
                .collect::<HashSet<PgPassEntry>>()
        })
        .collect::<HashSet<PgPassEntry>>()
}

fn pgpass_from_file(file: &File) -> Result<HashSet<PgPassEntry>, errors::ToolsErrors> {
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| {
            let line = line.map_err(errors::ToolsErrors::ReadLineError)?;
            Ok(PgPassEntry::from(line))
        })
        .collect::<Result<HashSet<PgPassEntry>, errors::ToolsErrors>>()
}
