use std::collections::BTreeMap;
use std::fmt::Display;
use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use argh::FromArgValue;
use serde::{Deserialize, Serialize};

pub type Rules = BTreeMap<String, Rule>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Rule {
    pub src: String,
    pub dst: String,
    pub mode: Mode,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Mode {
    #[serde(rename = "copy")]
    Copy,
    #[serde(rename = "link")]
    Link,
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Source: {}\nTarget: {}\nMode  : {}",
            self.src, self.dst, self.mode
        )
    }
}

impl Rule {
    pub fn apply(self, name: String) -> Result<()> {
        let (src, dst) = absolute_path(&self.src, &self.dst);

        is_exists(&name, "source", &src)?;

        match self.mode {
            Mode::Copy => apply_copy(name, src, dst),
            Mode::Link => apply_link(name, src, dst),
        }
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Copy => write!(f, "copy"),
            Mode::Link => write!(f, "link"),
        }
    }
}

impl FromArgValue for Mode {
    fn from_arg_value(value: &str) -> Result<Mode, String> {
        match value {
            "copy" => Ok(Mode::Copy),
            "link" => Ok(Mode::Link),
            _ => Err("Invalid mode".to_string()),
        }
    }
}

fn apply_copy(name: String, src: PathBuf, dst: PathBuf) -> Result<()> {
    if dst.exists() {
        if is_copied(&src, &dst) {
            return Ok(println!("Copied: [{}]", name));
        }
        bail!("Skiped: [{}] target {} exists", name, dst.display());
    }

    create_parent(&name, &dst)?;

    std::fs::copy(&src, &dst).with_context(|| format!("Skiped: [{}] {}", name, dst.display()))?;

    Ok(println!("Copyto: [{}] > {}", name, dst.display()))
}

fn apply_link(name: String, src: PathBuf, dst: PathBuf) -> Result<()> {
    if dst.exists() {
        if is_linked(&src, &dst) {
            return Ok(println!("Linked: [{}]", name));
        }
        bail!("Skiped: [{}] target {} exists", name, dst.display());
    }

    create_parent(&name, &dst)?;

    std::os::unix::fs::symlink(&src, &dst)
        .with_context(|| format!("Skiped: [{}] {}", name, dst.display()))?;

    Ok(println!("Linkto: [{}] > {}", name, dst.display()))
}

/// Check path exists and return error
fn is_exists(name: &String, note: &str, path: &PathBuf) -> Result<()> {
    if !path.exists() {
        bail!("Skiped: [{name}] {note} {} not exists", path.display());
    }

    Ok(())
}

/// Check if the SRC is copied to the DST
fn is_copied(src: &PathBuf, dst: &PathBuf) -> bool {
    if let Err(_) = std::fs::read_link(&dst) {
        if let Ok(src) = std::fs::File::open(src) {
            if let Ok(dst) = std::fs::File::open(dst) {
                return diff_files(src, dst);
            }
        }
    }

    false
}

/// Check if the SRC is linked to the DST
fn is_linked(src: &PathBuf, dst: &PathBuf) -> bool {
    if let Ok(dst) = std::fs::read_link(dst) {
        return &dst == src;
    }

    false
}

/// Return the absolute path (SRC and DST)
fn absolute_path(src: &String, dst: &String) -> (PathBuf, PathBuf) {
    let src = std::fs::canonicalize(src).unwrap_or(PathBuf::from(src));
    let dst = PathBuf::from(&dst);

    (src, dst)
}

/// Create parent directories
fn create_parent(name: &String, dst: &PathBuf) -> Result<()> {
    if let Some(parent) = dst.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Skiped: [{}] {}", name, dst.display()))?
    }

    Ok(())
}

/// Check the differences between SRC and DST
///
/// Return `true`, If they are same.
/// Return `false`, If they aren't same.
fn diff_files(mut src: std::fs::File, mut dst: std::fs::File) -> bool {
    const BUF_SIZE: usize = 4096;

    let mut buff1 = [0u8; BUF_SIZE];
    let mut buff2 = [0u8; BUF_SIZE];

    loop {
        if let Ok(src_len) = std::io::Read::read(&mut src, &mut buff1) {
            if let Ok(dst_len) = std::io::Read::read(&mut dst, &mut buff2) {
                if src_len != dst_len {
                    return false;
                }
                if src_len == 0 {
                    return true;
                }
                if &buff1[0..src_len] != &buff2[0..dst_len] {
                    return false;
                }
            };
        }
    }
}
