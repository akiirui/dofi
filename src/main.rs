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
        #[arg(short, help = "Profile name", default_value = "default")]
        profile: String,
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
        #[arg(short = 'f', help = "Overwrite existing rule")]
        overwrite: bool,
    },
    #[command(display_order = 2, about = "Delete a rule")]
    Del {
        #[arg(short, help = "Profile name", default_value = "default")]
        profile: String,
        #[arg(help = "Rule name")]
        rule: String,
    },
    #[command(display_order = 3, about = "Show rule information")]
    Show {
        #[arg(short, help = "Profile name", default_value = "default")]
        profile: String,
        #[arg(help = "Rule name")]
        rule: String,
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
    let r = match Cli::parse() {
        Cli::Add {
            profile,
            rule,
            src,
            dst,
            mode,
            overwrite,
        } => Profile::init(profile).add(rule, Rule { src, dst, mode }, overwrite),
        Cli::Del { profile, rule } => Profile::init(profile).del(rule),
        Cli::Show { profile, rule } => Profile::init(profile).show(rule),
        Cli::List { profile } => Profile::init(profile).list(),
        Cli::Apply { profile } => Profile::init(profile).apply(),
    };

    match r {
        Ok(_) => (),
        Err(e) => eprintln!("{:?}", e),
    }
}
