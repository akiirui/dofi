mod profile;
mod rule;

use crate::profile::Profile;
use crate::rule::{Mode, Rule};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version = option_env!("DOFI_VERSION").unwrap_or(env!("CARGO_PKG_VERSION")))]
enum Cli {
    #[command(display_order = 1, about = "Add a rule")]
    Add {
        #[arg(help = "Rule name")]
        rule: String,
        #[arg(help = "Path (relative or absolute)")]
        src: String,
        #[arg(help = "Path (absolute)")]
        dst: String,
        #[arg(
            short,
            value_enum,
            help = "Rule mode",
            default_value = "link",
            hide_possible_values(true)
        )]
        mode: Mode,
        #[arg(short, help = "Profile name", default_value = "default")]
        profile: String,
    },
    #[command(display_order = 2, about = "Delete a rule")]
    Del {
        #[arg(help = "Rule name")]
        rule: String,
        #[arg(short, help = "Profile name", default_value = "default")]
        profile: String,
    },
    #[command(display_order = 3, about = "Show rule information")]
    Show {
        #[arg(help = "Rule name")]
        rule: String,
        #[arg(short, help = "Profile name", default_value = "default")]
        profile: String,
    },
    #[command(display_order = 4, about = "List rules")]
    List {
        #[arg(help = "Profile name", default_value = "default")]
        profile: String,
    },
    #[command(display_order = 5, about = "Apply rules")]
    Apply {
        #[arg(help = "Profile name", default_value = "default")]
        profile: String,
    },
}

fn main() {
    let mut p = Profile::new();
    let r = match Cli::parse() {
        Cli::Add {
            rule,
            src,
            dst,
            mode,
            profile,
        } => {
            p.profile = profile;
            p.add(rule, Rule { src, dst, mode })
        }
        Cli::Del { rule, profile } => {
            p.profile = profile;
            p.del(rule)
        }
        Cli::Show { rule, profile } => {
            p.profile = profile;
            p.show(rule)
        }
        Cli::List { profile } => {
            p.profile = profile;
            p.list()
        }
        Cli::Apply { profile } => {
            p.profile = profile;
            p.apply()
        }
    };

    match r {
        Ok(_) => (),
        Err(e) => eprintln!("{:?}", e),
    }
}
