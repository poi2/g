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

### Shell function setup

Add shell functions to enable directory switching with `g wt switch` commands.

**zsh (.zshrc):**

```zsh
g() {
    if [[ "$1" == "wt" || "$1" == "worktree" ]] && [[ "$2" == "switch" ]]; then
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
    if [[ "$1" == "wt" || "$1" == "worktree" ]] && [[ "$2" == "switch" ]]; then
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
    if test "$argv[1]" = "wt" -o "$argv[1]" = "worktree"
        if test "$argv[2]" = "switch"
            set result (command g $argv)
            if test -n "$result" -a -d "$result"
                cd $result
            else
                echo $result
            end
            return
        end
    end
    command g $argv
end
```

## Quick Start

```bash
# 1. Clone a repository
g repo clone https://github.com/poi2/my-project.git

# 2. Create a worktree
cd ~/src/github.com/poi2/my-project
g wt -c feature/auth

# 3. Interactive switching (with fzf)
g wt switch -i  # Select from worktree list
g switch -i     # Select from branch list

# 4. Delete a worktree
g wt -d feature-auth
```

## Commands

### Repository Management

```bash
g repository clone <url>  # Clone a repository
g repo clone <url>        # alias
```

### Worktree Operations

```bash
g wt -c <branch>          # Create worktree + switch
g wt -l                   # List worktrees
g wt switch -i            # Select worktree with fzf + switch
g wt switch <branch>      # Switch to worktree by branch name
g wt switch -c <branch>   # Create worktree + switch
g wt -d <branch>          # Delete worktree
g wt -D <branch>          # Force delete worktree
```

### Branch Operations

```bash
g switch <branch>         # Switch branch (git switch wrapper)
g switch -i               # Select branch with fzf + switch
g switch -c <branch>      # Create and switch to new branch
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

## Environment Variables

```bash
# Worktree base directory (default: $HOME/src/.worktrees)
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
