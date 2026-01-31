# Sonic Git (`g`) - The Fastest Git Worktree Manager

⚡ **Sonic-speed Git worktree management at your fingertips.**

Rust-based CLI tool for managing Git worktrees with blazing-fast performance. Switch between branches, create worktrees, and manage parallel workflows at sonic speed.

## Features

- ⚡ **Sonic Speed**: Lightning-fast operations with minimal overhead
- 🚀 **Smart Structure**: Repository management with `$HOME/src/github.com/org/repo` structure
- 🌳 **Parallel Workflows**: Work on multiple branches simultaneously using Git Worktree
- 🔍 **Interactive Selection**: Fast branch/worktree switching with fzf integration
- 📁 **Hierarchical Organization**: Worktree management preserving branch structure

## Installation

### Prerequisites

- Rust (1.70+)
- Git (2.5+)
- fzf (optional, for interactive features)

### Build from source

```bash
git clone https://github.com/poi2/g.git
cd g
cargo install --path .
```

### Configuration

#### Git Config

Configure `g` command using `.gitconfig`:

```bash
# Set worktree base directory (default: $HOME/src/.worktrees)
git config --global sonic-git.root "$HOME/src"

# Add command aliases
git config --global sonic-git.alias.s "sonic-switch"
git config --global sonic-git.alias.si "sonic-switch -i"
git config --global sonic-git.alias.rc "sonic-repository clone"
```

#### Shell Function Setup

Add shell functions to enable directory switching with `g sonic-worktree switch` commands.

**zsh (.zshrc):**

```zsh
g() {
    if [[ "$1" == "sonic-worktree" ]] && [[ "$2" == "switch" ]]; then
        local result=$(command g "$@")
        if [ -n "$result" ] && [ -d "$result" ]; then
            cd "$result"
        else
            echo "$result"
        fi
    else
        command g "$@"
    fi
}
```

**bash (.bashrc):**

```bash
g() {
    if [[ "$1" == "sonic-worktree" ]] && [[ "$2" == "switch" ]]; then
        local result=$(command g "$@")
        if [ -n "$result" ] && [ -d "$result" ]; then
            cd "$result"
        else
            echo "$result"
        fi
    else
        command g "$@"
    fi
}
```

**fish (~/.config/fish/functions/g.fish):**

```fish
function g
    # Check if this might be a worktree/repository switch command
    # Handles both full commands and aliases
    set result (command g $argv)
    
    # If result is a directory path, cd to it
    if test -n "$result" -a -d "$result"
        cd $result
    else
        echo $result
    end
end
```

## Quick Start

```bash
# 1. Clone a repository
g sonic-repository clone https://github.com/poi2/my-project.git

# 2. Create a worktree
cd ~/src/github.com/poi2/my-project
g sonic-worktree -c feature/auth

# 3. Interactive switching (with fzf)
g sonic-worktree switch -i  # Select from worktree list
g sonic-switch -i           # Select from branch list

# 4. Delete a worktree
g sonic-worktree -d feature-auth

# 5. Use aliases (configured in .gitconfig)
git config --global sonic-git.alias.si "sonic-switch -i"
g si  # Alias to sonic-switch -i

# 6. Git passthrough - all non-sonic commands go to git
g status        # Same as: git status
g commit -m "foo"  # Same as: git commit -m "foo"
```

## Commands

### Sonic Commands

All custom commands use the `sonic-` prefix:

#### Repository Management

```bash
g sonic-repository clone <url>              # Clone a repository
g sonic-repository ls                       # List all repositories
g sonic-repository switch -i                # Switch to repository (interactive)
g sonic-repository switch <repo>           # Switch to repository
g sonic-repository delete -i                # Delete repository (interactive)
g sonic-repository delete <repo>           # Delete repository
g sonic-repository new <repo>              # Create new repository
```

#### Worktree Operations

```bash
g sonic-worktree new <branch>             # Create worktree
g sonic-worktree ls                       # List worktrees
g sonic-worktree mv <old> <new>           # Rename worktree (branch + directory)
g sonic-worktree mv <new>                 # Rename current worktree
g sonic-worktree switch -i                # Switch to worktree (interactive)
g sonic-worktree switch <branch>          # Switch to worktree
g sonic-worktree delete <branch>          # Delete worktree
g sonic-worktree delete -f <branch>       # Force delete worktree
g sonic-worktree delete -a                # Delete all except current
g sonic-worktree delete -a -f             # Force delete all except current
g sonic-worktree delete -i                # Delete worktree (interactive)
g sonic-worktree delete -i -f             # Force delete worktree (interactive)
```

