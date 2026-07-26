use std::fs;
use std::path::{Path, PathBuf};
use std::{env, str};

const WEBOTS_LINUX_PATH: &str = "/usr/local/webots";
const WEBOTS_MACOS_PATH: &str = "/Applications/Webots.app/Contents";
const WEBOTS_WINDOWS_PATH: &str = "C:\\Program Files\\Webots";
const ENV_WEBOTS_HOME: &str = "WEBOTS_HOME";
const ENV_DOCS_RS: &str = "DOCS_RS";
const ENV_TARGET_OS: &str = "CARGO_CFG_TARGET_OS";

fn feature_env_to_version(feature: &str) -> Option<String> {
    let version = feature.strip_prefix("CARGO_FEATURE_V")?;
    Some(format!(
        "v{}",
        version.to_ascii_lowercase().replace('_', "")
    ))
}

fn selected_bindings_version() -> Result<String, Box<dyn std::error::Error>> {
    let enabled: Vec<_> = env::vars()
        .filter_map(|(name, _)| feature_env_to_version(&name))
        .collect();

    match enabled.as_slice() {
        [version] => Ok(version.clone()),
        [] => Err("No Webots bindings version feature enabled.".into()),
        _ => Err("Multiple Webots bindings version features enabled at once.".into()),
    }
}

fn wrapper_header_path(version: &str) -> PathBuf {
    Path::new("headers")
        .join(version.trim_start_matches('v'))
        .join("wrapper.h")
}

fn version_module_name(version: &str) -> String {
    version.to_string()
}

fn bindings_source_path(version: &str) -> PathBuf {
    Path::new("src")
        .join(version_module_name(version))
        .join("bindings.rs")
}

fn copy_directory_recursively(
    source: &Path,
    destination: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    if !destination.exists() {
        fs::create_dir_all(destination)?;
    }

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let entry_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        let metadata = entry.metadata()?;

        if metadata.is_dir() {
            copy_directory_recursively(&entry_path, &destination_path)?;
        } else {
            fs::copy(&entry_path, &destination_path)?;
        }
    }

    Ok(())
}

/// Default install path checked for a given target OS, if this build script knows one.
///
/// `target_os` must be the compilation target's OS (`CARGO_CFG_TARGET_OS`), not the host
/// running the build script -- a cross-compile has a different target than host, and staging
/// the host's runtime layout under a foreign target directory produces a binary that cannot
/// link or run.
fn default_webots_path_for_os(target_os: &str) -> Option<&'static str> {
    match target_os {
        "linux" => Some(WEBOTS_LINUX_PATH),
        "macos" => Some(WEBOTS_MACOS_PATH),
        "windows" => Some(WEBOTS_WINDOWS_PATH),
        _ => None,
    }
}

/// Where a resolved Webots path came from, so failure messages can tell the user which knob
/// to turn: an explicit `WEBOTS_HOME` is user-editable and wrong; the OS default is not
/// user-editable, so the fix there is either installing Webots or setting `WEBOTS_HOME`.
enum WebotsPathSource {
    EnvVar,
    DefaultForOs,
}

/// Resolves the Webots installation directory, or fails with a prescriptive error.
///
/// Detection order: `WEBOTS_HOME` first, then the OS default install path for `target_os`.
/// A `WEBOTS_HOME` that is set but is not a directory is a distinct error from "not found" --
/// a user who set it wrong needs to be told, not silently redirected to the default path.
fn resolve_webots_path(
    target_os: &str,
) -> Result<(String, WebotsPathSource), Box<dyn std::error::Error>> {
    if let Ok(configured) = env::var(ENV_WEBOTS_HOME) {
        return if Path::new(&configured).is_dir() {
            Ok((configured, WebotsPathSource::EnvVar))
        } else {
            Err(format!(
                "{ENV_WEBOTS_HOME} is set to '{configured}', but that is not a directory. \
                 Point {ENV_WEBOTS_HOME} at a real Webots installation, or unset it to use \
                 the default install location for this OS."
            )
            .into())
        };
    }

    let Some(default_path) = default_webots_path_for_os(target_os) else {
        return Err(format!(
            "Webots detection has no default install path for target OS '{target_os}'. \
             Set {ENV_WEBOTS_HOME} to your Webots installation directory."
        )
        .into());
    };

    if Path::new(default_path).is_dir() {
        Ok((default_path.to_string(), WebotsPathSource::DefaultForOs))
    } else {
        Err(format!(
            "Webots was not found at the default install location for this OS \
             ({default_path}). Install Webots R2025a, or set {ENV_WEBOTS_HOME} to your \
             Webots installation directory."
        )
        .into())
    }
}

