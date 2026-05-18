use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use tera::{self, Tera};

use crate::structs::CV;

pub mod structs;

const DEFAULT_FNAME: &str = "cv2res.toml";
const DEFAULT_DIR: &str = ".";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = DEFAULT_FNAME)]
    fpath: String,

    #[arg(short, long, default_value = DEFAULT_DIR)]
    dir: String,
}

fn parse_cfg_file(path: &str) -> Result<CV> {
    let toml_str = fs::read_to_string(path)?;
    let cv = toml::from_str::<CV>(&toml_str)?;
    Ok(cv)
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("Running on {} in {}.", args.fpath, args.dir);

    if args.dir != DEFAULT_DIR {
        std::env::set_current_dir(args.dir)?;
    }

    let cv =
        parse_cfg_file(&args.fpath).with_context(|| "Failed to read config file.".to_string())?;

    let tmplt = include_str!("../template/resume.tmplt");
    let mut tera = Tera::default();
    tera.add_raw_template("core_cv", tmplt)?;
    let output = tera.render("core_cv", &tera::Context::from_serialize(&cv)?)?;

    println!("{output}");
    std::fs::write("output.typ", output)
        .with_context(|| "Failed to dump output to file.".to_string())?;

    Ok(())
}
