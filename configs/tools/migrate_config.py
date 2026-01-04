#!/usr/bin/env python3
"""
Configuration Migration Tool for AgentAsKit
Handles version migration and backwards compatibility for configuration files
"""

import json
import yaml
import sys
import argparse
from pathlib import Path
from typing import Dict, Any, Callable, List
from dataclasses import dataclass
from copy import deepcopy


@dataclass
class MigrationStep:
    """Represents a single migration step"""
    from_version: str
    to_version: str
    name: str
    description: str
    handler: Callable


class ConfigMigrator:
    """Manages configuration migrations and version upgrades"""

    # Define migration path
    MIGRATIONS: Dict[str, List[MigrationStep]] = {}

    def __init__(self):
        self._register_migrations()

    def _register_migrations(self):
        """Register all available migrations"""
        # You can add more migrations here as versions evolve
        pass

    def migrate(self, config: Dict[str, Any], from_version: str, to_version: str) -> Dict[str, Any]:
        """Migrate configuration from one version to another"""

        if from_version == to_version:
            return deepcopy(config)

        # Find migration path
        migrations = self._find_migration_path(from_version, to_version)

        if not migrations:
            raise ValueError(f"No migration path found from {from_version} to {to_version}")

        current = deepcopy(config)
        for migration in migrations:
            print(f"Applying migration: {migration.name}")
            current = migration.handler(current)

        return current

    def migrate_file(self, filepath: str, to_version: str, output: str = None):
        """Migrate a configuration file"""
        path = Path(filepath)

        if not path.exists():
            raise FileNotFoundError(f"Config file not found: {filepath}")

        # Load config
        with open(path) as f:
            if path.suffix in [".yaml", ".yml"]:
                config = yaml.safe_load(f) or {}
            elif path.suffix == ".json":
                config = json.load(f)
            else:
                raise ValueError(f"Unsupported format: {path.suffix}")

        # Detect current version
        from_version = config.get("version", "1.0.0")

        # Perform migration
        migrated = self.migrate(config, from_version, to_version)
        migrated["version"] = to_version

        # Write output
        output_path = Path(output or f"{path.stem}-{to_version}{path.suffix}")

        with open(output_path, "w") as f:
            if output_path.suffix in [".yaml", ".yml"]:
                yaml.dump(migrated, f, default_flow_style=False, sort_keys=False)
            elif output_path.suffix == ".json":
                json.dump(migrated, f, indent=2)

        return output_path

    def validate_compatibility(self, config: Dict[str, Any], target_version: str) -> List[str]:
        """Check if a configuration is compatible with target version"""
        issues = []

        # Version check
        current_version = config.get("version")
        if not current_version:
            issues.append("Configuration missing 'version' field")
            return issues

        # For now, any version can migrate to any future version
        # Add specific checks here as needed

        return issues

    def _find_migration_path(self, from_version: str, to_version: str) -> List[MigrationStep]:
        """Find the migration path between two versions"""
        # Simplified: direct migrations are pre-defined
        # In production, you'd implement proper version graph traversal

        key = f"{from_version}->{to_version}"
        return self.MIGRATIONS.get(key, [])

    def generate_changelog(self, from_version: str, to_version: str) -> str:
        """Generate a changelog for migration"""
        migrations = self._find_migration_path(from_version, to_version)

        lines = [
            f"Configuration Migration Changelog: {from_version} → {to_version}",
            "=" * 60,
            ""
        ]

        if not migrations:
            lines.append("No changes required")
        else:
            for migration in migrations:
                lines.append(f"• {migration.name}")
                lines.append(f"  {migration.description}")
                lines.append("")

        return "\n".join(lines)


class ConfigDiffer:
    """Compares two configuration versions"""

    @staticmethod
    def diff(config1: Dict[str, Any], config2: Dict[str, Any]) -> Dict[str, Any]:
        """Calculate differences between two configurations"""
        diff = {
            "added": {},
            "removed": {},
            "changed": {}
        }

        # Find added and changed fields
        for key, value2 in config2.items():
            if key not in config1:
                diff["added"][key] = value2
            elif config1[key] != value2:
                diff["changed"][key] = {
                    "old": config1[key],
                    "new": value2
                }

        # Find removed fields
        for key, value1 in config1.items():
            if key not in config2:
                diff["removed"][key] = value1

        return diff

    @staticmethod
    def apply_diff(config: Dict[str, Any], diff: Dict[str, Any]) -> Dict[str, Any]:
        """Apply a diff to a configuration"""
        result = deepcopy(config)

        # Apply removals
        for key in diff.get("removed", {}):
            result.pop(key, None)

        # Apply additions
        result.update(diff.get("added", {}))

        # Apply changes
        for key, change in diff.get("changed", {}).items():
            result[key] = change["new"]

        return result


def main():
    parser = argparse.ArgumentParser(
        description="Migrate AgentAsKit configuration files between versions"
    )
    parser.add_argument(
        "command",
        choices=["migrate", "validate", "diff", "changelog"],
        help="Command to execute"
    )
    parser.add_argument(
        "file",
        nargs="?",
        help="Configuration file to process"
    )
    parser.add_argument(
        "-v", "--version",
        required=True,
        help="Target version for migration or validation"
    )
    parser.add_argument(
        "-o", "--output",
        help="Output file for migrated configuration"
    )
    parser.add_argument(
        "-c", "--compare",
        help="Second file to compare for diff"
    )

    args = parser.parse_args()

    migrator = ConfigMigrator()

    try:
        if args.command == "migrate":
            if not args.file:
                parser.error("'file' argument required for migrate command")

            output_path = migrator.migrate_file(args.file, args.version, args.output)
            print(f"✓ Configuration migrated to {output_path}")

        elif args.command == "validate":
            if not args.file:
                parser.error("'file' argument required for validate command")

            with open(args.file) as f:
                config = yaml.safe_load(f) or {}

            issues = migrator.validate_compatibility(config, args.version)

            if issues:
                print(f"⚠️  Compatibility issues found:")
                for issue in issues:
                    print(f"  - {issue}")
                sys.exit(1)
            else:
                print(f"✓ Configuration is compatible with version {args.version}")

        elif args.command == "diff":
            if not args.file or not args.compare:
                parser.error("Both 'file' and '--compare' required for diff command")

            with open(args.file) as f:
                config1 = yaml.safe_load(f) or {}
            with open(args.compare) as f:
                config2 = yaml.safe_load(f) or {}

            diff = ConfigDiffer.diff(config1, config2)

            print("Configuration Differences:")
            print("=" * 60)

            if diff["added"]:
                print("\n✨ Added fields:")
                for key, value in diff["added"].items():
                    print(f"  + {key}: {value}")

            if diff["removed"]:
                print("\n❌ Removed fields:")
                for key, value in diff["removed"].items():
                    print(f"  - {key}: {value}")

            if diff["changed"]:
                print("\n🔄 Changed fields:")
                for key, change in diff["changed"].items():
                    print(f"  ~ {key}: {change['old']} → {change['new']}")

            if not any([diff["added"], diff["removed"], diff["changed"]]):
                print("\n✓ No differences found")

        elif args.command == "changelog":
            current_version = "1.0.0"  # Default
            changelog = migrator.generate_changelog(current_version, args.version)
            print(changelog)

    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
