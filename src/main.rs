use anyhow::{Context, Result};
use serde::Deserialize;
use std::env;
use std::fs;

const FNAME: &str = "cv2res.toml";

#[derive(Debug, Deserialize)]
struct CV {
    _name: String,
    _phone: String,
    _email: String,
    _site: String,
    _github: String,
    _projs: Vec<Proj>,
    _small_projs: Vec<SmallProj>,
    _hobby_projs: Vec<HobbyProj>,
}

#[derive(Debug, Deserialize)]
struct Proj {
    _title: String,
    _url: String,
    _stack: String,
    _desc: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct SmallProj {
    _title: String,
    _url: String,
    _desc: Vec<String>,
}

type HobbyProj = SmallProj;

fn parse_cfg_file(path: &str) -> Result<CV> {
    let _toml_str = fs::read_to_string(path).with_context(|| format!("Failed to read {path}."))?;

    // TODO: deserialize to structs

    todo!()
}

fn main() -> Result<()> {
    // Chdir if provided
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        eprintln!("Usage: {} <dir>", args[0]);
        std::process::exit(1);
    } else if args.len() == 2 {
        std::env::set_current_dir(&args[1])?;
    }

    // TODO: add flag for config file

    // TODO: add flag for chdir

    // read config.toml
    let _content = parse_cfg_file(FNAME);

    Ok(())
}