/// Controller library filenames per OS, in the order this build script will accept them.
///
/// Confirmed from the Webots R2025a controller build (`src/controller/c/Makefile` and
/// `Controller.def` in the cyberbotics/webots repository): Linux produces `libController.so`,
/// macOS produces `libController.dylib`. The Windows entry is intentionally permissive --
/// `Controller.dll`, `Controller.lib` (MSVC import library), and `libController.a` (MinGW
/// import library) are all plausible layouts and this script has not been exercised against a
/// real Windows Webots installation, so it accepts any of them rather than hard-failing a
/// platform it cannot verify.
const OS_LIBRARY_NAMES: &[(&str, &[&str])] = &[
    ("linux", &["libController.so"]),
    ("macos", &["libController.dylib"]),
    (
        "windows",
        &["Controller.lib", "libController.a", "Controller.dll"],
    ),
];

/// Controller library filenames expected for `target_os`, or an empty slice if this build
/// script does not know any for that OS.
fn controller_library_names(target_os: &str) -> &'static [&'static str] {
    OS_LIBRARY_NAMES
        .iter()
        .find(|(os, _)| *os == target_os)
        .map(|(_, names)| *names)
        .unwrap_or(&[])
}

/// If `dir` contains a controller library for some OS other than `target_os`, names that OS.
///
/// This turns "not found" into "found, but for the wrong OS" when a directory looks like a
/// genuine Webots installation just not one built for the compilation target -- the case a
/// cross-compile from a fully-installed host hits directly, and the case most likely to
/// confuse a user who is looking straight at a real installation.
fn detect_foreign_os_library(dir: &Path, target_os: &str) -> Option<&'static str> {
    OS_LIBRARY_NAMES
        .iter()
        .filter(|(os, _)| *os != target_os)
        .find(|(_, names)| names.iter().any(|name| dir.join(name).is_file()))
        .map(|(os, _)| *os)
}

/// Builds the single prescriptive error for "resolved a Webots directory, but no controller
/// library for `target_os` under it" -- listing every library file actually looked for and
/// naming the fix that matches where the directory came from.
///
/// This deliberately validates the library filename only, not the CPU architecture it was
/// built for: telling an `x86_64` `libController.so` apart from an `aarch64` one would mean
/// parsing ELF/Mach-O headers, and an architecture mismatch already fails loudly at link time
/// rather than silently, so there is nothing to hide there.
fn missing_controller_library_error(
    webots_path: &str,
    source: &WebotsPathSource,
    target_os: &str,
    candidate_dirs: &[PathBuf],
) -> Box<dyn std::error::Error> {
    for dir in candidate_dirs {
        if let Some(found_os) = detect_foreign_os_library(dir, target_os) {
            let dir_display = dir.display();
            return format!(
                "{dir_display} looks like a Webots installation for '{found_os}', not \
                 '{target_os}'. A build targeting '{target_os}' needs a Webots installation \
                 built for that OS. Point {ENV_WEBOTS_HOME} at a Webots installation for \
                 '{target_os}'."
            )
            .into();
        }
    }

    // Cargo prints a failing build script's returned error with `Debug` formatting, which
    // renders embedded newlines as literal "\n" rather than line breaks. Keep every message
    // here on one logical line so what the user sees is prose, not an escaped list.
    let library_names = controller_library_names(target_os);
    let checked = candidate_dirs
        .iter()
        .flat_map(|dir| library_names.iter().map(move |name| dir.join(name)))
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>()
        .join(", ");

    match source {
        WebotsPathSource::EnvVar => format!(
            "{ENV_WEBOTS_HOME} is set to '{webots_path}', but no Webots controller library for \
             target OS '{target_os}' was found under it. Checked: {checked}. Point \
             {ENV_WEBOTS_HOME} at a Webots installation for '{target_os}', or unset it to use \
             the default install location for this OS."
        ),
        WebotsPathSource::DefaultForOs => format!(
            "No Webots controller library for target OS '{target_os}' was found under the \
             default install location for this OS ('{webots_path}'). Checked: {checked}. \
             Install Webots R2025a there, or set {ENV_WEBOTS_HOME} to a Webots installation for \
             '{target_os}'."
        ),
    }
    .into()
}

