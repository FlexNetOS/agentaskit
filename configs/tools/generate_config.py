#!/usr/bin/env python3
"""
Configuration Generator for AgentAsKit
Generates environment-specific configurations from templates with variable substitution
"""

import os
import sys
import json
import yaml
import argparse
import re
from pathlib import Path
from typing import Dict, Any, List
from string import Template


class ConfigGenerator:
    """Generates configuration files from templates"""

    # Environment-specific templates
    TEMPLATES = {
        "agentgateway": {
            "dev": {
                "gateway": {
                    "host": "127.0.0.1",
                    "port": 8080,
                    "debug": True,
                    "log_level": "debug"
                },
                "server": {
                    "workers": 2,
                    "request_timeout": 30
                },
                "mcp": {
                    "enabled": True,
                    "protocol": "stdio"
                }
            },
            "staging": {
                "gateway": {
                    "host": "0.0.0.0",
                    "port": 8080,
                    "debug": False,
                    "log_level": "info"
                },
                "server": {
                    "workers": 4,
                    "request_timeout": 60
                },
                "mcp": {
                    "enabled": True,
                    "protocol": "stdio"
                }
            },
            "production": {
                "gateway": {
                    "host": "0.0.0.0",
                    "port": 8080,
                    "debug": False,
                    "log_level": "warn"
                },
                "server": {
                    "workers": 8,
                    "request_timeout": 120
                },
                "mcp": {
                    "enabled": True,
                    "protocol": "http"
                }
            }
        },
        "resource_config": {
            "dev": {
                "resources": {
                    "cpu_limit": "500m",
                    "memory_limit": "512Mi",
                    "cpu_request": "100m",
                    "memory_request": "128Mi"
                }
            },
            "staging": {
                "resources": {
                    "cpu_limit": "2000m",
                    "memory_limit": "2Gi",
                    "cpu_request": "1000m",
                    "memory_request": "1Gi"
                }
            },
            "production": {
                "resources": {
                    "cpu_limit": "4000m",
                    "memory_limit": "4Gi",
                    "cpu_request": "2000m",
                    "memory_request": "2Gi"
                }
            }
        }
    }

    def __init__(self):
        self.variables: Dict[str, str] = {}
        self._load_env_vars()

    def _load_env_vars(self):
        """Load environment variables for substitution"""
        # Load from .env file if it exists
        env_file = Path(".env")
        if env_file.exists():
            with open(env_file) as f:
                for line in f:
                    if line.strip() and not line.startswith("#"):
                        key, value = line.strip().split("=", 1)
                        self.variables[key] = value

        # Load from environment
        for key, value in os.environ.items():
            if key.startswith("AGENTASKIT_"):
                self.variables[key] = value

    def generate_config(
        self,
        config_name: str,
        environment: str = "dev",
        output_format: str = "yaml"
    ) -> str:
        """Generate configuration for the given config and environment"""

        if config_name not in self.TEMPLATES:
            raise ValueError(f"Unknown configuration: {config_name}")

        if environment not in ["dev", "staging", "production"]:
            raise ValueError(f"Invalid environment: {environment}")

        template = self.TEMPLATES[config_name][environment]
        config = self._substitute_variables(template)

        if output_format == "yaml":
            return yaml.dump(config, default_flow_style=False, sort_keys=False)
        elif output_format == "json":
            return json.dumps(config, indent=2)
        elif output_format == "toml":
            return self._dict_to_toml(config)
        else:
            raise ValueError(f"Unsupported output format: {output_format}")

    def generate_all(self, environment: str = "dev", output_dir: str = None) -> List[str]:
        """Generate all configurations for an environment"""
        files = []

        if output_dir is None:
            output_dir = f"configs/{environment}"

        Path(output_dir).mkdir(parents=True, exist_ok=True)

        for config_name in self.TEMPLATES:
            content = self.generate_config(config_name, environment, "yaml")

            # Determine filename
            if config_name == "resource_config":
                filename = f"resource_config.yaml"
            else:
                filename = f"{config_name}-{environment}.yaml"

            filepath = Path(output_dir) / filename
            with open(filepath, "w") as f:
                f.write(content)

            files.append(str(filepath))

        return files

    def _substitute_variables(self, obj: Any) -> Any:
        """Recursively substitute variables in config"""
        if isinstance(obj, str):
            return self._substitute_string(obj)
        elif isinstance(obj, dict):
            return {k: self._substitute_variables(v) for k, v in obj.items()}
        elif isinstance(obj, list):
            return [self._substitute_variables(item) for item in obj]
        else:
            return obj

    def _substitute_string(self, value: str) -> str:
        """Substitute variables in a string"""
        pattern = r'\$\{([^}]+)\}'

        def replacer(match):
            var_name = match.group(1)
            # Check for default value syntax: ${VAR:default}
            if ":" in var_name:
                var_name, default = var_name.split(":", 1)
                return self.variables.get(var_name, default)
            return self.variables.get(var_name, match.group(0))

        return re.sub(pattern, replacer, value)

    def _dict_to_toml(self, d: Dict[str, Any], parent_key: str = "") -> str:
        """Convert dictionary to TOML format"""
        lines = []

        for key, value in d.items():
            full_key = f"{parent_key}.{key}" if parent_key else key

            if isinstance(value, dict):
                lines.append(f"\n[{full_key}]")
                for k, v in value.items():
                    lines.append(f"{k} = {self._value_to_toml(v)}")
            else:
                lines.append(f"{key} = {self._value_to_toml(value)}")

        return "\n".join(lines)

    def _value_to_toml(self, value: Any) -> str:
        """Convert a value to TOML representation"""
        if isinstance(value, bool):
            return "true" if value else "false"
        elif isinstance(value, str):
            return f'"{value}"'
        elif isinstance(value, (int, float)):
            return str(value)
        elif isinstance(value, list):
            items = [self._value_to_toml(v) for v in value]
            return "[" + ", ".join(items) + "]"
        else:
            return str(value)

    def validate_template(self, config_name: str) -> bool:
        """Validate that a template exists and is well-formed"""
        if config_name not in self.TEMPLATES:
            return False

        template = self.TEMPLATES[config_name]
        required_envs = {"dev", "staging", "production"}

        return all(env in template for env in required_envs)

    def list_templates(self) -> List[str]:
        """List all available templates"""
        return list(self.TEMPLATES.keys())


