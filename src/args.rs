use crate::profile::Profile;
use crate::rule::{Mode, Rule};

use anyhow::Result;
use argh::FromArgs;

/// A simple dotfile manager
#[derive(FromArgs, Debug)]
pub struct Dofi {
    #[argh(subcommand)]
    subcommand: Subcommand,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
enum Subcommand {
    Add(Add),
    Del(Del),
    Show(Show),
    List(List),
    Apply(Apply),
}

/// add rule
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "add")]
struct Add {
    /// rule name
    #[argh(positional)]
    rule: String,
    /// source path
    #[argh(positional)]
    src: String,
    /// target path
    #[argh(positional)]
    dst: String,
    /// apply method [copy, link]
    #[argh(option, short = 'm', default = "Mode::Link")]
    mode: Mode,
    /// profile name
    #[argh(option, short = 'p', default = "default_profile()")]
    profile: String,
    /// overwrite existing rule
    #[argh(switch, short = 'o')]
    overwrite: bool,
}

/// del rule
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "del")]
struct Del {
    /// rule name
    #[argh(positional)]
    rule: String,
    /// profile name
    #[argh(option, short = 'p', default = "default_profile()")]
    profile: String,
}

/// show rule information
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "show")]
struct Show {
    /// rule name
    #[argh(positional)]
    rule: String,
    /// profile name
    #[argh(option, short = 'p', default = "default_profile()")]
    profile: String,
}

/// list rules
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "list")]
struct List {
    /// profile name
    #[argh(option, short = 'p', default = "default_profile()")]
    profile: String,
}

/// apply rules
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "apply")]
struct Apply {
    /// profile name
    #[argh(option, short = 'p', default = "default_profile()")]
    profile: String,
}

fn default_profile() -> String {
    String::from("default")
}

impl Dofi {
    pub fn run(self) -> Result<()> {
        match self.subcommand {
            Subcommand::Add(Add {
                rule,
                src,
                dst,
                mode,
                profile,
                overwrite,
            }) => Profile::init(profile).add(rule, Rule { src, dst, mode }, overwrite)?,
            Subcommand::Del(Del { profile, rule }) => Profile::init(profile).del(rule)?,
            Subcommand::Show(Show { profile, rule }) => Profile::init(profile).show(rule)?,
            Subcommand::List(List { profile }) => Profile::init(profile).list()?,
            Subcommand::Apply(Apply { profile }) => Profile::init(profile).apply()?,
        }

        Ok(())
    }
}
