//! Documentation output handling

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Documentation tree structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocTree {
    /// Root path of documentation
    pub root: PathBuf,

    /// Documentation sections
    pub sections: Vec<DocSection>,

    /// Generated metadata
    #[serde(default)]
    pub metadata: DocMetadata,
}

/// Documentation section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocSection {
    /// Section title
    pub title: String,

    /// Section order/index
    pub order: u32,

    /// Path to section file
    pub path: PathBuf,

    /// Child documents
    #[serde(default)]
    pub children: Vec<DocSection>,

    /// Section type
    #[serde(default)]
    pub section_type: SectionType,
}

/// Section type
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SectionType {
    #[default]
    Overview,
    Architecture,
    Workflow,
    DeepDive,
    Module,
    Api,
    Custom,
}

/// Documentation metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DocMetadata {
    /// Project name
    #[serde(default)]
    pub project_name: Option<String>,

    /// Project version
    #[serde(default)]
    pub version: Option<String>,

    /// Generation timestamp
    #[serde(default)]
    pub generated_at: Option<String>,

    /// Generator version
    #[serde(default)]
    pub generator_version: Option<String>,

    /// Target language
    #[serde(default)]
    pub language: Option<String>,

    /// Total files analyzed
    #[serde(default)]
    pub files_analyzed: u32,

    /// Total documentation pages
    #[serde(default)]
    pub pages_generated: u32,

    /// Diagrams generated
    #[serde(default)]
    pub diagrams_generated: u32,
}

impl DocTree {
    /// Create a new documentation tree
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            sections: Vec::new(),
            metadata: DocMetadata::default(),
        }
    }

    /// Load documentation tree from output directory
    pub fn from_directory(dir: &PathBuf) -> crate::Result<Self> {
        let mut tree = DocTree::new(dir.clone());

        // Standard Litho output structure
        let standard_sections = [
            ("1、项目概述.md", "Project Overview", 1, SectionType::Overview),
            ("2、架构概览.md", "Architecture Overview", 2, SectionType::Architecture),
            ("3、工作流程.md", "Workflow Overview", 3, SectionType::Workflow),
        ];

        for (file, title, order, section_type) in standard_sections {
            let path = dir.join(file);
            if path.exists() {
                tree.sections.push(DocSection {
                    title: title.to_string(),
                    order,
                    path,
                    children: Vec::new(),
                    section_type,
                });
            }
        }

        // Check for deep dive directory
        let deep_dive_dir = dir.join("4、深入探索");
        if deep_dive_dir.exists() && deep_dive_dir.is_dir() {
            let mut deep_dive_section = DocSection {
                title: "Deep Dive".to_string(),
                order: 4,
                path: deep_dive_dir.clone(),
                children: Vec::new(),
                section_type: SectionType::DeepDive,
            };

            // Add child documents
            if let Ok(entries) = std::fs::read_dir(&deep_dive_dir) {
                let mut child_order = 1;
                for entry in entries.filter_map(|e| e.ok()) {
                    let path = entry.path();
                    if path.extension().map_or(false, |ext| ext == "md") {
                        let title = path.file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or("Unknown")
                            .to_string();

                        deep_dive_section.children.push(DocSection {
                            title,
                            order: child_order,
                            path,
                            children: Vec::new(),
                            section_type: SectionType::Module,
                        });
                        child_order += 1;
                    }
                }
            }

            if !deep_dive_section.children.is_empty() {
                tree.sections.push(deep_dive_section);
            }
        }

        Ok(tree)
    }

    /// Get all document paths
    pub fn all_paths(&self) -> Vec<PathBuf> {
        let mut paths = Vec::new();
        for section in &self.sections {
            self.collect_paths(section, &mut paths);
        }
        paths
    }

    fn collect_paths(&self, section: &DocSection, paths: &mut Vec<PathBuf>) {
        if section.path.is_file() {
            paths.push(section.path.clone());
        }
        for child in &section.children {
            self.collect_paths(child, paths);
        }
    }

    /// Count total documents
    pub fn document_count(&self) -> usize {
        self.all_paths().len()
    }

    /// Generate table of contents
    pub fn generate_toc(&self) -> String {
        let mut toc = String::from("# Table of Contents\n\n");

        for section in &self.sections {
            self.append_toc_entry(section, &mut toc, 0);
        }

        toc
    }

    fn append_toc_entry(&self, section: &DocSection, toc: &mut String, indent: usize) {
        let indent_str = "  ".repeat(indent);
        let link = section.path.file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        toc.push_str(&format!("{}- [{}]({})\n", indent_str, section.title, link));

        for child in &section.children {
            self.append_toc_entry(child, toc, indent + 1);
        }
    }
}

