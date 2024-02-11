use core::fmt;
use std::hash::Hash;
#[cfg(target_family = "unix")]
use std::os::unix::fs::OpenOptionsExt;
use std::{
    collections::HashSet,
    fs::{File, OpenOptions},
    hash::Hasher,
    io::{BufRead, BufReader, Seek, SeekFrom, Write},
    path::PathBuf,
};

use crate::{
    helpers::datastores::DatastoresInfo,
    login::{self, data::Credentials},
    tools::errors,
};

use super::PgPass;

pub async fn run<R>(params: PgPass, user_input_stream: R) -> Result<(), errors::ToolsError>
where
    R: std::io::BufRead,
{
    let (credentials, datastores_info) =
        login::run_with_file(&params.login, user_input_stream).await?;

    let satori_pgpass = pgpass_from_satori_db(&datastores_info, &credentials);
    log::debug!("Satori pgpass: {satori_pgpass:?}");

    let pgpass_file = params.path;
    if pgpass_file.exists() {
        log::debug!("Pgpass file exists, updating it");
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
            writeln!(file, "{entry}").map_err(errors::ToolsError::FailedWritingToPgpassFile)?;
        }
        for entry in satori_entries {
            writeln!(file, "{entry}").map_err(errors::ToolsError::FailedWritingToPgpassFile)?;
        }
        for entry in satori_exiting_entries {
            writeln!(file, "{entry}").map_err(errors::ToolsError::FailedWritingToPgpassFile)?;
        }
    } else {
        let mut file = create_pgpass_file(pgpass_file)?;
        log::debug!("Created pgpass file at {file:?}");
        for entry in satori_pgpass {
            writeln!(file, "{entry}").map_err(errors::ToolsError::FailedWritingToPgpassFile)?;
        }
    }

    Ok(())
}

fn get_pgpass_file(pgpass_file: PathBuf) -> Result<File, errors::ToolsError> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .open(pgpass_file)
        .map_err(errors::ToolsError::FailedToOpenPgpassFile)
}

fn create_pgpass_file(pgpass_file: PathBuf) -> Result<File, errors::ToolsError> {
    let mut open_options = OpenOptions::new();
    open_options.write(true).create(true);

    set_permissions(&mut open_options);

    open_options
        .open(pgpass_file)
        .map_err(errors::ToolsError::FailedToCreatePgpassFile)
}

#[cfg(target_family = "unix")]
fn set_permissions(open_options: &mut OpenOptions) {
    open_options.mode(0o600);
}
#[cfg(target_family = "windows")]
fn set_permissions(_open_options: &mut OpenOptions) {
    log::debug!("Need to implement the windows mode");
}

#[derive(Eq, Ord, PartialOrd)]
pub struct PgPassEntry {
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
    fn from_creds(credentials: Credentials, port: u16, host: String, database: String) -> Self {
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

impl fmt::Debug for PgPassEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PgPassEntry {{ host: {}, port: {}, database: {}, username: {}, password: ******* }}",
            self.host, self.port, self.database, self.username
        )
    }
}

fn pgpass_from_satori_db(
    datastores_info: &DatastoresInfo,
    credentials: &Credentials,
) -> HashSet<PgPassEntry> {
    datastores_info
        .datastores
        .values()
        .filter(|info| {
            let is_postgres_datastore = info.r#type.is_postgres_dialect();
            log::debug!("Datastore is postgres: {is_postgres_datastore}, datastore: {info:?}");

            let has_port = if info.port.is_some() {
                true
            } else {
                log::debug!("Datastore info: {info:?} has no port, not adding to pgpass");
                false
            };
            is_postgres_datastore && has_port
})
        .flat_map(|datastore_info| {
            if datastore_info.databases.is_empty() {
                log::warn!("Datastore info: {datastore_info:?} has no databases, not adding to pgpass");
            }
            datastore_info
                .databases
                .iter()
                .map(|database| {
                    log::debug!("Adding Datastore info: {datastore_info:?} to pgpass with database: {database}");
                    PgPassEntry::from_creds(
                        credentials.clone(),
                        datastore_info.port.expect("Unexpected missing port"),
                        datastore_info
                            .get_datastore_name()
                            .expect("Failed to get satori host")
                            .clone(),
                        database.clone(),
                    )
                }
            )
                .collect::<HashSet<PgPassEntry>>()
        })
        .collect::<HashSet<PgPassEntry>>()
}

fn pgpass_from_file(file: &File) -> Result<HashSet<PgPassEntry>, errors::ToolsError> {
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| {
            let line = line.map_err(errors::ToolsError::ReadLineError)?;
            Ok(PgPassEntry::from(line))
        })
        .collect::<Result<HashSet<PgPassEntry>, errors::ToolsError>>()
}
