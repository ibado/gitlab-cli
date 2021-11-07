use std::error::Error;
use crate::config::{load_config, write_config};

pub const GITLAB_USER_KEY: &str = "GITLAB_USER";
pub const GITLAB_TOKEN_KEY: &str = "GITLAB_TOKEN";

pub struct GitlabCredentials {
    pub user_name: String,
    pub user_token: String,
}

impl GitlabCredentials {
    pub fn get() -> Result<Self, Box<dyn Error>> {
        load_config();
        let user_name = std::env::var(GITLAB_USER_KEY)?;
        let user_token = std::env::var(GITLAB_TOKEN_KEY)?;

        Result::Ok(
            GitlabCredentials {
                user_name,
                user_token,
            }
        )
    }
}

pub fn write_credentials(credentials: GitlabCredentials) -> Result<(), std::io::Error> {
    write_config(GITLAB_USER_KEY.to_string(), credentials.user_name)?;
    write_config(GITLAB_TOKEN_KEY.to_string(), credentials.user_token)?;
    Ok(())
}
