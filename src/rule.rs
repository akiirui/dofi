use serde::{Deserialize, Serialize};
use thiserror::Error;

use std::path::PathBuf;

#[derive(Error, Debug)]
pub enum RuleError {
    #[error("Skiped: [{0}] source {1} not exists")]
    SrcNotExists(String, std::path::PathBuf),
    #[error("Skiped: [{0}] target {1} exists")]
    DstExists(String, std::path::PathBuf),
    #[error("Skiped: [{0}] unknown mode {1}")]
    ModeUnknown(String, String),
    #[error("Skiped: [{0}] {1} {2}")]
    Io(String, std::path::PathBuf, std::io::Error),
}

#[derive(Error, Debug)]
pub enum RuleInfo {
    #[error("Linkto: [{0}] > {1}")]
    Link(String, std::path::PathBuf),
    #[error("Copyto: [{0}] > {1}")]
    Copy(String, std::path::PathBuf),
    #[error("Linked: [{0}]")]
    Linked(String),
    #[error("Copyed: [{0}]")]
    Copyed(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rule {
    pub src: String,
    pub dst: String,
    pub mode: String,
}

impl Rule {
    pub fn apply(self, name: String) -> Result<RuleInfo, RuleError> {
        let src = std::fs::canonicalize(&self.src).unwrap_or(PathBuf::from(&self.src));
        let dst = PathBuf::from(&self.dst);

        if !src.exists() {
            return Err(RuleError::SrcNotExists(name, src));
        }

        match self.mode.as_str() {
            "symlink" => apply_link(name, src, dst),
            "copy" => apply_copy(name, src, dst),
            _ => Err(RuleError::ModeUnknown(name, self.mode)),
        }
    }
}

fn apply_link(name: String, src: PathBuf, dst: PathBuf) -> Result<RuleInfo, RuleError> {
    if dst.exists() {
        if is_linked(&src, &dst) {
            return Ok(RuleInfo::Linked(name));
        }
        return Err(RuleError::DstExists(name, dst));
    }

    create_parent(&name, &dst)?;

    match std::os::unix::fs::symlink(&src, &dst) {
        Ok(_) => Ok(RuleInfo::Link(name, dst)),
        Err(err) => Err(RuleError::Io(name, dst, err)),
    }
}

fn apply_copy(name: String, src: PathBuf, dst: PathBuf) -> Result<RuleInfo, RuleError> {
    if dst.exists() {
        if is_copyed(&src, &dst) {
            return Ok(RuleInfo::Copyed(name));
        }
        return Err(RuleError::DstExists(name, dst));
    }

    create_parent(&name, &dst)?;

    match std::fs::copy(&src, &dst) {
        Ok(_) => Ok(RuleInfo::Copy(name, dst)),
        Err(err) => Err(RuleError::Io(name, dst, err)),
    }
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

fn create_parent(name: &String, dst: &PathBuf) -> Result<(), RuleError> {
    if let Some(parent) = dst.parent() {
        if let Err(err) = std::fs::create_dir_all(parent) {
            return Err(RuleError::Io(name.clone(), parent.to_path_buf(), err));
        }
    }

    Ok(())
}

pub fn diff_files(mut src: std::fs::File, mut dst: std::fs::File) -> bool {
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