#### Branch Operations

```bash
g sonic-switch <branch>                   # Switch branch
g sonic-switch -i                         # Switch branch (interactive)

g sonic-branch ls [options]               # List branches (git branch)
g sonic-branch new <branch>               # Create branch (git switch -c)
g sonic-branch mv <old> <new>             # Rename branch
g sonic-branch mv <new>                   # Rename current branch
g sonic-branch delete <branch>            # Delete branch
g sonic-branch delete -f <branch>         # Force delete branch
g sonic-branch delete -a                  # Delete all except base/current
g sonic-branch delete -a -f               # Force delete all except base/current
g sonic-branch delete -i                  # Delete merged branches (interactive)
g sonic-branch delete -i -f               # Delete branches (interactive)
```

### Git Passthrough

All non-sonic commands are passed through to git:

```bash
g status                 # git status
g commit -m "message"    # git commit -m "message"
g push origin main       # git push origin main
```

### Aliases

Configure custom aliases in `.gitconfig`:

```bash
# Example alias configuration
git config --global sonic-git.alias.s "sonic-switch"
git config --global sonic-git.alias.si "sonic-switch -i"

# Repository aliases
git config --global sonic-git.alias.rc "sonic-repository clone"
git config --global sonic-git.alias.rs "sonic-repository switch -i"
git config --global sonic-git.alias.rd "sonic-repository delete -i"
git config --global sonic-git.alias.rn "sonic-repository new"

# Branch aliases
git config --global sonic-git.alias.bl "sonic-branch ls"
git config --global sonic-git.alias.bn "sonic-branch new"
git config --global sonic-git.alias.bm "sonic-branch mv"
git config --global sonic-git.alias.bd "sonic-branch delete"

# Worktree aliases
git config --global sonic-git.alias.wtn "sonic-worktree new"
git config --global sonic-git.alias.wtl "sonic-worktree ls"
git config --global sonic-git.alias.wtm "sonic-worktree mv"
git config --global sonic-git.alias.wts "sonic-worktree switch"
git config --global sonic-git.alias.wtsi "sonic-worktree switch -i"
git config --global sonic-git.alias.wtd "sonic-worktree delete"

# Usage
g si              # sonic-switch -i
g rc <url>        # sonic-repository clone <url>
g rs              # sonic-repository switch -i
g bn feat         # sonic-branch new feat
g wtsi            # sonic-worktree switch -i
```

## Architecture

```
$HOME/src/
├── github.com/
│   └── poi2/
│       └── my-project/          # main branch
│           ├── .git/
│           └── src/
└── .worktrees/                  # $G_WORKTREE_BASE
    └── github.com/
        └── poi2/
            └── my-project/
                ├── feature/
                │   ├── auth/
                │   └── login/
                └── fix/
                    └── bug-123/
```

### Design Philosophy

- **Main branch is special**: Treated as a regular clone, preserving existing workflows
- **Worktrees are separated**: Placed in `.worktrees/` to avoid build artifact conflicts
- **Hierarchical structure preserved**: Branch names like `feature/auth` maintain directory hierarchy

## Configuration

### Git Config

```bash
# Worktree base directory (default: $HOME/src/.worktrees)
git config --global sonic-git.root "$HOME/src"

# Command aliases
git config --global sonic-git.alias.s "sonic-switch"
git config --global sonic-git.alias.si "sonic-switch -i"
```

### Environment Variables (Legacy)

```bash
# Still supported for backward compatibility
export G_WORKTREE_BASE="/custom/path/.worktrees"
```

## Troubleshooting

### fzf not found

```bash
# macOS
brew install fzf

# Ubuntu/Debian
apt install fzf

# Fedora
dnf install fzf
```

### Directory already exists error

A directory with the same name already exists. Use a different name or remove the existing directory first.

```bash
rm -rf <worktree-path>
```

## License

MIT

## Contributing

Pull requests are welcome!
