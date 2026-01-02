# TODO File Schema Documentation

## Overview

This document defines the schema and validation rules for the `.todo` file in the AgentAsKit Production repository. The TODO file serves as the single source of truth for all project tasks, following a structured format that enables automated tracking, validation, and synchronization with GitHub Issues.

**Task Reference**: OPS-TODO-VALIDATE
**Owner**: @platform
**Last Updated**: 2026-01-02

## Purpose

The `.todo` file schema enforces:
- **Consistency**: Standardized format across all tasks
- **Traceability**: Every task has a unique reference code
- **Accountability**: Clear ownership and dependencies
- **Quality**: Well-defined acceptance criteria and evidence
- **Automation**: Machine-readable format for CI/CD integration

## Schema Definition

### Required Fields

Every task entry **MUST** include the following fields:

#### 1. Checkbox Status
```
- [ ]  # Open task
- [x]  # Completed task
```

#### 2. Priority
```
(A)  # Critical priority
(B)  # High priority
(C)  # Medium priority
```

**Format**: `(A)`, `(B)`, or `(C)`
**Validation**: Must appear immediately after the date (if present) or after the checkbox

#### 3. Owner
```
owner:@team-name
owner:@person-name
```

**Format**: `owner:@[a-zA-Z0-9_-]+`
**Validation**: Must start with `owner:@` followed by a valid identifier
**Examples**:
- `owner:@platform`
- `owner:@sec-oncall`
- `owner:@perf-oncall`
- `owner:@docs`

#### 4. Dependencies
```
deps:[REF1, REF2, REF3]  # Multiple dependencies
deps:[]                   # No dependencies
```

**Format**: `deps:[DEPENDENCY-LIST]` or `deps:[]`
**Validation**:
- Must be present (use `deps:[]` if no dependencies)
- Dependencies should be valid REF codes in UPPERCASE-KEBAB-CASE
- Multiple dependencies separated by commas and optional spaces
**Examples**:
- `deps:[PERF-001, OBS-001, PERF-BENCH]`
- `deps:[SUPPLY-SIGN]`
- `deps:[]`

#### 5. Acceptance Criteria
```
acc:"Specific, measurable criteria for task completion"
```

**Format**: `acc:"criteria text"`
**Validation**:
- Must be enclosed in double quotes
- Cannot be empty (`acc:""` is invalid)
- Should be specific and measurable
**Examples**:
- `acc:"CI verifies signatures/hashes for all releases"`
- `acc:"≥3 executed entries with artifacts, transcripts, hashes; decisions/risks captured"`
- `acc:"Fail PR when owner/deps/acc/evid/REF missing; lint report posted"`

#### 6. Evidence
```
evid:"path/to/evidence, path/to/artifacts"
```

**Format**: `evid:"evidence paths"`
**Validation**:
- Must be enclosed in double quotes
- Cannot be empty (`evid:""` is invalid)
- Can list multiple paths separated by commas
**Examples**:
- `evid:".github/workflows/verify.yml, TEST/verify/*.log"`
- `evid:"core/src/orchestration/sot.md, operational_hash/HASHES.txt"`
- `evid:"CODEOWNERS, CONTRIBUTING.md"`

#### 7. Reference Code (REF)
```
[REF: TASK-CODE]
```

**Format**: `[REF: [A-Z0-9_-]+]`
**Validation**:
- Must be unique across the entire TODO file
- Should be in UPPERCASE-KEBAB-CASE
- Descriptive and meaningful
**Examples**:
- `[REF: SUPPLY-VERIFY]`
- `[REF: OPS-TODO-VALIDATE]`
- `[REF: GOV-SOT-EXEC]`
- `[REF: PERF-001]`

### Recommended Fields

These fields are **RECOMMENDED** but not required. Warnings will be issued if missing:

#### 8. Tags
```
+Tag1 +Tag2 +Tag3
```

**Format**: `+[A-Za-z0-9_-]+`
**Purpose**: Categorize and group related tasks
**Examples**:
- `+SupplyChain +Verify`
- `+Governance +SoT +Audit`
- `+Automation +CI +QualityGate`

#### 9. Context
```
@context
```

