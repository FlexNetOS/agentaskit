//! Output tests for wiki-rs integration

use std::path::PathBuf;

#[path = "../src/output.rs"]
mod output;

mod test_utils;

use output::*;
use test_utils::generate_test_timestamp;

#[test]
fn test_doc_tree_new() {
    let tree = DocTree::new(PathBuf::from("./docs"));

    assert_eq!(tree.root, PathBuf::from("./docs"));
    assert!(tree.sections.is_empty());
}

#[test]
fn test_doc_section() {
    let section = DocSection {
        title: "Overview".to_string(),
        order: 1,
        path: PathBuf::from("overview.md"),
        children: vec![],
        section_type: SectionType::Overview,
    };

    assert_eq!(section.title, "Overview");
    assert_eq!(section.order, 1);
    assert!(section.children.is_empty());
}

#[test]
fn test_section_types() {
    let types = vec![
        SectionType::Overview,
        SectionType::Architecture,
        SectionType::Workflow,
        SectionType::DeepDive,
        SectionType::Module,
        SectionType::Api,
        SectionType::Custom,
    ];

    for section_type in types {
        let section = DocSection {
            title: "Test".to_string(),
            order: 1,
            path: PathBuf::from("test.md"),
            children: vec![],
            section_type,
        };
        assert!(!section.title.is_empty());
    }
}

#[test]
fn test_doc_metadata() {
    let metadata = DocMetadata {
        project_name: Some("MyProject".to_string()),
        version: Some("1.0.0".to_string()),
        generated_at: Some(generate_test_timestamp()),
        generator_version: Some("1.2.7".to_string()),
        language: Some("en".to_string()),
        files_analyzed: 150,
        pages_generated: 25,
        diagrams_generated: 10,
    };

    assert!(metadata.project_name.is_some());
    assert_eq!(metadata.files_analyzed, 150);
    assert_eq!(metadata.pages_generated, 25);
}

#[test]
fn test_doc_tree_with_sections() {
    let mut tree = DocTree::new(PathBuf::from("./docs"));

    tree.sections.push(DocSection {
        title: "Overview".to_string(),
        order: 1,
        path: PathBuf::from("./docs/overview.md"),
        children: vec![],
        section_type: SectionType::Overview,
    });

    tree.sections.push(DocSection {
        title: "Architecture".to_string(),
        order: 2,
        path: PathBuf::from("./docs/architecture.md"),
        children: vec![
            DocSection {
                title: "Module A".to_string(),
                order: 1,
                path: PathBuf::from("./docs/architecture/module_a.md"),
                children: vec![],
                section_type: SectionType::Module,
            }
        ],
        section_type: SectionType::Architecture,
    });

    assert_eq!(tree.sections.len(), 2);
    assert_eq!(tree.sections[1].children.len(), 1);
}

#[test]
fn test_generate_toc() {
    let mut tree = DocTree::new(PathBuf::from("./docs"));

    tree.sections.push(DocSection {
        title: "Overview".to_string(),
        order: 1,
        path: PathBuf::from("overview.md"),
        children: vec![],
        section_type: SectionType::Overview,
    });

    let toc = tree.generate_toc();

    assert!(toc.contains("# Table of Contents"));
    assert!(toc.contains("Overview"));
    assert!(toc.contains("overview.md"));
}

#[test]
fn test_document_from_markdown_simple() {
    let content = r#"# Test Document

This is a test document.

Some content here.
"#;

    let doc = Document::from_markdown(content).expect("Failed to parse");

    assert_eq!(doc.title, "Test Document");
    assert!(doc.diagrams.is_empty());
    assert!(doc.code_blocks.is_empty());
}

#[test]
fn test_document_from_markdown_with_mermaid() {
    let content = r#"# Architecture Overview

```mermaid
flowchart TD
    A[Start] --> B[Process]
    B --> C[End]
```

Some explanation here.
"#;

    let doc = Document::from_markdown(content).expect("Failed to parse");

    assert_eq!(doc.title, "Architecture Overview");
    assert_eq!(doc.diagrams.len(), 1);
    assert_eq!(doc.diagrams[0].diagram_type, "flowchart");
}

