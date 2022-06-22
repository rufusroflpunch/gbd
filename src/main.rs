use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let branches = Command::new("git").arg("branch").output()?;
    let out = String::from_utf8_lossy(&branches.stdout);

    println!("{}", branches.status);
    println!("{out:?}");

    return Ok(());
}
