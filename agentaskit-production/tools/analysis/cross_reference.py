#!/usr/bin/env python3
"""
Cross-Reference Analysis Tool

Analyzes production code against archive versions (V2-V7) to identify:
- Lineage relationships between files
- Duplicate code across versions
- Missing production components
- File evolution history
"""

import argparse
import hashlib
import json
import os
import sys
from collections import defaultdict
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Set, Tuple


class CrossReferenceAnalyzer:
    """Analyzes cross-references between production and archive code."""

    def __init__(self, production_dir: str, archive_dir: str, output_dir: str):
        self.production_dir = Path(production_dir)
        self.archive_dir = Path(archive_dir)
        self.output_dir = Path(output_dir)
        self.output_dir.mkdir(parents=True, exist_ok=True)

        # Results storage
        self.production_files: Dict[str, dict] = {}
        self.archive_files: Dict[str, Dict[str, dict]] = defaultdict(dict)
        self.duplicates: List[dict] = []
        self.lineage: Dict[str, List[str]] = defaultdict(list)
        self.missing: List[str] = []

    def compute_file_hash(self, filepath: Path) -> str:
        """Compute SHA-256 hash of file contents."""
        try:
            with open(filepath, 'rb') as f:
                return hashlib.sha256(f.read()).hexdigest()
        except Exception:
            return ""

    def get_file_info(self, filepath: Path, base_dir: Path) -> dict:
        """Get file metadata and hash."""
        relative = filepath.relative_to(base_dir)
        stat = filepath.stat()
        return {
            "path": str(relative),
            "name": filepath.name,
            "size": stat.st_size,
            "hash": self.compute_file_hash(filepath),
            "extension": filepath.suffix,
            "modified": datetime.fromtimestamp(stat.st_mtime).isoformat(),
        }

    def scan_directory(self, directory: Path, base_dir: Path) -> Dict[str, dict]:
        """Scan directory and collect file information."""
        files = {}
        if not directory.exists():
            return files

        for filepath in directory.rglob("*"):
            if filepath.is_file() and not self._should_skip(filepath):
                info = self.get_file_info(filepath, base_dir)
                files[info["path"]] = info

        return files

    def _should_skip(self, filepath: Path) -> bool:
        """Check if file should be skipped."""
        skip_patterns = [".git", "__pycache__", ".pyc", "node_modules", "target/debug", "target/release", ".DS_Store"]
        path_str = str(filepath)
        return any(pattern in path_str for pattern in skip_patterns)

    def analyze(self) -> dict:
        """Run full cross-reference analysis."""
        print("Scanning production directory...")
        self.production_files = self.scan_directory(self.production_dir, self.production_dir)
        print(f"  Found {len(self.production_files)} files")

        if self.archive_dir.exists():
            for version_dir in sorted(self.archive_dir.iterdir()):
                if version_dir.is_dir():
                    version = version_dir.name
                    print(f"Scanning archive {version}...")
                    self.archive_files[version] = self.scan_directory(version_dir, version_dir)
                    print(f"  Found {len(self.archive_files[version])} files")

        print("Analyzing duplicates...")
        self._find_duplicates()
        print("Tracing file lineage...")
        self._trace_lineage()
        print("Identifying missing components...")
        self._find_missing()

        return self._generate_report()

    def _find_duplicates(self):
        hash_to_files: Dict[str, List[Tuple[str, str]]] = defaultdict(list)
        for path, info in self.production_files.items():
            if info["hash"]:
                hash_to_files[info["hash"]].append(("production", path))
        for version, files in self.archive_files.items():
            for path, info in files.items():
                if info["hash"]:
                    hash_to_files[info["hash"]].append((version, path))
        for file_hash, locations in hash_to_files.items():
            if len(locations) > 1:
                self.duplicates.append({"hash": file_hash[:16], "locations": [{"source": loc[0], "path": loc[1]} for loc in locations]})

    def _trace_lineage(self):
        for prod_path, prod_info in self.production_files.items():
            basename = prod_info["name"]
            lineage = []
            for version in sorted(self.archive_files.keys()):
                for arch_path, arch_info in self.archive_files[version].items():
                    if arch_info["name"] == basename:
                        lineage.append({"version": version, "path": arch_path, "hash_match": prod_info["hash"] == arch_info["hash"]})
            if lineage:
                self.lineage[prod_path] = lineage

    def _find_missing(self):
        expected_dirs = ["core/src", "services", "tests", "docs", "deploy", "security", "dashboards", "alerts", "slo"]
        expected_files = ["Cargo.toml", "README.md", "CHANGELOG.md", ".todo"]
        for dir_name in expected_dirs:
            if not (self.production_dir / dir_name).exists():
                self.missing.append(f"directory: {dir_name}")
        for file_name in expected_files:
            if not (self.production_dir / file_name).exists():
                self.missing.append(f"file: {file_name}")

    def _generate_report(self) -> dict:
        report = {
            "timestamp": datetime.now().isoformat(),
            "summary": {
                "production_files": len(self.production_files),
                "archive_versions": len(self.archive_files),
                "archive_files": sum(len(f) for f in self.archive_files.values()),
                "duplicates_found": len(self.duplicates),
                "files_with_lineage": len(self.lineage),
                "missing_components": len(self.missing),
            },
            "duplicates": self.duplicates[:50],
            "lineage_sample": dict(list(self.lineage.items())[:20]),
            "missing": self.missing,
            "critical_missing": [m for m in self.missing if "core" in m or "security" in m],
        }
        with open(self.output_dir / "report.json", "w") as f:
            json.dump(report, f, indent=2)
        with open(self.output_dir / "report.md", "w") as f:
            f.write(self._format_markdown(report))
        print(f"Reports saved to {self.output_dir}")
        return report

    def _format_markdown(self, report: dict) -> str:
        lines = ["# Cross-Reference Analysis Report", "", f"**Generated:** {report['timestamp']}", "", "## Summary", ""]
        lines.extend([f"- Production files: {report['summary']['production_files']}", f"- Duplicates found: {report['summary']['duplicates_found']}", f"- Missing components: {report['summary']['missing_components']}", "", "## Missing Components", ""])
        lines.extend([f"- {item}" for item in report["missing"]] if report["missing"] else ["*No missing components*"])
        return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser(description="Cross-reference analysis tool")
    parser.add_argument("--production-dir", required=True)
    parser.add_argument("--archive-dir", required=True)
    parser.add_argument("--output-dir", required=True)
    args = parser.parse_args()
    analyzer = CrossReferenceAnalyzer(args.production_dir, args.archive_dir, args.output_dir)
    report = analyzer.analyze()
    print(f"\n=== Analysis Complete ===\nProduction files: {report['summary']['production_files']}\nMissing: {report['summary']['missing_components']}")
    sys.exit(1 if report["critical_missing"] else 0)


if __name__ == "__main__":
    main()
