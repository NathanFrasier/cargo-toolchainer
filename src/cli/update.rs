use std::{fs, path::PathBuf};

use clap::Args;
use toml::Table;

use crate::update_channel::ChannelFormat;
use crate::utils::find_toolchain_file;

#[derive(Args)]
pub struct UpdateArgs {
    #[arg(short, long)]
    project_dir: Option<PathBuf>,

    /// Allow cargo-toolchainer to update to an "anchored" version from a "floating" one
    #[arg(short, long)]
    anchor: bool,

    /// Specify the channel format to use. Auto-detect if not supplied. Implies --anchor
    #[arg(short, long)]
    channel: Option<ChannelFormat>,

    /// Ensure arguments are valid and lookup the latest version, but do not write it to the
    /// rust-toolchain.toml file
    #[arg(short, long)]
    dry_run: bool,
}
pub fn update(args: UpdateArgs) {
    // get the info in the current toolchain file
    let toolchain_file_path = find_toolchain_file(args.project_dir).unwrap();
    let mut existing_data = fs::read_to_string(&toolchain_file_path)
        .expect("failed to read toolchain file")
        .parse::<Table>()
        .expect("failed to parse toolchain file");

    //verify that path is not set
    if existing_data
        .get("toolchain")
        .and_then(|t| t.get("path"))
        .is_some()
    {
        panic!("Cannot set toolchain channel when path is present");
    }

    //check if they told us what format to use, if they did, no need to auto-detect
    let channel_format = if args.channel.is_none() {
        // get the current channel text
        let channel_string = existing_data
            .get("toolchain")
            .and_then(|t| t.get("channel"))
            .and_then(|c| c.as_str())
            .expect("could not identify current channel");
        //check if current version is anchored, and error with a warning message if it isn't and there's no anchor flag
        let (detected_channel, is_anchored) = ChannelFormat::detect_channel(channel_string);
        if !is_anchored && !args.anchor {
            panic!("cargo-toolchainer will not overwrite a floating channel with an anchored one. To override this behavior, supply the --anchor flag");
        }
        detected_channel
    } else {
        args.channel.unwrap()
    };

    let version = channel_format.get_latest();

    // write the updated data back to the toolchain file
    if !args.dry_run {
        let version_field = existing_data
            .get_mut("toolchain")
            .and_then(|tc| tc.get_mut("channel"))
            .unwrap();
        *version_field = toml::Value::String(version.to_string());
        let _ = fs::write(
            toolchain_file_path,
            toml::to_string(&existing_data).unwrap(),
        );
    }
}
