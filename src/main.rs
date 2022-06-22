use std::process::Command;

use dialoguer::{theme::ColorfulTheme, Confirm, MultiSelect};

fn main() -> anyhow::Result<()> {
    let branches = Command::new("git").arg("branch").output()?;

    if !branches.status.success() {
        return Err(anyhow::anyhow!("Git branch not detected"));
    }

    let out = String::from_utf8_lossy(&branches.stdout);
    let branches: Vec<_> = out
        .lines()
        .filter(|i| !i.starts_with('*'))
        .map(|i| i.trim())
        .collect();

    if branches.len() == 0 {
        eprintln!("No branches found.");
        return Ok(());
    }

    let ms = MultiSelect::with_theme(&ColorfulTheme::default())
        .items(&branches)
        .interact()?;

    let delete: Vec<_> = branches
        .iter()
        .enumerate()
        .filter(|(i, _x)| ms.contains(i))
        .map(|(_, s)| s)
        .collect();

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Are you sure you want to continue?")
        .interact()?
    {
        for n in delete {
            let status = Command::new("git").args(["branch", "-D"]).arg(n).status()?;
            if !status.success() {
                return Err(anyhow::anyhow!("Unable to remove branch {n}"));
            }
        }
    }

    return Ok(());
}
