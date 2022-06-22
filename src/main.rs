use std::process::Command;

use dialoguer::{
    theme::{ColorfulTheme, SimpleTheme, Theme},
    Confirm, MultiSelect,
};
use gumdrop::Options;

#[derive(Debug, Options)]
pub struct CliArgs {
    #[options(help = "print help message")]
    help: bool,

    #[options(help = "select all options", default = "false")]
    select_all: bool,

    #[options(help = "disable colors", default = "false")]
    no_color: bool,
}

fn main() -> anyhow::Result<()> {
    let opts = CliArgs::parse_args_default_or_exit();
    let branches = Command::new("git").arg("branch").output()?;

    if !branches.status.success() {
        return Err(anyhow::anyhow!("Git branch not detected"));
    }

    let out = String::from_utf8_lossy(&branches.stdout);
    let branches: Vec<_> = out
        .lines()
        .filter(|i| !i.starts_with('*'))
        .map(|i| (i.trim(), opts.select_all))
        .collect();

    if branches.len() == 0 {
        eprintln!("No branches found.");
        return Ok(());
    }

    let theme = theme(&opts);

    let ms = MultiSelect::with_theme(theme.as_ref())
        .items_checked(&branches)
        .interact()?;

    let delete: Vec<_> = branches
        .iter()
        .enumerate()
        .filter(|(i, _x)| ms.contains(i))
        .map(|(_, s)| s)
        .collect();

    if Confirm::with_theme(theme.as_ref())
        .with_prompt("Are you sure you want to continue?")
        .interact()?
    {
        for (n, _) in delete {
            let status = Command::new("git").args(["branch", "-D"]).arg(n).status()?;
            if !status.success() {
                return Err(anyhow::anyhow!("Unable to remove branch {n}"));
            }
        }
    }

    return Ok(());
}

fn theme(opts: &CliArgs) -> Box<dyn Theme> {
    if opts.no_color {
        Box::new(SimpleTheme {})
    } else {
        Box::new(ColorfulTheme::default())
    }
}
