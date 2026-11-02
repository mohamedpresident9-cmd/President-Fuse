//! Initial President Fuse product concepts layered on the Fyrox project template.

use serde::{Deserialize, Serialize};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

const PROJECT_FORMAT_VERSION: u32 = 1;

/// Asset categories planned for the authoring library.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetKind {
    #[default]
    Character,
    Clothing,
    Animal,
    Animation,
    Vehicle,
    Weapon,
    Building,
    Other,
}

/// The two explicit routes offered by the Animate action.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnimateAction {
    ImportFromComputer,
    OpenAnimationSites,
}

/// User-configured site shortcut. Opening a site never triggers an automatic download.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AnimationSite {
    pub label: String,
    pub url: String,
}

/// Lightweight reference to an asset in the user's project/library.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AssetDescriptor {
    pub name: String,
    pub kind: AssetKind,
    pub source_path: PathBuf,
    pub rigged: bool,
}

/// Versioned native project metadata stored in `.pfuse/project.toml`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProjectManifest {
    pub format_version: u32,
    pub name: String,
    #[serde(default)]
    pub assets: Vec<AssetDescriptor>,
    #[serde(default)]
    pub animation_sites: Vec<AnimationSite>,
}

impl ProjectManifest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            format_version: PROJECT_FORMAT_VERSION,
            name: name.into(),
            assets: Vec::new(),
            animation_sites: Vec::new(),
        }
    }

    /// Create the native project folder and write its initial manifest.
    pub fn create(project_dir: &Path, name: impl Into<String>) -> io::Result<Self> {
        let manifest = Self::new(name);
        manifest.save(project_dir)?;
        Ok(manifest)
    }

    /// Load a project manifest, rejecting formats newer than this application understands.
    pub fn load(project_dir: &Path) -> io::Result<Self> {
        let source = fs::read_to_string(manifest_path(project_dir))?;
        let manifest: Self = toml::from_str(&source).map_err(invalid_data)?;
        if manifest.format_version > PROJECT_FORMAT_VERSION {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                format!(
                    "Project format {} is newer than supported format {}",
                    manifest.format_version, PROJECT_FORMAT_VERSION
                ),
            ));
        }
        Ok(manifest)
    }

    /// Save project metadata without copying source assets into the project.
    pub fn save(&self, project_dir: &Path) -> io::Result<()> {
        let directory = project_dir.join(".pfuse");
        fs::create_dir_all(&directory)?;
        let serialized = toml::to_string_pretty(self).map_err(invalid_data)?;
        fs::write(directory.join("project.toml"), serialized)
    }

    /// Copy a selected source file into the project library and register it in the manifest.
    pub fn import_asset(
        &mut self,
        project_dir: &Path,
        source_path: &Path,
        kind: AssetKind,
    ) -> io::Result<AssetDescriptor> {
        if !source_path.is_file() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "The selected asset is not a regular file",
            ));
        }

        let library_dir = project_dir.join(".pfuse").join("assets");
        fs::create_dir_all(&library_dir)?;

        let file_name = source_path.file_name().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "The asset has no file name")
        })?;
        let mut destination = library_dir.join(file_name);
        let mut suffix = 2;
        while destination.exists() {
            let stem = source_path
                .file_stem()
                .and_then(|value| value.to_str())
                .unwrap_or("asset");
            let extension = source_path.extension().and_then(|value| value.to_str());
            let candidate = match extension {
                Some(extension) => format!("{stem}-{suffix}.{extension}"),
                None => format!("{stem}-{suffix}"),
            };
            destination = library_dir.join(candidate);
            suffix += 1;
        }

        fs::copy(source_path, &destination)?;
        let asset = AssetDescriptor {
            name: source_path
                .file_stem()
                .and_then(|value| value.to_str())
                .unwrap_or("Asset")
                .to_owned(),
            kind,
            source_path: destination,
            rigged: false,
        };
        self.add_asset(project_dir, asset.clone());
        Ok(asset)
    }

    /// Add/update an asset reference. Paths inside the project are stored relatively.
    pub fn add_asset(&mut self, project_dir: &Path, mut asset: AssetDescriptor) {
        let project_root = absolute_lexical(project_dir);
        let source = absolute_lexical(&asset.source_path);
        asset.source_path = source
            .strip_prefix(&project_root)
            .map(Path::to_path_buf)
            .unwrap_or(source);

        if let Some(existing) = self
            .assets
            .iter_mut()
            .find(|item| item.source_path == asset.source_path)
        {
            *existing = asset;
        } else {
            self.assets.push(asset);
        }
    }
}

fn manifest_path(project_dir: &Path) -> PathBuf {
    project_dir.join(".pfuse").join("project.toml")
}

fn invalid_data(error: impl std::fmt::Display) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, error.to_string())
}

fn absolute_lexical(path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(path)
    }
}
