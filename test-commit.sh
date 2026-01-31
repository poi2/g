#!/bin/bash
set -e

cd /Users/id/src/github.com/poi2/g

# Check current branch
git branch --show-current

# Stage changes
git add -A

# Show status
git status

# Create commit
git commit -m "fix: correct git worktree add argument order and suppress fatal messages

Fixed two issues:

1. git worktree add argument order bug
   - Base branch was placed before path, causing 'invalid reference' error
   - Correct order: git worktree add -b <branch> <path> [<start-point>]
   - Fixed: g wtn foo --base main now works correctly

2. Suppress fatal messages from check_branch_exists()
   - Added stdout/stderr redirection to Stdio::null()
   - Eliminates confusing 'fatal: not a valid ref' messages
   - Clean output for normal worktree creation

3. Improve fish shell function
   - Simplified logic to detect directory output
   - Works with aliases (wtsi, rs, etc)
   - Updated README example

Fixes reported issue with: g wtn feature/wt-list-worktree --base feature/wt-list

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>"

echo "Commit created successfully"
