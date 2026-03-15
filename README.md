# webots

Rust bindings and safe wrappers for the Webots controller API.

[![CI](https://github.com/jBernavaPrah/webots-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/jBernavaPrah/webots-rs/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/webots.svg)](https://crates.io/crates/webots)
[![Docs.rs](https://img.shields.io/docsrs/webots)](https://docs.rs/webots)

`webots` provides checked-in Rust bindings for the Webots controller API plus a thin, safe wrapper
layer for common controller operations and device access.

## Build model

This crate is designed so companion crates can compile on machines that do not have Webots
installed.

- Default builds use checked-in, versioned bindings such as `src/v2025a/bindings.rs`.
- Default builds also select a versioned wrapper header such as `headers/2025a/wrapper.h`.
- Real controller linking happens automatically if Webots is installed in a standard location or if `WEBOTS_HOME` is set.
- If Webots is not installed, the build script will automatically fall back to generating stub bindings.
- Running a controller still requires a valid Webots installation.

This fallback behavior makes the crate suitable for CI/CD jobs where the goal is to compile Rust code, not
to execute it inside Webots.

## Highlights

- Checked-in generated bindings for reproducible builds.
- Safe wrapper entrypoints for robot lifecycle and common devices.
- Versioned API namespaces so multiple Webots releases can coexist over time.
- Automatic runtime linking for real controller binaries with fallback to stubs for CI/CD.

## Usage

Basic usage:

```toml
[dependencies]
webots = "0.1"
```

Version-explicit API:

```rust,no_run
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let simulator = webots_rs::v2025a::Simulator::new()?;
    let webots = webots_rs::v2025a::Webots::new()?;
    Ok(())
}
```

Runtime linking automatically looks for Webots in the standard host install location and also honors
`WEBOTS_HOME` if Webots already set it. If it cannot find Webots, it falls back to stub bindings, which is useful for CI/CD.

## Quick start

```rust,no_run
use webots_rs::Webots;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let webots = Webots::new()?;
    let time_step = webots.get_basic_time_step()? as i32;

    let left_motor = webots.motor("left wheel motor")?;
    let right_motor = webots.motor("right wheel motor")?;

    left_motor.set_velocity(3.0)?;
    right_motor.set_velocity(3.0)?;

    while webots.step(time_step)? {
        // controller loop
    }

    Ok(())
}
```

## Example

```rust,no_run
use webots_rs::Webots;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let webots = Webots::new()?;
    let time_step = webots.get_basic_time_step()? as i32;

    while webots.step(time_step)? {
        // simulation loop
    }

    Ok(())
}
```

## Supported bindings versions

One version feature must be selected at a time.

- `v2025a` (default)

The selected version is exposed in Rust as `webots::WEBOTS_API_VERSION`.
The current versioned namespace is `webots::v2025a`.

Each supported Webots release owns its own Rust module tree under `src/vXXXX/`.

## Release and CI

The repository ships with GitHub Actions for:

- CI on pushes and pull requests: `fmt`, `check`, `clippy`, `doc`, and `cargo package`.
- `release-plz`-driven releases: pushes to `main`/`master` update or create a release PR, and
  merging that PR publishes the crate to crates.io and creates a GitHub release.

The release workflow expects a `CARGO_REGISTRY_TOKEN` repository secret and uses the default
`GITHUB_TOKEN` for release PRs and GitHub releases.

## Maintainer workflow

This repository uses a small internal workspace member to scaffold and regenerate versioned API
trees. End users do not build this helper crate.

Scaffold a new Webots version from an existing Rust API tree:

```bash
cargo bindings-generator scaffold v2025b v2025a
```

That copies `src/v2025a/` to `src/v2025b/`, rewrites internal module paths, copies
`headers/2025a/wrapper.h` to `headers/2025b/wrapper.h`, and adds the `v2025b` feature/export
boilerplate.

Generate a bindings file:

```bash
cargo bindings-generator v2025a
```

That command reads `headers/2025a/wrapper.h` and writes `src/v2025a/bindings.rs`.

If Webots is installed somewhere non-standard, pass it explicitly:

```bash
cargo bindings-generator generate v2025a --webots-home /path/to/webots
```

The generator also honors `WEBOTS_HOME` if it is already present in the environment.

## Adding a new Webots version

No `build.rs` changes are needed for a new version.

1. Run `cargo bindings-generator scaffold v2025b v2025a`.
2. Edit `headers/2025b/wrapper.h` for the new Webots header surface.
3. Run `cargo bindings-generator v2025b`.
4. Review `src/v2025b/` and make any API changes required by that Webots release.
