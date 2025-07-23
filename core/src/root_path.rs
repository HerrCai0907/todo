use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("cannot find environment '{}'", env))]
    Env {
        source: std::env::VarError,
        env: String,
    },
    #[snafu(display("failed to create directory in '{}'", path))]
    CreateDir {
        source: std::io::Error,
        path: String,
    },
}

pub fn get_folder() -> Result<String, Error> {
    let home_path = std::env::var("HOME").context(EnvSnafu { env: "HOME" })?;
    let dir_path = format!("{}/.todo", home_path);
    std::fs::create_dir_all(&dir_path).context(CreateDirSnafu {
        path: dir_path.clone(),
    })?;
    Ok(dir_path)
}
