use crate::rule::{Rule, Rules};

use anyhow::{bail, Context, Result};
use std::path::PathBuf;

const DOFI_DIR: &'static str = ".dofi";

#[derive(Debug)]
pub struct Profile {
    pub profile: String,
    pub rules: Rules,
}

impl Profile {
    /// Init `Profile` struct from a profile name
    pub fn init(profile: String) -> Profile {
        Profile {
            profile,
            rules: Rules::new(),
        }
    }

    /// Add a rule
    pub fn add(mut self, rule: String, mut data: Rule, overwrite: bool) -> Result<()> {
        self.read()?;

        if self.rules.contains_key(&rule) && !overwrite {
            bail!("Rule [{}]: Duplicate (use -f to overwrite)", rule);
        }

        shrink_home(&mut data)?;

        println!("Add [{}]", rule);

        self.rules.insert(rule, data);
        self.write()?;

        Ok(())
    }

    /// Delete a rule
    pub fn del(mut self, rule: String) -> Result<()> {
        self.read()?;
        self.is_empty()?;

        if !self.rules.contains_key(&rule) {
            bail!("Rule [{}]: Not found", rule);
        }

        println!("Delete [{}]", rule);

        self.rules.remove(&rule);
        self.write()?;

        Ok(())
    }

    /// Show rule information
    pub fn show(mut self, rule: String) -> Result<()> {
        self.read()?;
        self.is_empty()?;

        match self.rules.get(&rule) {
            Some(data) => println!("[{}]\n{}", rule, data),
            None => bail!("Rule [{}]: Not found", rule),
        }

        Ok(())
    }

    /// List rules
    pub fn list(mut self) -> Result<()> {
        self.read()?;
        self.is_empty()?;

        for (rule, _) in self.rules {
            println!("{}", rule);
        }

        Ok(())
    }

    /// Apply rules
    pub fn apply(mut self) -> Result<()> {
        self.read()?;
        self.is_empty()?;

        for (rule, mut data) in self.rules {
            expand_home(&mut data)?;

            match data.apply(rule) {
                Ok(_) => (),
                Err(e) => eprintln!("{:#}", e),
            };
        }

        Ok(())
    }

    /// Read profile data
    fn read(&mut self) -> Result<()> {
        let path = profile_path(&self.profile);

        if let Ok(data) = std::fs::read_to_string(path) {
            self.rules = toml::from_str(&data)
                .with_context(|| format!("Profile [{}]: Failed to decode", self.profile))?;
        }

        Ok(())
    }

    /// Write profile data
    fn write(self) -> Result<()> {
        let _ = std::fs::create_dir(DOFI_DIR);

        let path = profile_path(&self.profile);
        let data = toml::to_string(&self.rules)
            .with_context(|| format!("Profile [{}]: Failed to encode", self.profile))?;

        std::fs::write(&path, data)
            .with_context(|| format!("Profile [{}]: Failed to write", self.profile))?;

        Ok(())
    }

    /// Check profile is empty or not
    fn is_empty(&self) -> Result<()> {
        if self.rules.is_empty() {
            bail!("Profile [{}]: Empty profile", self.profile);
        }

        Ok(())
    }
}

/// Return profile path
///
/// `.dofi/PROFILE_NAME.toml`
fn profile_path(profile: &String) -> PathBuf {
    let mut path = PathBuf::from(DOFI_DIR).join(profile);

    path.set_extension("toml");

    path
}

/// Expand ~ to user homedir
///
/// `~/PATH -> /home/username/PATH`
fn expand_home(data: &mut Rule) -> Result<()> {
    for path in [&mut data.src, &mut data.dst] {
        if path.starts_with("~") {
            if let Some(home) = home_dir() {
                if let Some(home) = home.to_str() {
                    path.remove(0);
                    path.insert_str(0, home);
                    return Ok(());
                }
            }
            bail!("Error: Failed to expand ~ to home path");
        }
    }

    Ok(())
}

/// Shrink user homedir to ~
///
/// `/home/username/PATH -> ~/PATH`
fn shrink_home(data: &mut Rule) -> Result<()> {
    for path in [&mut data.src, &mut data.dst] {
        if let Some(home) = home_dir() {
            if let Some(home) = home.to_str() {
                if path.starts_with(home) {
                    path.replace_range(0..home.len(), "~");
                }
            }
        }
    }
    Ok(())
}

/// Get user homedir
fn home_dir() -> Option<PathBuf> {
    #[allow(deprecated)]
    std::env::home_dir()
}
