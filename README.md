# webots

Rust bindings and safe wrappers for the Webots controller API.

## Build model

This crate is designed so companion crates can compile on machines that do not have Webots
installed.

- Default builds use checked-in, versioned bindings such as `src/v2025a/bindings.rs`.
- Default builds also select a versioned wrapper header such as `headers/2025a/wrapper.h`.
- Default builds do not link `libController`.
- Real controller linking is opt-in through the `runtime_link` feature.
- Running a controller still requires a valid Webots installation.

This makes the default mode suitable for CI/CD jobs where the goal is to compile Rust code, not
to execute it inside Webots.

## Usage

Library-only or CI compilation:

```toml
[dependencies]
webots = "0.1"
```

Version-explicit API:

```rust
let simulator = webots::v2025a::Simulator::new()?;
let webots = webots::v2025a::Webots::new()?;
```

Executable controller that should link against a real Webots installation:

```toml
[dependencies]
webots = { version = "0.1", features = ["runtime_link"] }
```

Runtime linking looks for Webots in the standard host install location and also honors
`WEBOTS_HOME` if Webots already set it.

## Example

```rust,no_run
use webots::Webots;

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

## Runtime linking

To link the real Webots controller library during build:

```bash
cargo build --features runtime_link
```

If runtime linking is requested and Webots is missing from the standard install location, the build fails fast.
