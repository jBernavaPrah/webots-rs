use clap::{Parser, Subcommand};
use std::fs;
use std::path::{Path, PathBuf};

const WEBOTS_LINUX_PATH: &str = "/usr/local/webots";
const WEBOTS_MACOS_PATH: &str = "/Applications/Webots.app/Contents";
const WEBOTS_WINDOWS_PATH: &str = "C:\\Program Files\\Webots";
const ENV_WEBOTS_HOME: &str = "WEBOTS_HOME";

#[derive(Parser)]
#[command(name = "bindings-generator")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    version: Option<String>,
    #[arg(long)]
    webots_home: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    Generate {
        version: String,
        #[arg(long)]
        webots_home: Option<PathBuf>,
    },
    Scaffold {
        version: String,
        from: String,
    },
}

fn default_webots_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let default = if cfg!(target_os = "linux") {
        WEBOTS_LINUX_PATH
    } else if cfg!(target_os = "macos") {
        WEBOTS_MACOS_PATH
    } else if cfg!(target_os = "windows") {
        WEBOTS_WINDOWS_PATH
    } else {
        return Err("Unsupported host OS for Webots bindings generation.".into());
    };

    let path = PathBuf::from(default);
    if path.exists() {
        Ok(path)
    } else {
        Err(format!(
            "No Webots installation found at the default location: {}",
            path.display()
        )
        .into())
    }
}

fn get_webots_path(webots_home: Option<PathBuf>) -> Result<PathBuf, Box<dyn std::error::Error>> {
    match webots_home {
        Some(path) => {
            if path.exists() {
                Ok(path)
            } else {
                Err(format!("Provided Webots path does not exist: {}", path.display()).into())
            }
        }
        None => match std::env::var(ENV_WEBOTS_HOME) {
            Ok(path) => {
                let path = PathBuf::from(path);
                if path.exists() {
                    Ok(path)
                } else {
                    Err(format!("WEBOTS_HOME does not exist: {}", path.display()).into())
                }
            }
            Err(_) => default_webots_path(),
        },
    }
}

fn normalize_webots_root(path: PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let lib_path = path.join("lib/controller");
    if lib_path.exists() {
        return Ok(path);
    }

    if cfg!(target_os = "macos") {
        let contents = path.join("Contents");
        if contents.join("lib/controller").exists() {
            return Ok(contents);
        }
    }

    Err(format!(
        "Webots controller library directory not found under {}",
        path.display()
    )
    .into())
}

fn normalize_version(version: &str) -> String {
    if version.starts_with('v') {
        version.to_string()
    } else {
        format!("v{}", version)
    }
}

fn wrapper_header_path(workspace_root: &Path, version: &str) -> PathBuf {
    workspace_root
        .join("headers")
        .join(version.trim_start_matches('v'))
        .join("wrapper.h")
}

fn bindings_output_path(workspace_root: &Path, version: &str) -> PathBuf {
    workspace_root.join("src").join(version).join("bindings.rs")
}

fn version_root_path(workspace_root: &Path, version: &str) -> PathBuf {
    workspace_root.join("src").join(version)
}

fn copy_directory_recursively(
    source: &Path,
    destination: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(destination)?;

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

fn rewrite_version_paths(
    root: &Path,
    from: &str,
    to: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = entry.metadata()?;

        if metadata.is_dir() {
            rewrite_version_paths(&path, from, to)?;
            continue;
        }

        if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
            continue;
        }

        let content = fs::read_to_string(&path)?;
        let updated = content.replace(from, to);
        if updated != content {
            fs::write(&path, updated)?;
        }
    }

    Ok(())
}

fn ensure_feature_in_manifest(
    workspace_root: &Path,
    version: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let manifest_path = workspace_root.join("Cargo.toml");
    let manifest = fs::read_to_string(&manifest_path)?;
    let feature_line = format!("{version} = []");

    if manifest.contains(&feature_line) {
        return Ok(());
    }

    let updated = manifest.replace(
        "runtime_link = []",
        &format!("{feature_line}\nruntime_link = []"),
    );
    if updated == manifest {
        return Err("Could not update Cargo.toml features section.".into());
    }

    fs::write(manifest_path, updated)?;
    Ok(())
}

