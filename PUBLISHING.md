# Publishing Setup Guide

This guide explains how to set up the GitHub repository for automatic publishing to crates.io and docs.rs.

## Prerequisites

1. A GitHub repository at `https://github.com/indyzai/oracledb-rs`
2. A crates.io account
3. Admin access to the GitHub repository

## Step 1: Get Your crates.io API Token

1. Log in to [crates.io](https://crates.io/)
2. Go to Account Settings → API Tokens
3. Click "New Token"
4. Give it a name (e.g., "GitHub Actions")
5. Copy the generated token (you won't be able to see it again!)

## Step 2: Add GitHub Secret

1. Go to your GitHub repository
2. Navigate to Settings → Secrets and variables → Actions
3. Click "New repository secret"
4. Name: `CARGO_REGISTRY_TOKEN`
5. Value: Paste your crates.io API token
6. Click "Add secret"

## Step 3: Enable GitHub Pages (for documentation)

1. Go to Settings → Pages
2. Source: Select "GitHub Actions"
3. Save

## Step 4: Publishing a New Version

### Option 1: Using Git Tags (Recommended)

```bash
# Update version in Cargo.toml
# Update CHANGELOG.md with changes

# Commit changes
git add Cargo.toml CHANGELOG.md
git commit -m "Bump version to 0.1.0"

# Create and push tag
git tag v0.1.0
git push origin main
git push origin v0.1.0
```

This will automatically:
- Run all tests
- Publish to crates.io
- Create a GitHub release
- Build and deploy documentation

### Option 2: Manual Publishing

```bash
# Ensure you're logged in to crates.io
cargo login

# Publish
cargo publish
```

## Workflow Files

The following GitHub Actions workflows have been created:

### `.github/workflows/ci.yml`
- Runs on every push and PR
- Tests on Linux, macOS, and Windows
- Tests with stable and beta Rust
- Runs formatting and clippy checks
- Generates code coverage

### `.github/workflows/publish.yml`
- Triggers on version tags (v*.*.*)
- Verifies version matches tag
- Publishes to crates.io

### `.github/workflows/docs.yml`
- Builds documentation
- Deploys to GitHub Pages
- Runs on main branch pushes and tags

### `.github/workflows/release.yml`
- Creates GitHub releases
- Triggers on version tags

## Before First Publish

Make sure to:

1. ✅ Update `Cargo.toml` with correct metadata
2. ✅ Add comprehensive README.md
3. ✅ Add LICENSE files (MIT and Apache-2.0)
4. ✅ Test locally: `cargo publish --dry-run`
5. ✅ Verify documentation builds: `cargo doc --open`
6. ✅ Run all tests: `cargo test`
7. ✅ Check formatting: `cargo fmt -- --check`
8. ✅ Run clippy: `cargo clippy -- -D warnings`

## Versioning

This project follows [Semantic Versioning](https://semver.org/):

- **MAJOR** version for incompatible API changes
- **MINOR** version for new functionality (backwards compatible)
- **PATCH** version for bug fixes (backwards compatible)

## Troubleshooting

### Publish fails with "crate name already exists"

The crate name `oracledb-rs` might be taken. Choose a different name in `Cargo.toml`.

### GitHub Actions fail

Check the Actions tab in your repository for detailed error logs.

### Documentation doesn't deploy

Ensure GitHub Pages is enabled and set to "GitHub Actions" source.

## Additional Resources

- [Cargo Publishing Guide](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [crates.io Publishing Guide](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [docs.rs Documentation](https://docs.rs/about)
