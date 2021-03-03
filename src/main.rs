use structopt::StructOpt;

mod profile;
mod rule;

use crate::profile::Profile;
use crate::rule::Rule;

#[derive(Debug, StructOpt)]
#[structopt(name = env!("CARGO_PKG_NAME"))]
#[structopt(version = option_env!("DOFI_VERSION").unwrap_or(env!("CARGO_PKG_VERSION")))]
#[structopt(author = env!("CARGO_PKG_AUTHORS"))]
#[structopt(about = env!("CARGO_PKG_DESCRIPTION"))]
enum Opt {
    #[structopt(display_order = 1, about = "Add rule to profile")]
    Add {
        #[structopt(help = "Rule name")]
        rule: String,
        #[structopt(help = "Path (relative or absolute)")]
        src: String,
        #[structopt(help = "Path (absolute)")]
        dst: String,
        #[structopt(
            short,
            long,
            help = "Rule mode",
            default_value = "symlink",
            possible_value("symlink"),
            possible_value("copy"),
            hide_possible_values(true)
        )]
        mode: String,
        #[structopt(short, long, help = "Profile name", default_value = "default")]
        profile: String,
    },
    #[structopt(display_order = 2, about = "Delete rule from profile")]
    Del {
        #[structopt(help = "Rule name")]
        rule: String,
        #[structopt(short, long, help = "Profile name", default_value = "default")]
        profile: String,
    },
    #[structopt(display_order = 3, about = "Apply profile rules")]
    Apply {
        #[structopt(help = "Profile name", default_value = "default")]
        profile: String,
    },
    #[structopt(display_order = 4, about = "List rules of profile")]
    List {
        #[structopt(help = "Profile name", default_value = "default")]
        profile: String,
        #[structopt(short, long, help = "Prints full infomations")]
        full: bool,
    },
}
fn main() {
    let opt = Opt::from_args();
    let result = match opt {
        Opt::Add {
            rule,
            src,
            dst,
            mode,
            profile,
        } => {
            let r = Rule { src, dst, mode };
            let mut p = Profile::new();
            p.profile = profile;
            p.add(rule, r)
        }
        Opt::Del { rule, profile } => {
            let mut p = Profile::new();
            p.profile = profile;
            p.del(rule)
        }
        Opt::Apply { profile } => {
            let mut p = Profile::new();
            p.profile = profile;
            p.apply()
        }
        Opt::List { profile, full } => {
            let mut p = Profile::new();
            p.profile = profile;
            p.list(full)
        }
    };
    match result {
        Ok(info) => print!("{}", info),
        Err(error) => eprintln!("{}", error),
    }
}