def main():
    parser = argparse.ArgumentParser(
        description="Generate AgentAsKit configuration files"
    )
    parser.add_argument(
        "config",
        nargs="?",
        help="Configuration to generate (or 'all' for all configs)"
    )
    parser.add_argument(
        "-e", "--environment",
        choices=["dev", "staging", "production"],
        default="dev",
        help="Target environment (default: dev)"
    )
    parser.add_argument(
        "-o", "--output",
        help="Output file or directory"
    )
    parser.add_argument(
        "-f", "--format",
        choices=["yaml", "json", "toml"],
        default="yaml",
        help="Output format (default: yaml)"
    )
    parser.add_argument(
        "-l", "--list",
        action="store_true",
        help="List available templates"
    )

    args = parser.parse_args()

    generator = ConfigGenerator()

    if args.list:
        print("Available configurations:")
        for name in generator.list_templates():
            print(f"  - {name}")
        return

    if not args.config:
        parser.print_help()
        sys.exit(1)

    try:
        if args.config == "all":
            files = generator.generate_all(args.environment, args.output)
            print(f"Generated {len(files)} configuration files:")
            for f in files:
                print(f"  ✓ {f}")
        else:
            content = generator.generate_config(
                args.config,
                args.environment,
                args.format
            )

            if args.output:
                with open(args.output, "w") as f:
                    f.write(content)
                print(f"✓ Configuration written to {args.output}")
            else:
                print(content)

    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
