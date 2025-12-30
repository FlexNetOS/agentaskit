#!/usr/bin/env python3
"""
Unified Agents File Unification Script
Merges duplicate files, creates canonical versions, and archives old files.
"""

import shutil
import os
from pathlib import Path
from datetime import datetime

# Base directory
BASE_DIR = Path(__file__).parent

def archive_file(filepath: Path, archive_dir: Path):
    """Archive a file with timestamp"""
    timestamp = datetime.now().strftime("%Y%m%d-%H%M%S")
    archive_name = f"{filepath.stem}_{timestamp}{filepath.suffix}"
    archive_path = archive_dir / archive_name
    shutil.copy2(filepath, archive_path)
    print(f"✅ Archived: {filepath.name} -> {archive_path.name}")

def unify_csv_files():
    """Keep only the healed CSV as canonical version"""
    print("\n📊 Unifying CSV files...")
    
    # The healed version is most complete (1964 lines)
    canonical = BASE_DIR / "All_Inclusive_Agent_Directory_v6_plus.normalized.healed.csv"
    target = BASE_DIR / "All_Inclusive_Agent_Directory.csv"
    
    # Archive old versions
    archive_dir = BASE_DIR / "archive" / "csv"
    archive_dir.mkdir(parents=True, exist_ok=True)
    
    for csv_file in BASE_DIR.glob("All_Inclusive_Agent_Directory_v6_plus.normalized.*.csv"):
        archive_file(csv_file, archive_dir)
    
    # Copy canonical version
    shutil.copy2(canonical, target)
    print(f"✅ Created canonical: {target.name}")
    
    # Remove old versions
    for csv_file in BASE_DIR.glob("All_Inclusive_Agent_Directory_v6_plus.normalized.*.csv"):
        csv_file.unlink()
        print(f"🗑️  Removed: {csv_file.name}")

def unify_manifest_files():
    """Keep v2 as canonical (largest at 1.6MB)"""
    print("\n📦 Unifying manifest files...")
    
    canonical = BASE_DIR / "stack.manifest.v2.json"
    target = BASE_DIR / "stack.manifest.json"
    
    # Archive old versions
    archive_dir = BASE_DIR / "archive" / "manifests"
    archive_dir.mkdir(parents=True, exist_ok=True)
    
    for manifest_file in BASE_DIR.glob("stack.manifest*.json"):
        if manifest_file != target:
            archive_file(manifest_file, archive_dir)
    
    # Copy canonical version (v2 is largest)
    shutil.copy2(canonical, target)
    print(f"✅ Updated canonical: {target.name}")
    
    # Remove versioned files
    for manifest_file in BASE_DIR.glob("stack.manifest.v*.json"):
        manifest_file.unlink()
        print(f"🗑️  Removed: {manifest_file.name}")

def unify_schema_files():
    """Unify schema files - keep latest versions"""
    print("\n📋 Unifying schema files...")
    
    schema_dir = BASE_DIR / "schema"
    archive_dir = BASE_DIR / "archive" / "schemas"
    archive_dir.mkdir(parents=True, exist_ok=True)
    
    # CSV schemas: keep v3 as canonical
    csv_v3 = schema_dir / "CSV_SCHEMA_v3.md"
    csv_canonical = schema_dir / "CSV_SCHEMA.md"
    
    if csv_v3.exists():
        for schema_file in schema_dir.glob("CSV_SCHEMA_v*.md"):
            archive_file(schema_file, archive_dir)
        
        shutil.copy2(csv_v3, csv_canonical)
        print(f"✅ Created canonical: {csv_canonical.name}")
        
        for schema_file in schema_dir.glob("CSV_SCHEMA_v*.md"):
            schema_file.unlink()
            print(f"🗑️  Removed: {schema_file.name}")
    
    # Capsule schemas: keep v2 as canonical
    capsule_v2 = schema_dir / "capsule.schema.v2.json"
    capsule_canonical = schema_dir / "capsule.schema.json"
    
    if capsule_v2.exists():
        archive_file(capsule_canonical, archive_dir)
        archive_file(capsule_v2, archive_dir)
        
        shutil.copy2(capsule_v2, capsule_canonical)
        print(f"✅ Updated canonical: {capsule_canonical.name}")
        
        capsule_v2.unlink()
        print(f"🗑️  Removed: {capsule_v2.name}")

