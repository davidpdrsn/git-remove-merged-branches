use anyhow::Result;
use extend::ext;
use serde::Deserialize;
use std::{
    process::{Command, Stdio},
    thread,
};

fn main() -> Result<()> {
    ensure_logged_in_to_github_cli()?;

    let thread_handles = branches()?
        .into_iter()
        .filter(|branch| branch != "main")
        .filter(|branch| branch != "master")
        .filter(|branch| branch != "(no branch)")
        .map(|branch| {
            thread::spawn(move || -> Result<_> {
                if let Some(state) = branch_state(&branch)? {
                    Ok(Some((branch, state)))
                } else {
                    eprintln!("no pr for `{}`, skipping", branch);
                    Ok(None)
                }
            })
        });

    let branch_and_states = thread_handles
        .filter_map(|handle| handle.join().expect("thread panicked").transpose())
        .collect::<Result<Vec<_>>>()?;

    for (branch, state) in branch_and_states {
        match state {
            PrState::Open => {
                eprintln!("`{}` not merged", branch);
            }
            PrState::Merged | PrState::Closed => {
                eprintln!("deleting `{}`", branch);
                Command::new("git")
                    .arg("branch")
                    .arg("-D")
                    .arg(&branch)
                    .run()?;
            }
        }
    }

    Ok(())
}

fn ensure_logged_in_to_github_cli() -> Result<()> {
    let status = Command::new("gh")
        .arg("auth")
        .arg("status")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    anyhow::ensure!(
        status.success(),
        "You're not logged in with the GitHub CLI. Run `gh auth login`"
    );
    Ok(())
}

fn branches() -> Result<Vec<String>> {
    let output = Command::new("git").arg("branch").run()?;
    let branches = output
        .lines()
        .map(|line| line.trim())
        .map(|line| line.strip_prefix("* ").unwrap_or(line))
        .map(|s| s.to_string())
        .collect();
    Ok(branches)
}

fn branch_state(branch: &str) -> Result<Option<PrState>> {
    let output = if let Ok(output) = Command::new("gh")
        .arg("pr")
        .arg("view")
        .arg(&branch)
        .arg("--json")
        .arg("state")
        .run()
    {
        output
    } else {
        return Ok(None);
    };

    let pr_view_output = serde_json::from_str::<PrViewOutput>(&output)?;

    Ok(Some(pr_view_output.state))
}

#[ext]
impl &mut Command {
    fn run(&mut self) -> Result<String> {
        let output = self.output().unwrap();

        if output.status.success() {
            Ok(String::from_utf8(output.stdout)?)
        } else {
            let stderr = String::from_utf8(output.stderr).unwrap();
            anyhow::bail!("Command failed: {:?}.\nSTDERR: {}", self, stderr)
        }
    }
}

#[derive(Deserialize)]
struct PrViewOutput {
    state: PrState,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum PrState {
    Merged,
    Open,
    Closed,
}
