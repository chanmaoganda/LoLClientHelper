use base64::engine::{general_purpose, Engine};
use sysinfo::System;


use super::{authorize_info::AuthorizeInfo, error::LcuError};

#[cfg(target_os = "windows")]
const TARGET_PROCESS: &str = "LeagueClientUx.exe";


pub fn get_authorize_info() -> Result<AuthorizeInfo, LcuError>{
    let mut system = System::new_all();
    system.refresh_processes();

    let args = system
        .processes_by_name(TARGET_PROCESS)
        .nth(0)
        .ok_or(LcuError::ProcessNotFound)?
        .cmd();

    let port = args
        .iter()
        .find(|arg| arg.starts_with("--app-port="))
        .map(|arg| arg.strip_prefix("--app-port=").unwrap().to_string())
        .ok_or(LcuError::PortNotFound)?;

    let auth_token = args
        .iter()
        .find(|arg| arg.starts_with("--remoting-auth-token="))
        .map(|arg| {
            arg.strip_prefix("--remoting-auth-token=")
                .unwrap()
                .to_string()
        })
        .ok_or(LcuError::TokenNotFound)?;
    let encoded_token = general_purpose::STANDARD.encode(format!("riot:{}", auth_token));
    Ok(AuthorizeInfo::new(port, format!("Basic {}", encoded_token)))
}