/// Document content representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// Document title
    pub title: String,

    /// Document content (markdown)
    pub content: String,

    /// Mermaid diagrams in the document
    #[serde(default)]
    pub diagrams: Vec<MermaidDiagram>,

    /// Code blocks in the document
    #[serde(default)]
    pub code_blocks: Vec<CodeBlock>,

    /// Cross-references to other documents
    #[serde(default)]
    pub references: Vec<String>,
}

/// Mermaid diagram
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MermaidDiagram {
    /// Diagram type (flowchart, sequence, class, etc.)
    pub diagram_type: String,

    /// Diagram content
    pub content: String,

    /// Caption
    #[serde(default)]
    pub caption: Option<String>,
}

/// Code block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeBlock {
    /// Programming language
    pub language: String,

    /// Code content
    pub content: String,

    /// File path reference
    #[serde(default)]
    pub file_path: Option<String>,

    /// Line range
    #[serde(default)]
    pub line_range: Option<(u32, u32)>,
}

impl Document {
    /// Parse a markdown file into a Document
    pub fn from_file(path: &PathBuf) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Self::from_markdown(&content)
    }

    /// Parse markdown content into a Document
    pub fn from_markdown(content: &str) -> crate::Result<Self> {
        let mut title = String::new();
        let mut diagrams = Vec::new();
        let mut code_blocks = Vec::new();

        // Extract title from first heading
        for line in content.lines() {
            if line.starts_with("# ") {
                title = line.trim_start_matches("# ").to_string();
                break;
            }
        }

        // Extract Mermaid diagrams
        let mermaid_re = regex::Regex::new(r"```mermaid\s*\n([\s\S]*?)```").unwrap();
        for cap in mermaid_re.captures_iter(content) {
            if let Some(diagram_content) = cap.get(1) {
                let diagram_text = diagram_content.as_str().trim();
                let diagram_type = diagram_text.lines()
                    .next()
                    .unwrap_or("unknown")
                    .split_whitespace()
                    .next()
                    .unwrap_or("unknown")
                    .to_string();

                diagrams.push(MermaidDiagram {
                    diagram_type,
                    content: diagram_text.to_string(),
                    caption: None,
                });
            }
        }

        // Extract code blocks
        let code_re = regex::Regex::new(r"```(\w+)\s*\n([\s\S]*?)```").unwrap();
        for cap in code_re.captures_iter(content) {
            if let (Some(lang), Some(code)) = (cap.get(1), cap.get(2)) {
                let language = lang.as_str().to_string();
                if language != "mermaid" {
                    code_blocks.push(CodeBlock {
                        language,
                        content: code.as_str().to_string(),
                        file_path: None,
                        line_range: None,
                    });
                }
            }
        }

        Ok(Self {
            title,
            content: content.to_string(),
            diagrams,
            code_blocks,
            references: Vec::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doc_tree_new() {
        let tree = DocTree::new(PathBuf::from("./docs"));
        assert!(tree.sections.is_empty());
    }

    #[test]
    fn test_document_from_markdown() {
        let content = r#"# Test Document

Some content here.

```mermaid
flowchart TD
    A --> B
```

```rust
fn main() {
    println!("Hello");
}
```
"#;

        let doc = Document::from_markdown(content).unwrap();
        assert_eq!(doc.title, "Test Document");
        assert_eq!(doc.diagrams.len(), 1);
        assert_eq!(doc.diagrams[0].diagram_type, "flowchart");
        assert_eq!(doc.code_blocks.len(), 1);
        assert_eq!(doc.code_blocks[0].language, "rust");
    }

    #[test]
    fn test_generate_toc() {
        let mut tree = DocTree::new(PathBuf::from("./docs"));
        tree.sections.push(DocSection {
            title: "Overview".to_string(),
            order: 1,
            path: PathBuf::from("overview.md"),
            children: Vec::new(),
            section_type: SectionType::Overview,
        });

        let toc = tree.generate_toc();
        assert!(toc.contains("Overview"));
        assert!(toc.contains("overview.md"));
    }
}
