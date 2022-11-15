use std::fmt::Display;
use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Rule {
    pub src: String,
    pub dst: String,
    pub mode: Mode,
}

#[derive(Serialize, Deserialize, Debug, Clone, clap::ValueEnum)]
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

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Copy => write!(f, "copy"),
            Mode::Link => write!(f, "link"),
        }
    }
}

impl Rule {
    pub fn apply(self, name: String) -> Result<()> {
        let src = std::fs::canonicalize(&self.src).unwrap_or(PathBuf::from(&self.src));
        let dst = PathBuf::from(&self.dst);

        if !src.exists() {
            bail!("Skiped: [{}] source {} not exists", name, src.display());
        }

        match self.mode {
            Mode::Copy => apply_copy(name, src, dst),
            Mode::Link => apply_link(name, src, dst),
        }
    }
}

fn apply_copy(name: String, src: PathBuf, dst: PathBuf) -> Result<()> {
    if dst.exists() {
        if is_copyed(&src, &dst) {
            return Ok(println!("Copyed: [{}]", name));
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

fn is_linked(src: &PathBuf, dst: &PathBuf) -> bool {
    if let Ok(dst) = std::fs::read_link(dst) {
        return &dst == src;
    }

    false
}

fn is_copyed(src: &PathBuf, dst: &PathBuf) -> bool {
    if let Err(_) = std::fs::read_link(&dst) {
        if let Ok(src) = std::fs::File::open(src) {
            if let Ok(dst) = std::fs::File::open(dst) {
                return diff_files(src, dst);
            }
        }
    }

    false
}

fn create_parent(name: &String, dst: &PathBuf) -> Result<()> {
    if let Some(parent) = dst.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Skiped: [{}] {}", name, dst.display()))?
    }

    Ok(())
}

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
