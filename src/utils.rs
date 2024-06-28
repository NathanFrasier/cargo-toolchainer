use std::{env, io, path::PathBuf};

pub fn find_toolchain_file(starting_path: Option<PathBuf>) -> io::Result<PathBuf> {
    let path = starting_path
        .map(|mut sp| {
            if sp.is_dir() {
                sp
            } else {
                sp.pop();
                sp
            }
        })
        .unwrap_or(env::current_dir()?);

    let path_ancestors = path.as_path().ancestors();

    for p in path_ancestors {
        let tested_path = p.join("rust-toolchain.toml");
        if tested_path.try_exists().unwrap_or(false) {
            return Ok(tested_path);
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "rust-toolchain.toml not found",
    ))
}
