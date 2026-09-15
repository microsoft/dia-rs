use microsoft_dia::{DataSource, Session};
use windows_core::*;

pub fn get_test_session() -> Result<Session> {
    let source = DataSource::open()?;
    let executable = std::env::current_exe().unwrap();
    source.load_exe(executable.to_str().unwrap(), None)?;
    source.open_session()
}
