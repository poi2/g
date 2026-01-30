# Sonic Git Branding - Remaining Tasks

This document tracks the remaining branding tasks for the Sonic Git project that require manual intervention or are outside the scope of automated changes.

## Completed ✅

- [x] Update Cargo.toml with project name "sonic-git"
- [x] Add description emphasizing speed
- [x] Update README with Sonic Git branding
- [x] Add tagline "The Fastest Git Worktree Manager"
- [x] Add speed-focused feature descriptions

## Remaining Tasks

### 1. GitHub Repository Settings

**Repository Description** (Settings → General)
```
⚡ Sonic Git - The fastest Git worktree manager. Lightning-fast parallel branch workflows with `g` CLI.
```

**Repository Topics** (Settings → General → Topics)
```
git worktree cli rust speed productivity git-tools sonic-git
```

**Repository URL/Name** (Optional)
- Current: `github.com/poi2/g`
- Consider: Keep the short URL `g` for the command alignment, but rely on "Sonic Git" branding in description

### 2. Visual Identity (Optional)

**Logo Ideas:**
- Sonic wave visualization
- Lightning bolt + Git branch
- Speedometer with Git logo
- Blue/electric color scheme to match "sonic" theme

**Where to add:**
- GitHub social preview image (1280x640px)
- README header (optional)
- Documentation site (if created)

**Tools for creating:**
- Figma
- Canva
- DALL-E/Midjourney for AI generation
- Hire a designer on Fiverr

### 3. Metadata Updates

**Cargo.toml** (Already done, but for reference)
```toml
name = "sonic-git"
description = "The fastest Git worktree manager - parallel branch workflow at sonic speed"
repository = "https://github.com/poi2/g"
license = "MIT"
keywords = ["git", "worktree", "cli", "speed", "productivity"]
categories = ["command-line-utilities", "development-tools"]
```

### 4. External Platform Updates

**crates.io** (When published)
- Package will be listed as "sonic-git"
- Binary name stays as "g"
- Description will appear in search results

**Search Engine Optimization**
- "sonic git" is now searchable and unique
- GitHub repo description will improve Google indexing
- Consider adding documentation site for better SEO

### 5. Community & Marketing

**Announcement Ideas:**
- Reddit: r/rust, r/git
- Hacker News
- Twitter/X with #RustLang #Git hashtags
- Dev.to blog post explaining the speed focus

**Key Messages:**
- "Why `g`? Because speed matters. Sonic Git is built for velocity."
- "The fastest way to manage Git worktrees"
- "Rust-powered performance for parallel Git workflows"

## Notes

- The ultra-short command name `g` is now justified by the speed theme
- "Sonic Git" provides searchability while `g` provides usability
- All code changes preserve backward compatibility
- Binary name remains `g` for minimal typing

## Priority

1. **High**: Update GitHub repository description and topics (5 minutes)
2. **Medium**: Create basic logo/visual identity (1-2 hours or hire designer)
3. **Low**: Marketing announcements (when ready for users)

## Files Modified in This PR

- `Cargo.toml`: Package metadata with sonic-git branding
- `README.md`: Header, tagline, and feature descriptions
- `BRANDING_TODO.md`: This file (remaining tasks)