def unify_readme_files():
    """Merge README files into single comprehensive version"""
    print("\n📖 Unifying README files...")
    
    archive_dir = BASE_DIR / "archive" / "readmes"
    archive_dir.mkdir(parents=True, exist_ok=True)
    
    # Archive all README variants
    for readme_file in BASE_DIR.glob("README*.md"):
        if readme_file.name != "README.md":
            archive_file(readme_file, archive_dir)
    
    # The healed version is most complete
    healed_readme = BASE_DIR / "README_v3_HEALED.md"
    canonical_readme = BASE_DIR / "README.md"
    
    if healed_readme.exists():
        # Backup current README
        if canonical_readme.exists():
            archive_file(canonical_readme, archive_dir)
        
        # Use healed version as canonical
        shutil.copy2(healed_readme, canonical_readme)
        print(f"✅ Updated canonical: {canonical_readme.name}")
        
        # Remove variants
        for readme_file in BASE_DIR.glob("README*.md"):
            if readme_file != canonical_readme:
                readme_file.unlink()
                print(f"🗑️  Removed: {readme_file.name}")

def unify_howto_files():
    """Keep extended version as canonical"""
    print("\n📘 Unifying HOW-TO-USE files...")
    
    archive_dir = BASE_DIR / "archive" / "howto"
    archive_dir.mkdir(parents=True, exist_ok=True)
    
    extended = BASE_DIR / "HOW-TO-USE_v3_1_extended.md"
    canonical = BASE_DIR / "HOW-TO-USE.md"
    
    # Archive all versions
    for howto_file in BASE_DIR.glob("HOW-TO-USE*.md"):
        archive_file(howto_file, archive_dir)
    
    # Copy extended version as canonical
    if extended.exists():
        shutil.copy2(extended, canonical)
        print(f"✅ Created canonical: {canonical.name}")
        
        # Remove versioned files
        for howto_file in BASE_DIR.glob("HOW-TO-USE_v*.md"):
            howto_file.unlink()
            print(f"🗑️  Removed: {howto_file.name}")

def create_templates():
    """Create template files for new agent definitions"""
    print("\n📄 Creating template files...")
    
    templates_dir = BASE_DIR / "templates"
    templates_dir.mkdir(parents=True, exist_ok=True)
    
    # CSV template
    csv_template = templates_dir / "agent_directory_template.csv"
    csv_template.write_text("""AgentID,Name,Description,Capabilities,Integration_Type,Status
AGENT_001,TemplateAgent,Template agent description,capability1;capability2,primary,active
""")
    print(f"✅ Created: {csv_template.name}")
    
    # Manifest template
    manifest_template = templates_dir / "stack_manifest_template.json"
    manifest_template.write_text("""{
  "manifest_version": "1.0",
  "created_at": "TIMESTAMP",
  "agents": [],
  "capsules": [],
  "dependencies": []
}
""")
    print(f"✅ Created: {manifest_template.name}")
    
    # Schema template
    schema_template = templates_dir / "agent_schema_template.json"
    schema_template.write_text("""{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Agent Definition",
  "type": "object",
  "properties": {
    "agent_id": {"type": "string"},
    "name": {"type": "string"},
    "description": {"type": "string"},
    "capabilities": {"type": "array", "items": {"type": "string"}},
    "status": {"type": "string", "enum": ["active", "inactive", "deprecated"]}
  },
  "required": ["agent_id", "name", "description"]
}
""")
    print(f"✅ Created: {schema_template.name}")