fn setup_webots_linking(
    webots_path: &str,
    source: &WebotsPathSource,
    target_os: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let base_path = PathBuf::from(webots_path);
    let mut candidate_dirs = vec![base_path.join("lib/controller")];

    if target_os == "macos" {
        candidate_dirs.push(base_path.join("Contents").join("lib/controller"));
    }

    let library_names = controller_library_names(target_os);
    if library_names.is_empty() {
        return Err(format!(
            "Webots controller library validation is not implemented for target OS \
             '{target_os}'. Set {ENV_WEBOTS_HOME} to a Webots installation for that OS and \
             verify manually that linking succeeds."
        )
        .into());
    }

    let lib_path = candidate_dirs
        .iter()
        .find(|dir| library_names.iter().any(|name| dir.join(name).is_file()))
        .cloned();

    let Some(lib_path) = lib_path else {
        return Err(missing_controller_library_error(
            webots_path,
            source,
            target_os,
            &candidate_dirs,
        ));
    };

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    fs::create_dir_all(&out_dir)?;

    let staging_root = out_dir.join("webots-runtime");
    if staging_root.exists() {
        fs::remove_dir_all(&staging_root)?;
    }

    let staged_controller_dir = if target_os == "macos" {
        let target = staging_root.join("Contents").join("lib/controller");
        copy_directory_recursively(&lib_path, &target)?;
        target
    } else {
        let target = staging_root.join("lib/controller");
        copy_directory_recursively(&lib_path, &target)?;
        target
    };

    println!(
        "cargo:rustc-link-search={}",
        staged_controller_dir.display()
    );
    println!("cargo:rustc-link-lib=Controller");

    let target_profile_dir = out_dir
        .parent()
        .and_then(|path| path.parent())
        .and_then(|path| path.parent())
        .ok_or("Unable to determine Cargo target directory")?
        .to_path_buf();

    let relative_controller_path = staged_controller_dir
        .strip_prefix(&target_profile_dir)
        .ok()
        .map(|path| path.to_path_buf());

    if target_os == "linux" {
        if let Some(relative_path) = &relative_controller_path {
            println!(
                "cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/{}",
                relative_path.to_string_lossy()
            );
        }

        println!(
            "cargo:rustc-link-arg=-Wl,-rpath,{}",
            staged_controller_dir.display()
        );
    } else if target_os == "macos" {
        let staged_bundle_root = staging_root.join("Contents");

        if let Ok(relative_path) = staged_bundle_root.strip_prefix(&target_profile_dir) {
            println!(
                "cargo:rustc-link-arg=-Wl,-rpath,@loader_path/{}",
                relative_path.to_string_lossy()
            );
        }

        println!(
            "cargo:rustc-link-arg=-Wl,-rpath,{}",
            staged_bundle_root.display()
        );
    } else if target_os == "windows" {
        let existing_path = env::var("PATH").unwrap_or_default();
        let staged_path = staged_controller_dir.to_string_lossy();
        let new_path = if existing_path.is_empty() {
            staged_path.to_string()
        } else {
            format!("{};{}", staged_path, existing_path)
        };

        println!("cargo:rustc-env=PATH={}", new_path);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-env-changed={}", ENV_WEBOTS_HOME);
    println!("cargo:rerun-if-env-changed={}", ENV_DOCS_RS);
    println!("cargo:rustc-check-cfg=cfg(webots_api_version)");

    let bindings_version = selected_bindings_version()?;
    let bindings_src_path = bindings_source_path(&bindings_version);
    let wrapper_header_path = wrapper_header_path(&bindings_version);

    println!("cargo:rerun-if-changed={}", bindings_src_path.display());
    println!("cargo:rerun-if-changed={}", wrapper_header_path.display());
    println!(
        "cargo:rustc-cfg=webots_api_version=\"{}\"",
        bindings_version
    );

    if !bindings_src_path.exists() {
        return Err(format!(
            "Checked-in bindings file not found: {}. Regenerate it with the bindings-generator crate.",
            bindings_src_path.display()
        )
        .into());
    }

    if !wrapper_header_path.exists() {
        return Err(format!(
            "Versioned wrapper header not found: {}",
            wrapper_header_path.display()
        )
        .into());
    }

    // rustdoc never links the native library, and docs.rs cannot install Webots,
    // so this is the only "no Webots" build that is correct rather than a hidden
    // degradation.
    if env::var(ENV_DOCS_RS).is_ok() {
        return Ok(());
    }

    let target_os = env::var(ENV_TARGET_OS)?;
    let (webots_path, source) = resolve_webots_path(&target_os)?;
    setup_webots_linking(&webots_path, &source, &target_os)
}
