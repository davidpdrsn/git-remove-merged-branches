# git-remove-merged-branches

CLI to remove git branches who's PR has been merged.

`git branch -d BRANCH` isn't good at determining if a branch has been merged if
you squash before merging. We can get around that by looking at the PR status.

## Setup

- [Install Rust](https://www.rust-lang.org/tools/install).
- [Install GitHub CLI](https://github.com/cli/cli).
- Run `cargo install --git https://github.com/davidpdrsn/git-remove-merged-branches`
- Thats it! You can now run `git remove-merged-branches`.
