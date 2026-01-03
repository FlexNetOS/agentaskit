//! Integration tests for Location Manager
//!
//! Tests validate:
//! - Path resolution
//! - Location type inference
//! - Workspace management

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Simulated location types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum LocationType {
    Source,
    Test,
    Documentation,
    Build,
    Artifact,
    Temp,
    Config,
    Custom(String),
}

/// Simulated location manager
struct TestLocationManager {
    root: PathBuf,
    source_dir: String,
    test_dir: String,
    doc_dir: String,
    build_dir: String,
    config_dir: String,
}

impl TestLocationManager {
    fn new(root: PathBuf) -> Self {
        Self {
            root,
            source_dir: "src".to_string(),
            test_dir: "tests".to_string(),
            doc_dir: "docs".to_string(),
            build_dir: "target".to_string(),
            config_dir: "config".to_string(),
        }
    }

    fn resolve(&self, deliverable: &str) -> (PathBuf, LocationType) {
        // Check for prefix format
        if let Some(idx) = deliverable.find(':') {
            let type_str = &deliverable[..idx];
            let path = &deliverable[idx + 1..];

            let location_type = match type_str.to_lowercase().as_str() {
                "src" | "source" | "code" => LocationType::Source,
                "test" | "tests" => LocationType::Test,
                "doc" | "docs" => LocationType::Documentation,
                "build" | "target" => LocationType::Build,
                "config" | "cfg" => LocationType::Config,
                other => LocationType::Custom(other.to_string()),
            };

            let base = self.get_base_for_type(&location_type);
            return (base.join(path), location_type);
        }

        // Infer from path/extension
        let location_type = self.infer_type(deliverable);
        let base = self.get_base_for_type(&location_type);
        (base.join(deliverable), location_type)
    }

    fn get_base_for_type(&self, ltype: &LocationType) -> PathBuf {
        let relative = match ltype {
            LocationType::Source => &self.source_dir,
            LocationType::Test => &self.test_dir,
            LocationType::Documentation => &self.doc_dir,
            LocationType::Build => &self.build_dir,
            LocationType::Config => &self.config_dir,
            _ => ".",
        };
        self.root.join(relative)
    }

    fn infer_type(&self, path: &str) -> LocationType {
        let lower = path.to_lowercase();

        if lower.contains("test") {
            return LocationType::Test;
        }
        if lower.ends_with(".md") || lower.ends_with(".txt") || lower.contains("doc") {
            return LocationType::Documentation;
        }
        if lower.ends_with(".toml") || lower.ends_with(".yaml") || lower.ends_with(".json") {
            return LocationType::Config;
        }
        if lower.contains("build") || lower.contains("target") {
            return LocationType::Build;
        }

        LocationType::Source
    }

    fn standard_paths(&self) -> HashMap<LocationType, PathBuf> {
        let mut paths = HashMap::new();
        paths.insert(LocationType::Source, self.root.join(&self.source_dir));
        paths.insert(LocationType::Test, self.root.join(&self.test_dir));
        paths.insert(LocationType::Documentation, self.root.join(&self.doc_dir));
        paths.insert(LocationType::Build, self.root.join(&self.build_dir));
        paths.insert(LocationType::Config, self.root.join(&self.config_dir));
        paths
    }
}

#[test]
fn test_resolve_with_prefix() {
    let manager = TestLocationManager::new(PathBuf::from("/project"));

    let (path, ltype) = manager.resolve("src:main.rs");
    assert_eq!(path, PathBuf::from("/project/src/main.rs"));
    assert_eq!(ltype, LocationType::Source);

    let (path, ltype) = manager.resolve("test:unit_tests.rs");
    assert_eq!(path, PathBuf::from("/project/tests/unit_tests.rs"));
    assert_eq!(ltype, LocationType::Test);

    let (path, ltype) = manager.resolve("doc:README.md");
    assert_eq!(path, PathBuf::from("/project/docs/README.md"));
    assert_eq!(ltype, LocationType::Documentation);
}

