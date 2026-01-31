#!/bin/bash

# Sonic Git Configuration Installer
# This script installs recommended aliases to your global git config

echo "Installing Sonic Git configuration..."

# Set repository root
git config --global sonic-git.root "$HOME/src"

# Repository aliases
git config --global sonic-git.alias.rc "sonic-repository clone"
git config --global sonic-git.alias.rs "sonic-repository switch -i"
git config --global sonic-git.alias.rd "sonic-repository delete -i"
git config --global sonic-git.alias.rn "sonic-repository new"

# Switch aliases
git config --global sonic-git.alias.s "sonic-switch"
git config --global sonic-git.alias.si "sonic-switch -i"

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

echo "✓ Configuration installed successfully!"
echo ""
echo "Try these commands:"
echo "  g si          # sonic-switch -i (interactive branch switch)"
echo "  g rs          # sonic-repository switch -i (switch repository)"
echo "  g bn feat     # sonic-branch new feat (create branch)"
echo "  g wtsi        # sonic-worktree switch -i (switch worktree)"
echo ""
echo "View all aliases:"
echo "  git config --get-regexp sonic-git.alias"
