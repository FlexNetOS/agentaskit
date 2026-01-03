//! Location Manager
//!
//! Manages target locations for workflow outputs including path resolution,
//! workspace management, and output organization.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Instant;

/// Types of output locations
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LocationType {
    /// Source code directory
    Source,
    /// Test directory
    Test,
    /// Documentation directory
    Documentation,
    /// Build output directory
    Build,
    /// Artifact storage
    Artifact,
    /// Temporary working directory
    Temp,
    /// Configuration directory
    Config,
    /// Custom location type
    Custom(String),
}

/// A managed location with metadata
#[derive(Debug, Clone)]
pub struct ManagedLocation {
    pub id: String,
    pub location_type: LocationType,
    pub path: PathBuf,
    pub writable: bool,
    pub created_at: Instant,
    pub metadata: HashMap<String, String>,
}

/// Location resolution result
#[derive(Debug, Clone)]
pub struct ResolvedLocation {
    pub path: PathBuf,
    pub exists: bool,
    pub writable: bool,
    pub location_type: LocationType,
}

/// Workspace configuration
#[derive(Debug, Clone)]
pub struct WorkspaceConfig {
    pub root: PathBuf,
    pub source_dir: String,
    pub test_dir: String,
    pub doc_dir: String,
    pub build_dir: String,
    pub artifact_dir: String,
    pub temp_dir: String,
    pub config_dir: String,
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            root: PathBuf::from("."),
            source_dir: "src".to_string(),
            test_dir: "tests".to_string(),
            doc_dir: "docs".to_string(),
            build_dir: "target".to_string(),
            artifact_dir: "artifacts".to_string(),
            temp_dir: ".tmp".to_string(),
            config_dir: "config".to_string(),
        }
    }
}

/// Manages output locations for workflow deliverables
pub struct LocationManager {
    workspace: WorkspaceConfig,
    locations: HashMap<String, ManagedLocation>,
    path_cache: HashMap<String, PathBuf>,
    base_path: PathBuf,
}

impl LocationManager {
    /// Create a new location manager with default configuration
    pub fn new(base_path: PathBuf) -> Self {
        let workspace = WorkspaceConfig {
            root: base_path.clone(),
            ..Default::default()
        };

        Self {
            workspace,
            locations: HashMap::new(),
            path_cache: HashMap::new(),
            base_path,
        }
    }

    /// Create a location manager with custom workspace configuration
    pub fn with_config(config: WorkspaceConfig) -> Self {
        let base_path = config.root.clone();
        Self {
            workspace: config,
            locations: HashMap::new(),
            path_cache: HashMap::new(),
            base_path,
        }
    }

    /// Resolve a deliverable name to its target location
    pub fn resolve(&self, deliverable: &str) -> ResolvedLocation {
        // Check cache first
        if let Some(cached) = self.path_cache.get(deliverable) {
            return ResolvedLocation {
                path: cached.clone(),
                exists: cached.exists(),
                writable: self.is_writable(cached),
                location_type: self.infer_type(cached),
            };
        }

        // Infer location from deliverable name
        let (location_type, relative_path) = self.parse_deliverable(deliverable);
        let full_path = self.get_base_for_type(&location_type).join(relative_path);

        ResolvedLocation {
            path: full_path.clone(),
            exists: full_path.exists(),
            writable: self.is_writable(&full_path),
            location_type,
        }
    }

    /// Parse a deliverable string to determine type and path
    fn parse_deliverable(&self, deliverable: &str) -> (LocationType, PathBuf) {
        // Support prefixed format: type:path
        if let Some(idx) = deliverable.find(':') {
            let type_str = &deliverable[..idx];
            let path = &deliverable[idx + 1..];

            let location_type = match type_str.to_lowercase().as_str() {
                "src" | "source" | "code" => LocationType::Source,
                "test" | "tests" => LocationType::Test,
                "doc" | "docs" | "documentation" => LocationType::Documentation,
                "build" | "target" => LocationType::Build,
                "artifact" | "artifacts" => LocationType::Artifact,
                "tmp" | "temp" => LocationType::Temp,
                "config" | "cfg" => LocationType::Config,
                other => LocationType::Custom(other.to_string()),
            };

            return (location_type, PathBuf::from(path));
        }

        // Infer from file extension or path patterns
        let path = Path::new(deliverable);
        let location_type = self.infer_type_from_path(path);

        (location_type, PathBuf::from(deliverable))
    }

