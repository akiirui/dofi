use crate::rule::Rule;

use std::collections::BTreeMap;
use std::path::PathBuf;

use anyhow::{bail, Context, Result};

const DOFI_DIR: &'static str = ".dofi";

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

    pub fn add(mut self, rule: String, mut data: Rule) -> Result<()> {
        self.read()?;

        if self.rules.contains_key(&rule) {
            bail!("Duplicate rule [{}]", rule);
        }

        shrink_home(&mut data.src)?;
        shrink_home(&mut data.dst)?;

        println!("Add [{}]", rule);

        self.rules.insert(rule, data);
        self.write()?;

        Ok(())
    }

    pub fn del(mut self, rule: String) -> Result<()> {
        self.read()?;

        if !self.rules.contains_key(&rule) {
            bail!("Not found rule [{}]", rule);
        }

        println!("Delete [{}]", rule);

        self.rules.remove(&rule);
        self.write()?;

        Ok(())
    }

    pub fn show(mut self, rule: String) -> Result<()> {
        self.read()?;

        if self.rules.is_empty() {
            bail!("Profile [{}] is empty", self.profile);
        }

        match self.rules.get(&rule) {
            Some(v) => println!("[{}]\n{}", rule, v),
            None => bail!("Not found rule [{}]", rule),
        }

        Ok(())
    }

    pub fn list(mut self) -> Result<()> {
        self.read()?;

        if self.rules.is_empty() {
            bail!("Profile [{}] is empty", self.profile);
        }

        for (k, _) in self.rules {
            println!("{}", k);
        }

        Ok(())
    }

    pub fn apply(mut self) -> Result<()> {
        self.read()?;

        if self.rules.is_empty() {
            bail!("Profile [{}] is empty", self.profile);
        }

        for (k, mut v) in self.rules {
            expand_home(&mut v.src)?;
            expand_home(&mut v.dst)?;

            match v.apply(k) {
                Ok(_) => (),
                Err(e) => eprintln!("{:#}", e),
            };
        }

        Ok(())
    }

    fn read(&mut self) -> Result<()> {
        let path = profile_path(&self.profile);

        let data = std::fs::read(path)
            .with_context(|| format!("Failed to read profile [{}]", self.profile))?;
        self.rules = toml::from_slice(&data)
            .with_context(|| format!("Failed to decode profile [{}]", self.profile))?;

        Ok(())
    }

    fn write(self) -> Result<()> {
        let _ = std::fs::create_dir(DOFI_DIR);

        let path = profile_path(&self.profile);
        let data = toml::to_string(&self.rules)
            .with_context(|| format!("Failed to encode profile [{}]", self.profile))?;

        std::fs::write(&path, data)
            .with_context(|| format!("Failed to write profile [{}]", self.profile))?;

        Ok(())
    }
}

fn profile_path(profile: &String) -> PathBuf {
    let mut path = PathBuf::from(DOFI_DIR).join(profile);

    path.set_extension("toml");

    path
}

fn expand_home(path: &mut String) -> Result<()> {
    if path.starts_with("~") {
        if let Some(home) = dirs::home_dir() {
            if let Some(home) = home.to_str() {
                path.remove(0);
                path.insert_str(0, home);
                return Ok(());
            }
        }
        bail!("Failed to expand ~ to home path");
    }

    Ok(())
}

fn shrink_home(path: &mut String) -> Result<()> {
    if let Some(home) = dirs::home_dir() {
        if let Some(home) = home.to_str() {
            if path.starts_with(home) {
                path.replace_range(0..home.len(), "~");
            }
        }
    }

    Ok(())
}