fn ensure_lib_exports(
    workspace_root: &Path,
    version: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let lib_path = workspace_root.join("src/lib.rs");
    let lib = fs::read_to_string(&lib_path)?;
    let module_line = format!("#[cfg(feature = \"{version}\")]\npub mod {version};");
    let export_line = format!("#[cfg(feature = \"{version}\")]\npub use {version}::*;");

    let mut updated = lib;

    if !updated.contains(&module_line) {
        updated.push('\n');
        updated.push_str(&module_line);
        updated.push('\n');
    }

    if !updated.contains(&export_line) {
        updated.push('\n');
        updated.push_str(&export_line);
        updated.push('\n');
    }

    fs::write(lib_path, updated)?;
    Ok(())
}

fn scaffold_version(
    workspace_root: &Path,
    version: &str,
    from: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let source_root = version_root_path(workspace_root, from);
    if !source_root.exists() {
        return Err(format!(
            "Source version directory does not exist: {}",
            source_root.display()
        )
        .into());
    }

    let destination_root = version_root_path(workspace_root, version);
    if destination_root.exists() {
        return Err(format!(
            "Destination version directory already exists: {}",
            destination_root.display()
        )
        .into());
    }

    let source_header = wrapper_header_path(workspace_root, from);
    if !source_header.exists() {
        return Err(format!(
            "Source wrapper header does not exist: {}",
            source_header.display()
        )
        .into());
    }

    let destination_header = wrapper_header_path(workspace_root, version);
    let destination_header_parent = destination_header
        .parent()
        .ok_or("Could not determine destination header directory")?;

    copy_directory_recursively(&source_root, &destination_root)?;
    rewrite_version_paths(&destination_root, from, version)?;

    fs::create_dir_all(destination_header_parent)?;
    fs::copy(source_header, &destination_header)?;

    ensure_feature_in_manifest(workspace_root, version)?;
    ensure_lib_exports(workspace_root, version)?;

    println!(
        "Scaffolded {} from {}. Review {} and then regenerate bindings.",
        version,
        from,
        destination_root.display()
    );

    Ok(())
}

fn generate_bindings(
    workspace_root: &Path,
    version: &str,
    webots_home: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let webots_root = normalize_webots_root(get_webots_path(webots_home)?)?;
    let include_path = webots_root.join("include/controller/c");

    if !include_path.exists() {
        return Err(format!(
            "Webots include directory not found at {}",
            include_path.display()
        )
        .into());
    }

    let wrapper_path = wrapper_header_path(workspace_root, version);
    let output_path = bindings_output_path(workspace_root, version);

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    if !wrapper_path.exists() {
        return Err(format!("Missing wrapper header at {}", wrapper_path.display()).into());
    }

    let include_path = include_path
        .to_str()
        .ok_or("Webots include path contains invalid Unicode")?;

    let bindings = bindgen::Builder::default()
        .header(
            wrapper_path
                .to_str()
                .ok_or("Wrapper path contains invalid Unicode")?,
        )
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_args(["-I", include_path])
        .allowlist_function("wb.*")
        .raw_line(
            "#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]",
        )
        .generate()
        .map_err(|_| "Failed to generate bindings")?;

    bindings.write_to_file(&output_path)?;

    println!(
        "Generated {} from Webots at {}",
        output_path.display(),
        webots_root.display()
    );

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
        .parent()
        .ok_or("Could not determine workspace root")?;

    match cli.command {
        Some(Commands::Generate {
            version,
            webots_home,
        }) => generate_bindings(workspace_root, &normalize_version(&version), webots_home),
        Some(Commands::Scaffold { version, from }) => scaffold_version(
            workspace_root,
            &normalize_version(&version),
            &normalize_version(&from),
        ),
        None => {
            let version = cli.version.ok_or("Missing version argument.")?;
            generate_bindings(
                workspace_root,
                &normalize_version(&version),
                cli.webots_home,
            )
        }
    }
}