    /// Infer location type from path patterns
    fn infer_type_from_path(&self, path: &Path) -> LocationType {
        let path_str = path.to_string_lossy().to_lowercase();

        if path_str.contains("test") || path_str.ends_with("_test.rs") || path_str.ends_with(".test.") {
            return LocationType::Test;
        }

        if path_str.contains("doc") || path_str.ends_with(".md") || path_str.ends_with(".txt") {
            return LocationType::Documentation;
        }

        if path_str.contains("config") || path_str.ends_with(".toml") ||
           path_str.ends_with(".yaml") || path_str.ends_with(".json") {
            return LocationType::Config;
        }

        if path_str.contains("target") || path_str.contains("build") || path_str.contains("dist") {
            return LocationType::Build;
        }

        if path_str.contains("artifact") || path_str.ends_with(".tar.gz") || path_str.ends_with(".zip") {
            return LocationType::Artifact;
        }

        if path_str.contains("tmp") || path_str.contains("temp") {
            return LocationType::Temp;
        }

        // Default to source for code files
        if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy().to_lowercase();
            if ["rs", "py", "js", "ts", "go", "java", "cpp", "c", "h"].contains(&ext.as_str()) {
                return LocationType::Source;
            }
        }

        LocationType::Source
    }

    /// Infer location type from existing path
    fn infer_type(&self, path: &Path) -> LocationType {
        self.infer_type_from_path(path)
    }

    /// Get base directory for a location type
    fn get_base_for_type(&self, location_type: &LocationType) -> PathBuf {
        let relative = match location_type {
            LocationType::Source => &self.workspace.source_dir,
            LocationType::Test => &self.workspace.test_dir,
            LocationType::Documentation => &self.workspace.doc_dir,
            LocationType::Build => &self.workspace.build_dir,
            LocationType::Artifact => &self.workspace.artifact_dir,
            LocationType::Temp => &self.workspace.temp_dir,
            LocationType::Config => &self.workspace.config_dir,
            LocationType::Custom(name) => name,
        };

        self.workspace.root.join(relative)
    }

    /// Check if a path is writable
    fn is_writable(&self, path: &Path) -> bool {
        if path.exists() {
            std::fs::metadata(path)
                .map(|m| !m.permissions().readonly())
                .unwrap_or(false)
        } else {
            // Check if parent directory is writable
            path.parent()
                .map(|p| {
                    if p.exists() {
                        std::fs::metadata(p)
                            .map(|m| !m.permissions().readonly())
                            .unwrap_or(false)
                    } else {
                        true // Assume writable if parent doesn't exist yet
                    }
                })
                .unwrap_or(true)
        }
    }

    /// Register a managed location
    pub fn register(&mut self, id: &str, location_type: LocationType, path: PathBuf) -> &ManagedLocation {
        let location = ManagedLocation {
            id: id.to_string(),
            location_type,
            path: path.clone(),
            writable: self.is_writable(&path),
            created_at: Instant::now(),
            metadata: HashMap::new(),
        };

        self.locations.insert(id.to_string(), location);
        self.locations.get(id).unwrap()
    }

    /// Get a registered location by ID
    pub fn get(&self, id: &str) -> Option<&ManagedLocation> {
        self.locations.get(id)
    }

    /// List all registered locations
    pub fn list(&self) -> Vec<&ManagedLocation> {
        self.locations.values().collect()
    }

    /// Ensure a directory exists, creating it if necessary
    pub fn ensure_dir(&self, path: &Path) -> Result<(), String> {
        if path.exists() {
            if path.is_dir() {
                Ok(())
            } else {
                Err(format!("Path exists but is not a directory: {:?}", path))
            }
        } else {
            std::fs::create_dir_all(path)
                .map_err(|e| format!("Failed to create directory {:?}: {}", path, e))
        }
    }

    /// Ensure the parent directory of a file path exists
    pub fn ensure_parent(&self, path: &Path) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            self.ensure_dir(parent)
        } else {
            Ok(())
        }
    }

    /// Get the workspace root
    pub fn root(&self) -> &Path {
        &self.workspace.root
    }

    /// Get workspace configuration
    pub fn config(&self) -> &WorkspaceConfig {
        &self.workspace
    }

    /// Cache a resolved path for faster future lookups
    pub fn cache_path(&mut self, deliverable: &str, path: PathBuf) {
        self.path_cache.insert(deliverable.to_string(), path);
    }

    /// Clear the path cache
    pub fn clear_cache(&mut self) {
        self.path_cache.clear();
    }

    /// Get paths for all standard locations
    pub fn standard_paths(&self) -> HashMap<LocationType, PathBuf> {
        let mut paths = HashMap::new();

        paths.insert(LocationType::Source, self.get_base_for_type(&LocationType::Source));
        paths.insert(LocationType::Test, self.get_base_for_type(&LocationType::Test));
        paths.insert(LocationType::Documentation, self.get_base_for_type(&LocationType::Documentation));
        paths.insert(LocationType::Build, self.get_base_for_type(&LocationType::Build));
        paths.insert(LocationType::Artifact, self.get_base_for_type(&LocationType::Artifact));
        paths.insert(LocationType::Temp, self.get_base_for_type(&LocationType::Temp));
        paths.insert(LocationType::Config, self.get_base_for_type(&LocationType::Config));

        paths
    }

    /// Validate that all required directories exist
    pub fn validate_workspace(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        for (location_type, path) in self.standard_paths() {
            if !path.exists() {
                // Only warn, don't error - directories might be created later
                errors.push(format!("{:?} directory does not exist: {:?}", location_type, path));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Create all standard workspace directories
    pub fn initialize_workspace(&self) -> Result<(), String> {
        for (location_type, path) in self.standard_paths() {
            if !path.exists() {
                self.ensure_dir(&path).map_err(|e| {
                    format!("Failed to create {:?} directory: {}", location_type, e)
                })?;
            }
        }
        Ok(())
    }

    /// Get relative path from workspace root
    pub fn relative_path(&self, path: &Path) -> Option<PathBuf> {
        path.strip_prefix(&self.workspace.root)
            .ok()
            .map(|p| p.to_path_buf())
    }

    /// Convert relative path to absolute
    pub fn absolute_path(&self, relative: &Path) -> PathBuf {
        if relative.is_absolute() {
            relative.to_path_buf()
        } else {
            self.workspace.root.join(relative)
        }
    }
}

/// Legacy compatibility function
#[allow(dead_code)]
pub fn resolve(deliverable: &str) -> String {
    let manager = LocationManager::new(PathBuf::from("agentaskit-production"));
    manager.resolve(deliverable).path.to_string_lossy().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_source() {
        let manager = LocationManager::new(PathBuf::from("/project"));
        let resolved = manager.resolve("src:main.rs");

        assert_eq!(resolved.path, PathBuf::from("/project/src/main.rs"));
        assert_eq!(resolved.location_type, LocationType::Source);
    }

    #[test]
    fn test_resolve_test() {
        let manager = LocationManager::new(PathBuf::from("/project"));
        let resolved = manager.resolve("test:unit_tests.rs");

        assert_eq!(resolved.path, PathBuf::from("/project/tests/unit_tests.rs"));
        assert_eq!(resolved.location_type, LocationType::Test);
    }

    #[test]
    fn test_infer_type() {
        let manager = LocationManager::new(PathBuf::from("/project"));

        let resolved = manager.resolve("something_test.rs");
        assert_eq!(resolved.location_type, LocationType::Test);

        let resolved = manager.resolve("README.md");
        assert_eq!(resolved.location_type, LocationType::Documentation);

        let resolved = manager.resolve("config.toml");
        assert_eq!(resolved.location_type, LocationType::Config);
    }

    #[test]
    fn test_standard_paths() {
        let manager = LocationManager::new(PathBuf::from("/project"));
        let paths = manager.standard_paths();

        assert_eq!(paths.get(&LocationType::Source), Some(&PathBuf::from("/project/src")));
        assert_eq!(paths.get(&LocationType::Test), Some(&PathBuf::from("/project/tests")));
    }

    #[test]
    fn test_legacy_resolve() {
        let path = resolve("src:main.rs");
        assert!(path.contains("src") && path.contains("main.rs"));
    }
}
