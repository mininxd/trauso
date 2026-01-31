# GitHub Workflows

This directory contains GitHub Actions workflows for continuous integration and automated releases.

## Workflows

- `ci.yml` - Runs on every push and pull request to main branch to ensure the application builds correctly
- `release.yml` - Creates a GitHub release with artifacts when a new version tag is pushed

## Release Process

To create a new release:

1. Update the version in `package.json` and `src-tauri/tauri.conf.json`
2. Create and push a new tag (e.g., `git tag v1.0.3 && git push origin v1.0.3`)
3. The release workflow will automatically build the application for all platforms and create a GitHub release