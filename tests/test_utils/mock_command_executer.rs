use std::sync::OnceLock;

use satori_cli::run::ExecuteCommand;

static CALLED: OnceLock<bool> = OnceLock::new();

pub struct MockCommandExecuter {
    pub expected_command: String,
    pub expected_args: Vec<String>,
    pub expected_envs: Vec<(String, String)>,
}
impl MockCommandExecuter {
    #[allow(dead_code)]
    pub fn new(command_name: String) -> MockCommandExecuter {
        Self {
            expected_command: command_name,
            expected_args: vec![],
            expected_envs: vec![],
        }
    }
    #[allow(dead_code)]
    pub fn assert() {
        CALLED.get().expect("execute was never called");
    }
}

impl ExecuteCommand for MockCommandExecuter {
    fn execute<T, S, V, G, A>(
        &self,
        command_name: &str,
        args: A,
        env: T,
    ) -> Result<(), satori_cli::run::errors::RunError>
    where
        T: IntoIterator<Item = (S, V)>,
        A: IntoIterator<Item = G>,
        G: AsRef<std::ffi::OsStr>,
        S: AsRef<std::ffi::OsStr>,
        V: AsRef<std::ffi::OsStr>,
    {
        assert_eq!(command_name, self.expected_command);

        let string_args = args
            .into_iter()
            .map(|arg| arg.as_ref().to_str().unwrap().to_string())
            .collect::<Vec<String>>();
        assert_eq!(string_args, self.expected_args);

        let string_envs = env
            .into_iter()
            .map(|(k, v)| {
                (
                    k.as_ref().to_str().unwrap().to_string(),
                    v.as_ref().to_str().unwrap().to_string(),
                )
            })
            .collect::<Vec<(String, String)>>();
        assert_eq!(string_envs, self.expected_envs);
        CALLED.set(true).unwrap();
        Ok(())
    }
}
