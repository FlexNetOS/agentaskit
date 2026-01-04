#!/usr/bin/env python3
"""
Configuration Validator for AgentAsKit
Validates YAML/TOML configuration files against schemas and rules
"""

import json
import yaml
import toml
import sys
import argparse
from pathlib import Path
from typing import Dict, List, Any, Tuple
from dataclasses import dataclass


@dataclass
class ValidationError:
    """Represents a configuration validation error"""
    field: str
    error: str
    severity: str = "error"  # error, warning, info


class ConfigValidator:
    """Validates configuration files against schemas and rules"""

    # Schema definitions for known configs
    SCHEMAS = {
        "rate_limits.yaml": {
            "required_fields": ["rate_limits"],
            "field_types": {
                "rate_limits": dict,
                "rate_limits.default_rps": (int, float),
                "rate_limits.burst_size": (int, float),
            },
            "constraints": {
                "rate_limits.default_rps": lambda x: x > 0,
                "rate_limits.burst_size": lambda x: x > 0,
            }
        },
        "tracing.yaml": {
            "required_fields": ["version", "tracer", "exporters"],
            "field_types": {
                "version": str,
                "tracer": dict,
                "exporters": dict,
            }
        },
        "agentgateway-dev.yaml": {
            "required_fields": ["gateway", "server"],
            "field_types": {
                "gateway.port": (int, str),
                "server.host": str,
            }
        }
    }

    SENSITIVE_FIELDS = {
        "password", "secret", "token", "api_key", "private_key",
        "authorization", "credentials", "aws_key", "db_password"
    }

    def __init__(self):
        self.errors: List[ValidationError] = []
        self.warnings: List[ValidationError] = []

    def validate_file(self, filepath: str) -> bool:
        """Validate a configuration file"""
        path = Path(filepath)

        if not path.exists():
            self.errors.append(ValidationError(
                field=filepath,
                error=f"File not found: {filepath}"
            ))
            return False

        try:
            config = self._load_config(path)
        except Exception as e:
            self.errors.append(ValidationError(
                field=filepath,
                error=f"Failed to parse file: {str(e)}"
            ))
            return False

        # Validate against schema if available
        filename = path.name
        if filename in self.SCHEMAS:
            self._validate_schema(config, filename)

        # Generic validations
        self._validate_sensitive_fields(config)
        self._validate_structure(config)

        return len(self.errors) == 0

    def validate_directory(self, dirpath: str) -> bool:
        """Validate all config files in a directory"""
        path = Path(dirpath)

        if not path.exists():
            self.errors.append(ValidationError(
                field=dirpath,
                error=f"Directory not found: {dirpath}"
            ))
            return False

        valid = True
        for config_file in path.glob("*.yaml") + path.glob("*.yml") + path.glob("*.toml"):
            if not self.validate_file(str(config_file)):
                valid = False

        return valid

    def _load_config(self, path: Path) -> Dict[str, Any]:
        """Load configuration from YAML, YML, or TOML file"""
        if path.suffix in [".yaml", ".yml"]:
            with open(path) as f:
                return yaml.safe_load(f) or {}
        elif path.suffix == ".toml":
            return toml.load(path)
        else:
            raise ValueError(f"Unsupported file format: {path.suffix}")

    def _validate_schema(self, config: Dict, filename: str):
        """Validate config against known schema"""
        schema = self.SCHEMAS.get(filename)
        if not schema:
            return

        # Check required fields
        for field in schema.get("required_fields", []):
            if field not in config:
                self.errors.append(ValidationError(
                    field=field,
                    error=f"Required field missing: {field}"
                ))

        # Check field types
        for field_path, expected_type in schema.get("field_types", {}).items():
            value = self._get_nested_value(config, field_path)
            if value is not None and not isinstance(value, expected_type):
                self.errors.append(ValidationError(
                    field=field_path,
                    error=f"Type mismatch: expected {expected_type}, got {type(value)}"
                ))

        # Check constraints
        for field_path, constraint in schema.get("constraints", {}).items():
            value = self._get_nested_value(config, field_path)
            if value is not None and not constraint(value):
                self.errors.append(ValidationError(
                    field=field_path,
                    error=f"Constraint violation: {field_path}"
                ))

    def _validate_sensitive_fields(self, config: Dict, path: str = ""):
        """Check for unprotected sensitive fields"""
        for key, value in config.items():
            current_path = f"{path}.{key}" if path else key

            # Check if field name suggests sensitive data
            if any(sensitive in key.lower() for sensitive in self.SENSITIVE_FIELDS):
                if isinstance(value, str) and len(value) > 0:
                    self.warnings.append(ValidationError(
                        field=current_path,
                        error=f"Sensitive field exposed in config: {key}",
                        severity="warning"
                    ))

            # Recurse into nested dicts
            if isinstance(value, dict):
                self._validate_sensitive_fields(value, current_path)

    def _validate_structure(self, config: Dict):
        """Validate overall config structure"""
        # Empty config
        if not config:
            self.warnings.append(ValidationError(
                field="root",
                error="Configuration is empty",
                severity="warning"
            ))

    def _get_nested_value(self, config: Dict, path: str) -> Any:
        """Get nested value from config using dot notation"""
        parts = path.split(".")
        current = config
        for part in parts:
            if isinstance(current, dict):
                current = current.get(part)
            else:
                return None
        return current

    def report(self) -> str:
        """Generate validation report"""
        lines = []
        lines.append("=" * 60)
        lines.append("Configuration Validation Report")
        lines.append("=" * 60)

        if self.errors:
            lines.append(f"\n❌ ERRORS ({len(self.errors)}):")
            for error in self.errors:
                lines.append(f"  [{error.field}] {error.error}")

        if self.warnings:
            lines.append(f"\n⚠️  WARNINGS ({len(self.warnings)}):")
            for warning in self.warnings:
                lines.append(f"  [{warning.field}] {warning.error}")

        if not self.errors and not self.warnings:
            lines.append("\n✅ All validations passed!")

        lines.append("\n" + "=" * 60)
        return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser(
        description="Validate AgentAsKit configuration files"
    )
    parser.add_argument("path", help="Config file or directory to validate")
    parser.add_argument(
        "-d", "--directory",
        action="store_true",
        help="Validate all configs in directory"
    )
    parser.add_argument(
        "-j", "--json",
        action="store_true",
        help="Output results as JSON"
    )

    args = parser.parse_args()

    validator = ConfigValidator()

    if args.directory:
        validator.validate_directory(args.path)
    else:
        validator.validate_file(args.path)

    if args.json:
        result = {
            "valid": len(validator.errors) == 0,
            "errors": [
                {"field": e.field, "error": e.error}
                for e in validator.errors
            ],
            "warnings": [
                {"field": w.field, "warning": w.error}
                for w in validator.warnings
            ]
        }
        print(json.dumps(result, indent=2))
    else:
        print(validator.report())

    # Exit with error code if validation failed
    sys.exit(0 if len(validator.errors) == 0 else 1)


if __name__ == "__main__":
    main()