**Format**: `@[a-z]+`
**Purpose**: Specify environment or execution context
**Examples**:
- `@prod` - Production environment
- `@staging` - Staging environment
- `@dev` - Development environment

### Optional Fields

#### 10. Date
```
YYYY-MM-DD
```

**Format**: ISO date format
**Example**: `2025-10-05`

#### 11. Metrics
```
metric_name:value
```

**Format**: Bold text with metric name and value
**Examples**:
- `tput_tasks:10000_per_sec`
- `response_lt_ms:50`
- `startup_lt_ms:100`

## Complete Task Format

### Full Example
```
- [ ] (A) 2025-10-05 Task description [REF: TASK-CODE] owner:@team deps:[DEP1, DEP2] acc:"Acceptance criteria" evid:"evidence/path" +Tag1 +Tag2 @context metric:value
```

### Minimal Valid Example
```
- [ ] (B) Task description [REF: SIMPLE-TASK] owner:@platform deps:[] acc:"Task must be complete" evid:"path/to/evidence"
```

### Real-World Example
```
- [ ] (B) 2025-10-05 Signature verification [96m+SupplyChain +Verify[0m [95m@prod[0m [REF: SUPPLY-VERIFY] owner:@platform deps:[SUPPLY-SIGN] acc:"CI verifies signatures/hashes for all releases" evid:".github/workflows/verify.yml, TEST/verify/*.log"
```

## Validation Rules

### Critical Errors (Will Fail CI)

1. **Missing Priority**: Task must have `(A)`, `(B)`, or `(C)`
2. **Missing Owner**: Task must have `owner:@name`
3. **Missing Dependencies**: Task must have `deps:[...]` or `deps:[]`
4. **Empty Acceptance Criteria**: `acc:""` is not allowed
5. **Empty Evidence**: `evid:""` is not allowed
6. **Missing REF Code**: Task must have `[REF: CODE]`
7. **Duplicate REF Codes**: Each REF must be unique

### Warnings (Will Not Fail CI)

1. **Missing Tags**: No `+Tag` found
2. **Missing Context**: No `@context` found
3. **Malformed Dependencies**: Dependencies not in expected UPPERCASE format

## Validation Workflow

The `.github/workflows/todo-validate.yml` workflow automatically validates the TODO file on:
- Pull requests that modify `.todo`
- Pushes to `main` branch that modify `.todo`
- Manual workflow dispatch

### Validation Process

1. **Schema Validation**: Check all required fields are present and correctly formatted
2. **Duplicate Detection**: Ensure all REF codes are unique
3. **Report Generation**: Create detailed validation report with errors and warnings
4. **PR Comment**: Post validation results as a comment on pull requests
5. **CI Gate**: Fail the build if validation errors are found

### Example Validation Report

```markdown
# TODO Validation Report

**Generated**: 2026-01-02 17:00:00 UTC
**File**: .todo
**Workflow**: TODO Validation
**Run ID**: 123456789

## Summary

| Metric | Count |
|--------|-------|
| Total Tasks | 50 |
| Valid Tasks | 48 |
| Invalid Tasks | 2 |
| Warnings | 5 |

## ❌ Errors

- Line 46: Missing owner (format: owner:@name)
- Line 55: Missing acceptance criteria (format: acc:"criteria")

## ⚠️  Warnings

- Line 12: No tags found (recommended: +Tag format)
- Line 25: No context found (recommended: @context format)

## ✅ Overall Status: FAILED

Found 2 invalid task(s). Please fix the errors above.
```

## Best Practices

### Writing Good Task Descriptions

1. **Be Specific**: "Implement signature verification workflow" (good) vs "Fix stuff" (bad)
2. **Action-Oriented**: Start with a verb (Implement, Create, Update, Fix, etc.)
3. **Concise**: Keep under 80 characters when possible
4. **Self-Contained**: Should make sense without reading other tasks

### Choosing REF Codes

1. **Descriptive Prefixes**: Use domain prefixes (SUPPLY-, OPS-, GOV-, PERF-, SEC-, etc.)
2. **Meaningful Names**: `SUPPLY-VERIFY` is better than `TASK-123`
3. **Consistent Numbering**: Use sequential numbers within a domain (PERF-001, PERF-002)
4. **Avoid Duplicates**: Always check existing REF codes before adding new ones