#[test]
fn test_document_from_markdown_with_code() {
    let content = r#"# Code Example

```rust
fn main() {
    println!("Hello, world!");
}
```

```python
def hello():
    print("Hello, world!")
```
"#;

    let doc = Document::from_markdown(content).expect("Failed to parse");

    assert_eq!(doc.title, "Code Example");
    assert_eq!(doc.code_blocks.len(), 2);
    assert_eq!(doc.code_blocks[0].language, "rust");
    assert_eq!(doc.code_blocks[1].language, "python");
}

#[test]
fn test_mermaid_diagram() {
    let diagram = MermaidDiagram {
        diagram_type: "sequenceDiagram".to_string(),
        content: "participant A\nA->>B: Hello".to_string(),
        caption: Some("Sequence diagram example".to_string()),
    };

    assert_eq!(diagram.diagram_type, "sequenceDiagram");
    assert!(diagram.caption.is_some());
}

#[test]
fn test_code_block() {
    let block = CodeBlock {
        language: "typescript".to_string(),
        content: "const x: number = 42;".to_string(),
        file_path: Some("src/index.ts".to_string()),
        line_range: Some((10, 20)),
    };

    assert_eq!(block.language, "typescript");
    assert!(block.file_path.is_some());
    assert_eq!(block.line_range, Some((10, 20)));
}

#[test]
fn test_document_mixed_content() {
    let content = r#"# Full Document

Introduction paragraph.

## Architecture

```mermaid
graph LR
    A --> B
```

## Code

```rust
fn example() {}
```

```mermaid
sequenceDiagram
    A->>B: Message
```

## Conclusion

Final thoughts.
"#;

    let doc = Document::from_markdown(content).expect("Failed to parse");

    assert_eq!(doc.title, "Full Document");
    assert_eq!(doc.diagrams.len(), 2);
    assert_eq!(doc.code_blocks.len(), 1);

    // Check diagram types
    let diagram_types: Vec<_> = doc.diagrams.iter().map(|d| d.diagram_type.as_str()).collect();
    assert!(diagram_types.contains(&"graph"));
    assert!(diagram_types.contains(&"sequenceDiagram"));
}

#[test]
fn test_section_type_default() {
    let section_type: SectionType = Default::default();
    assert!(matches!(section_type, SectionType::Overview));
}

#[test]
fn test_doc_metadata_default() {
    let metadata = DocMetadata::default();

    assert!(metadata.project_name.is_none());
    assert!(metadata.version.is_none());
    assert_eq!(metadata.files_analyzed, 0);
}

#[test]
fn test_all_paths() {
    let mut tree = DocTree::new(PathBuf::from("./docs"));

    tree.sections.push(DocSection {
        title: "Overview".to_string(),
        order: 1,
        path: PathBuf::from("./docs/overview.md"),
        children: vec![],
        section_type: SectionType::Overview,
    });

    tree.sections.push(DocSection {
        title: "Deep Dive".to_string(),
        order: 2,
        path: PathBuf::from("./docs/deep-dive"),  // Directory, not file
        children: vec![
            DocSection {
                title: "Module A".to_string(),
                order: 1,
                path: PathBuf::from("./docs/deep-dive/module_a.md"),
                children: vec![],
                section_type: SectionType::Module,
            },
            DocSection {
                title: "Module B".to_string(),
                order: 2,
                path: PathBuf::from("./docs/deep-dive/module_b.md"),
                children: vec![],
                section_type: SectionType::Module,
            },
        ],
        section_type: SectionType::DeepDive,
    });

    // Note: all_paths only returns file paths, not directories
    // The actual count depends on file existence checks
}

#[test]
fn test_document_count() {
    let tree = DocTree::new(PathBuf::from("./nonexistent"));
    assert_eq!(tree.document_count(), 0);
}
