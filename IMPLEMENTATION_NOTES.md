# Implementation Notes for Issue #28 and #27

## Issue Analysis

After reviewing the three issues (#27, #28, #29), I've identified a critical scope concern that requires user input before proceeding.

## The Problem

Issue #28 proposes a **massive redesign** that goes far beyond simple renaming:

### Proposed in Issue #28:
1. **New command structure**:
   - `g sonic-repository clone/ls/switch/delete/new`
   - `g sonic-switch -i`
   - `g sonic-branch ls/new/mv/delete`

2. **Multiple new features not currently implemented**:
   - `sonic-repository ls` - list repositories
   - `sonic-repository switch` - switch between repositories
   - `sonic-repository delete` - delete repositories
   - `sonic-repository new` - create new repository
   - `sonic-branch ls` - list branches
   - `sonic-branch new` - create branch
   - `sonic-branch mv` - move/rename branch
   - `sonic-branch delete` - delete branch

3. **Extensive config/alias system**:
   - Default aliases for all new commands
   - Config-based customization

### Current Implementation:
- `g repository clone` (or `g repo clone`)
- `g worktree` commands (`wt -c`, `wt -l`, `wt -d`, `wt switch`)
- `g switch` (branch switching with fzf)

## Questions for User

### Option A: Minimal Implementation (Rename Only)
Simply rename existing commands to sonic-* namespace:
- `g repository` → `g sonic-repository`
- `g worktree` → Keep as-is (or rename to `g sonic-worktree`?)
- `g switch` → `g sonic-switch`
- No new features added

**Time**: 1-2 hours
**Risk**: Low
**Closes**: #27, partially addresses #28

### Option B: Full Implementation
Implement everything in #28:
- All sonic-* renames
- All new repository management commands
- All new branch management commands
- Full alias system integration with .gitconfig
- Update README and documentation

**Time**: 8-12 hours
**Risk**: High (major redesign)
**Closes**: #27, #28 completely

### Option C: Phased Approach
Phase 1 (now): Rename existing commands + git passthrough
- Rename to sonic-* namespace
- Add git command passthrough for everything else
- Basic alias support from .gitconfig

Phase 2 (later): Add new features incrementally
- sonic-repository ls/switch/delete/new
- sonic-branch commands
- Extended alias system

**Time**: Phase 1: 2-3 hours, Phase 2: TBD
**Risk**: Medium
**Closes**: #27 immediately, #28 over time

## My Recommendation

I cannot proceed with Issue #28 as written without clarification because:

1. **Scope is unclear**: Is this a rename or a complete redesign?
2. **New features not specified**: Behavior of `sonic-repository ls`, `sonic-branch mv`, etc. not defined
3. **Breaking changes**: Will affect all users and documentation

## What I Can Do Now

I can implement **Option A** (minimal rename) for Issue #27:
- Rename `g switch` to `g sonic-switch`
- Keep all other commands as-is
- Update documentation

OR

I can write detailed questions in this file about each proposed new feature in #28 for user review.

## Action Required

Please advise:
1. Should I proceed with Option A (minimal, safe)?
2. Should I proceed with Option C Phase 1 (moderate, recommended)?
3. Should I halt and wait for clarification on #28 scope?
4. Something else?