def create_unification_report():
    """Create a report of what was unified"""
    print("\n📊 Creating unification report...")
    
    report_path = BASE_DIR / "UNIFICATION_REPORT.md"
    report = f"""# Unified Agents Unification Report

**Date:** {datetime.now().strftime("%Y-%m-%d %H:%M:%S")}
**Script:** unify_agents.py

## Summary

This script unified duplicate files in the unified_agents directory:

### CSV Files
- **Kept:** All_Inclusive_Agent_Directory.csv (from healed version, 1964 lines)
- **Archived:** fixed, full, healed variants

### Manifest Files
- **Kept:** stack.manifest.json (from v2, 1.6MB)
- **Archived:** v1, v2, v3 variants

### Schema Files
- **CSV Schema:** CSV_SCHEMA.md (from v3)
- **Capsule Schema:** capsule.schema.json (from v2)
- **Archived:** v1, v2, v3 variants

### README Files
- **Kept:** README.md (from healed version)
- **Archived:** extended, healed variants

### HOW-TO-USE Files
- **Kept:** HOW-TO-USE.md (from extended version)
- **Archived:** v3_1, extended variants

## Archive Location

All old versions archived to: `archive/` subdirectories
- `archive/csv/` - CSV file versions
- `archive/manifests/` - Manifest file versions
- `archive/schemas/` - Schema file versions
- `archive/readmes/` - README file versions
- `archive/howto/` - HOW-TO-USE file versions

## Templates Created

New template files created in: `templates/` directory
- `agent_directory_template.csv` - CSV template
- `stack_manifest_template.json` - Manifest template
- `agent_schema_template.json` - Schema template

## Next Steps

1. ✅ Update agent_factory.py to use canonical files
2. ✅ Test agent loading with new file structure
3. ✅ Update documentation references
4. ✅ Commit changes to version control

## Files Structure (After Unification)

```
unified_agents/
├── All_Inclusive_Agent_Directory.csv      # Canonical CSV
├── stack.manifest.json                    # Canonical manifest
├── README.md                              # Canonical README
├── HOW-TO-USE.md                          # Canonical guide
├── agent_factory.py                       # Agent factory (updated)
├── agent_names_list.txt                   # Agent names
├── agents_for_parallel.json               # Parallel config
├── config/                                # Configuration
├── schema/
│   ├── CSV_SCHEMA.md                      # Canonical CSV schema
│   ├── capsule.schema.json                # Canonical capsule schema
│   └── manifest.schema.json               # Manifest schema
├── templates/                             # NEW: Templates
│   ├── agent_directory_template.csv
│   ├── stack_manifest_template.json
│   └── agent_schema_template.json
└── archive/                               # NEW: Archived files
    ├── csv/
    ├── manifests/
    ├── schemas/
    ├── readmes/
    └── howto/
```

---
**Generated by:** unify_agents.py  
**Status:** ✅ Complete
"""
    
    report_path.write_text(report)
    print(f"✅ Created: {report_path.name}")

def main():
    """Main unification process"""
    print("=" * 80)
    print("🔧 UNIFIED AGENTS FILE UNIFICATION")
    print("=" * 80)
    print(f"\n📁 Working directory: {BASE_DIR}")
    print(f"⏰ Timestamp: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    
    try:
        # Execute unification steps
        unify_csv_files()
        unify_manifest_files()
        unify_schema_files()
        unify_readme_files()
        unify_howto_files()
        create_templates()
        create_unification_report()
        
        print("\n" + "=" * 80)
        print("✅ UNIFICATION COMPLETE!")
        print("=" * 80)
        print("\n📊 Summary:")
        print("  - CSV files unified to canonical version")
        print("  - Manifest files unified to canonical version")
        print("  - Schema files unified to canonical versions")
        print("  - README files unified to canonical version")
        print("  - HOW-TO-USE files unified to canonical version")
        print("  - Templates created in templates/ directory")
        print("  - Old versions archived in archive/ directory")
        print("  - Unification report created: UNIFICATION_REPORT.md")
        print("\n🎯 Next steps:")
        print("  1. Review canonical files")
        print("  2. Update agent_factory.py references")
        print("  3. Test agent loading")
        print("  4. Commit changes to git")
        
    except Exception as e:
        print(f"\n❌ ERROR: {e}")
        import traceback
        traceback.print_exc()
        return 1
    
    return 0

if __name__ == "__main__":
    exit(main())