### Writing Acceptance Criteria

1. **Measurable**: Include specific metrics or deliverables
2. **Testable**: Should be possible to verify completion objectively
3. **Complete**: Cover all aspects needed for the task to be "done"
4. **Clear**: Anyone should understand what "done" means

**Good Examples**:
- `acc:"CI verifies signatures/hashes for all releases; blocks on failure"`
- `acc:"≥3 executed entries with artifacts, transcripts, hashes"`
- `acc:"Fail PR when owner/deps/acc/evid/REF missing; lint report posted"`

**Bad Examples**:
- `acc:"Make it work"` (not measurable)
- `acc:"Done"` (not specific)
- `acc:""` (empty, will fail validation)

### Specifying Evidence

1. **Specific Paths**: Use actual file paths, not vague descriptions
2. **Wildcards OK**: Can use `*` for patterns (e.g., `TEST/verify/*.log`)
3. **Multiple Paths**: Separate with commas
4. **Relative Paths**: Paths relative to repository root

**Examples**:
- `evid:".github/workflows/verify.yml, TEST/verify/*.log"`
- `evid:"operational_hash/HASHES.txt, TEST/*, docs/"`
- `evid:"sbom/*.json, .github/workflows/sbom.yml"`

## ANSI Color Codes

The TODO file supports ANSI color codes for visual enhancement in terminals:

- `[96m+Tag[0m` - Cyan tags
- `[95m@context[0m` - Magenta contexts
- `[91m(A)[0m` - Red critical priority
- `[93m(B)[0m` - Yellow high priority
- `[1mbold[0m` - Bold text for metrics

These are optional and ignored by validation.

## Integration with Other Systems

### GitHub Issues Sync (OPS-ISSUE-SYNC)

When implemented, the `OPS-ISSUE-SYNC` task will:
- Create/update GitHub Issues keyed by REF code
- Apply labels based on tags (`+Tag` → `Tag` label)
- Assign owners based on `owner:@name`
- Link dependencies
- Keep issues in sync with TODO file

### Pre-push Hooks (OPS-HOOKS)

The `OPS-HOOKS` task will implement pre-push validation to:
- Validate TODO schema before pushing
- Ensure SoT is updated when TODO changes
- Verify hash manifests are current

## Troubleshooting

### Common Validation Errors

**Error**: Missing priority (A), (B), or (C)
**Fix**: Add `(A)`, `(B)`, or `(C)` after the checkbox

**Error**: Missing or invalid owner
**Fix**: Add `owner:@teamname` to the task line

**Error**: Missing deps field
**Fix**: Add `deps:[]` if no dependencies, or `deps:[REF1, REF2]` if there are dependencies

**Error**: Missing acceptance criteria
**Fix**: Add `acc:"specific acceptance criteria"` with non-empty criteria

**Error**: Missing evidence field
**Fix**: Add `evid:"path/to/evidence"` with actual evidence paths

**Error**: Missing REF code
**Fix**: Add `[REF: UNIQUE-CODE]` with a unique reference code

**Error**: Duplicate REF code
**Fix**: Choose a different, unique REF code

### Getting Help

- Review this schema documentation: `lint/todo.schema.md`
- Check validation reports in PR comments
- Examine existing valid tasks in `.todo` for examples
- Review validation workflow: `.github/workflows/todo-validate.yml`

## References

- **Task**: OPS-TODO-VALIDATE
- **Owner**: @platform
- **Evidence**: `.github/workflows/todo-validate.yml`, `lint/todo.schema.md`
- **Dependencies**: None
- **Related Tasks**:
  - OPS-ISSUE-SYNC: Sync TODO with GitHub Issues
  - OPS-HOOKS: Pre-push validation hooks
  - GOV-SOT-EXEC: State of Truth maintenance

## Changelog

- **2026-01-02**: Initial schema documentation created
- **2026-01-02**: Validation workflow implemented
- **2026-01-02**: Duplicate REF detection added
