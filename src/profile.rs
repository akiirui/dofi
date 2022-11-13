use crate::rule::Rule;

use std::collections::BTreeMap;
use std::path::PathBuf;

use thiserror::Error;

const DOFI_DIR: &'static str = ".dofi";

#[derive(Error, Debug)]
pub enum ProfileError {
    #[error("Duplicate rule [{0}]")]
    RuleDuplicate(String),
    #[error("Not found rule [{0}]")]
    RuleNotFound(String),
    #[error("Profile <{0}> is empty")]
    ProfileEmpty(String),
    #[error("Failed to expand ~ as home path")]
    ExpandHomeFailed,
    #[error(transparent)]
    StdIo(#[from] std::io::Error),
    #[error("Profile decode failed. {0}")]
    TomlDeError(#[from] toml::de::Error),
    #[error(transparent)]
    TomlSerError(#[from] toml::ser::Error),
}

#[derive(Debug)]
pub struct Profile {
    pub profile: String,
    pub rules: BTreeMap<String, Rule>,
}

impl Profile {
    pub fn new() -> Profile {
        Profile {
            profile: String::new(),
            rules: BTreeMap::new(),
        }
    }

    pub fn add(mut self, rule: String, mut data: Rule) -> Result<(), ProfileError> {
        self.read()?;

        if self.rules.contains_key(&rule) {
            return Err(ProfileError::RuleDuplicate(rule));
        }

        shrink_home(&mut data.src)?;
        shrink_home(&mut data.dst)?;

        println!("Add [{0}]\n", rule);
        self.rules.insert(rule, data);
        self.write()?;

        Ok(())
    }

    pub fn del(mut self, rule: String) -> Result<(), ProfileError> {
        self.read()?;

        if !self.rules.contains_key(&rule) {
            return Err(ProfileError::RuleNotFound(rule));
        }

        println!("Delete [{0}]", rule);
        self.rules.remove(&rule);
        self.write()?;

        Ok(())
    }

    pub fn apply(mut self) -> Result<(), ProfileError> {
        self.read()?;

        if self.rules.is_empty() {
            return Err(ProfileError::ProfileEmpty(self.profile));
        }

        for (name, mut rule) in self.rules {
            expand_home(&mut rule.src)?;
            expand_home(&mut rule.dst)?;
            match rule.apply(name) {
                Ok(_) => (),
                Err(error) => eprintln!("{}", error),
            };
        }

        Ok(())
    }

    pub fn list(mut self, full: bool) -> Result<(), ProfileError> {
        self.read()?;

        if self.rules.is_empty() {
            return Err(ProfileError::ProfileEmpty(self.profile));
        }

        for (rule, data) in self.rules.iter() {
            if full {
                println!("[{}]", rule);
                println!("Source: {}", data.src);
                println!("Target: {}", data.dst);
                println!("Mode  : {}", data.mode);
            } else {
                println!("{}", rule);
            }
        }

        Ok(())
    }

    fn read(&mut self) -> Result<(), ProfileError> {
        let path = profile_path(&self.profile);

        if let Ok(data) = std::fs::read(path) {
            self.rules = toml::from_slice(&data)?;
        }

        Ok(())
    }

    fn write(&self) -> Result<(), ProfileError> {
        let _ = std::fs::create_dir(DOFI_DIR);

        let path = profile_path(&self.profile);
        let data = toml::to_string(&self.rules)?;

        std::fs::write(&path, data)?;

        Ok(())
    }
}

fn profile_path(profile: &String) -> PathBuf {
    let mut path = PathBuf::from(DOFI_DIR).join(profile);
    path.set_extension("toml");

    path
}

fn expand_home(path: &mut String) -> Result<(), ProfileError> {
    if path.starts_with("~") {
        if let Some(home) = dirs::home_dir() {
            if let Some(home) = home.to_str() {
                path.remove(0);
                path.insert_str(0, home);
                return Ok(());
            }
        }
        return Err(ProfileError::ExpandHomeFailed);
    }

    Ok(())
}

fn shrink_home(path: &mut String) -> Result<(), ProfileError> {
    if let Some(home) = dirs::home_dir() {
        if let Some(home) = home.to_str() {
            if path.starts_with(home) {
                path.replace_range(0..home.len(), "~");
            }
        }
    }

    Ok(())
}