#[test]
fn test_resolve_with_inference() {
    let manager = TestLocationManager::new(PathBuf::from("/project"));

    let (path, ltype) = manager.resolve("main.rs");
    assert_eq!(ltype, LocationType::Source);

    let (path, ltype) = manager.resolve("README.md");
    assert_eq!(ltype, LocationType::Documentation);

    let (path, ltype) = manager.resolve("settings.toml");
    assert_eq!(ltype, LocationType::Config);

    let (path, ltype) = manager.resolve("unit_test.rs");
    assert_eq!(ltype, LocationType::Test);
}

#[test]
fn test_standard_paths() {
    let manager = TestLocationManager::new(PathBuf::from("/project"));
    let paths = manager.standard_paths();

    assert_eq!(paths.get(&LocationType::Source), Some(&PathBuf::from("/project/src")));
    assert_eq!(paths.get(&LocationType::Test), Some(&PathBuf::from("/project/tests")));
    assert_eq!(paths.get(&LocationType::Documentation), Some(&PathBuf::from("/project/docs")));
    assert_eq!(paths.get(&LocationType::Build), Some(&PathBuf::from("/project/target")));
    assert_eq!(paths.get(&LocationType::Config), Some(&PathBuf::from("/project/config")));
}

#[test]
fn test_custom_location_type() {
    let manager = TestLocationManager::new(PathBuf::from("/project"));

    let (path, ltype) = manager.resolve("custom:special/file.txt");

    if let LocationType::Custom(name) = &ltype {
        assert_eq!(name, "custom");
    } else {
        panic!("Expected Custom location type");
    }
}

#[test]
fn test_nested_paths() {
    let manager = TestLocationManager::new(PathBuf::from("/project"));

    let (path, _) = manager.resolve("src:modules/auth/handler.rs");
    assert_eq!(path, PathBuf::from("/project/src/modules/auth/handler.rs"));

    let (path, _) = manager.resolve("test:integration/api/auth_tests.rs");
    assert_eq!(path, PathBuf::from("/project/tests/integration/api/auth_tests.rs"));
}

#[test]
fn test_relative_vs_absolute() {
    let manager = TestLocationManager::new(PathBuf::from("/project"));

    // Relative paths get joined
    let (path, _) = manager.resolve("src:main.rs");
    assert!(path.is_absolute());
    assert!(path.starts_with("/project"));
}

#[test]
fn test_path_with_dots() {
    let manager = TestLocationManager::new(PathBuf::from("/project"));

    let (path, ltype) = manager.resolve("file.test.rs");
    assert_eq!(ltype, LocationType::Test);

    let (path, ltype) = manager.resolve("config.local.yaml");
    assert_eq!(ltype, LocationType::Config);
}

#[test]
fn test_empty_deliverable() {
    let manager = TestLocationManager::new(PathBuf::from("/project"));

    let (path, ltype) = manager.resolve("");
    assert_eq!(ltype, LocationType::Source);
}

#[test]
fn test_special_characters() {
    let manager = TestLocationManager::new(PathBuf::from("/project"));

    let (path, _) = manager.resolve("src:file-with-dashes.rs");
    assert!(path.to_string_lossy().contains("file-with-dashes"));

    let (path, _) = manager.resolve("src:file_with_underscores.rs");
    assert!(path.to_string_lossy().contains("file_with_underscores"));
}

#[test]
fn test_case_insensitive_type() {
    let manager = TestLocationManager::new(PathBuf::from("/project"));

    let (_, ltype1) = manager.resolve("SRC:main.rs");
    let (_, ltype2) = manager.resolve("src:main.rs");
    let (_, ltype3) = manager.resolve("Src:main.rs");

    assert_eq!(ltype1, ltype2);
    assert_eq!(ltype2, ltype3);
}

#[test]
fn test_multiple_colons() {
    let manager = TestLocationManager::new(PathBuf::from("/project"));

    // Should only split on first colon
    let (path, ltype) = manager.resolve("src:path:with:colons.rs");
    assert_eq!(ltype, LocationType::Source);
    assert!(path.to_string_lossy().contains("path:with:colons.rs"));
}

#[test]
fn test_workspace_validation() {
    let manager = TestLocationManager::new(PathBuf::from("/project"));
    let paths = manager.standard_paths();

    // All standard paths should be under root
    for (_ltype, path) in paths {
        assert!(path.starts_with("/project"));
    }
}
