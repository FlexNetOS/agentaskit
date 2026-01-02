# ADR: TODO Source Deduplication

**Status:** Implemented
**Date:** 2026-01-02
**REF:** OPS-DEDUP-TODO
**Owner:** @program

## Context

The AgentAskit production repository had multiple TODO tracking files:
- `/home/user/agentaskit/agentaskit-production/.todo` - Main comprehensive TODO list
- `/home/user/agentaskit/agentaskit-production/agentask.subject.todo` - Subject inbox for chat-request items

This dual-source approach created:
- Potential for task duplication and inconsistency
- Confusion about which file to reference for current tasks
- Difficulty in maintaining a single authoritative source of truth
- Risk of tasks being tracked in one file but not the other

## Decision

Establish `/home/user/agentaskit/agentaskit-production/.todo` as the **single source of truth** for all TODO items.

### Actions Taken

1. **Archived Legacy File**
   - Moved `agentask.subject.todo` to `archive/legacy_todos/agentask.subject.todo.20260102`
   - Added archival header with reference to current TODO location
   - Preserved historical content for reference

2. **Updated Documentation**
   - Enhanced `CONTRIBUTING.md` with TODO Management section
   - Documented the single source of truth location
   - Added guidelines for adding new tasks
   - Referenced archive location for historical records

3. **Unique Content Analysis**
   - Reviewed both files for unique content
   - Main `.todo` contained comprehensive, detailed task specifications
   - `agentask.subject.todo` contained some unique subject items (SUBJ-* refs)
   - Subject items are preserved in archive for potential future migration if needed

## Consequences

### Positive
- Clear, unambiguous source for all TODO tracking
- Reduced maintenance overhead
- Consistent task format and structure
- Simplified onboarding for new contributors
- Better alignment with governance requirements (GOV-CODEOWNERS, DOC-001)

### Negative
- Some historical subject-specific items are now in archive only
- If subject inbox workflow was in active use, teams need to adapt to unified list

### Neutral
- Archive maintains full historical record
- Teams can extract archived items if needed for future work

## Compliance

This decision supports:
- **DOC-001**: Evidence trails & documentation requirements
- **GOV-SOT-EXEC**: Single source of truth for governance
- **OPS-TODO-VALIDATE**: CI validation against single schema
- **OPS-ISSUE-SYNC**: Simplified GitHub Issues synchronization

## Evidence

- **Archive:** `/home/user/agentaskit/agentaskit-production/archive/legacy_todos/agentask.subject.todo.20260102`
- **Updated Docs:** `/home/user/agentaskit/CONTRIBUTING.md`
- **Source of Truth:** `/home/user/agentaskit/agentaskit-production/.todo`
- **This ADR:** `/home/user/agentaskit/agentaskit-production/docs/decisions/ops-dedup-todo.md`

## Acceptance Criteria Met

- [x] `.todo` maintained as single source
- [x] `agentask.subject.todo` archived with timestamp
- [x] Archive file includes note about current TODO location
- [x] `CONTRIBUTING.md` updated with TODO reference
- [x] Decision documented in evidence file
- [x] Task marked complete in `.todo`

## Related

- **Depends On:** DOC-001 (Evidence trails requirement)
- **Supports:** GOV-CODEOWNERS, OPS-TODO-VALIDATE, OPS-ISSUE-SYNC
- **See Also:** `agentaskit-production/docs/` for other decision records
