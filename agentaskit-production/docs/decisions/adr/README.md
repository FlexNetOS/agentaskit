# Architecture Decision Records (ADR)

## Purpose

Architecture Decision Records (ADRs) document significant architectural and technical decisions made in the AgentAskit project. They capture the context, options considered, decision made, and consequences to provide a historical record and rationale for future reference.

## When to Write an ADR

Create an ADR when making decisions about:

- **Architecture:** Component structure, service boundaries, data models
- **Technology Selection:** Languages, frameworks, databases, infrastructure
- **Patterns and Practices:** Design patterns, coding standards, operational procedures
- **Security:** Authentication schemes, encryption methods, security controls
- **Performance:** Optimization strategies, scaling approaches, caching policies
- **Process:** Development workflows, deployment strategies, testing approaches
- **Integration:** External service integration, API contracts, protocols

## ADR Process

### 1. Proposal Phase

When facing a significant decision:

1. **Identify the decision** to be made and its scope
2. **Research options** and gather relevant information
3. **Draft an ADR** using the template (see `0000-template.md`)
4. **Assign an ADR number** (next available sequential number)
5. **Share for feedback** with relevant stakeholders

### 2. Review Phase

1. **Circulate the draft** to team members, architects, and domain experts
2. **Gather input** on options, trade-offs, and consequences
3. **Iterate on the ADR** based on feedback
4. **Build consensus** or escalate conflicts to decision authority

### 3. Decision Phase

1. **Make the decision** and update ADR status to "Accepted"
2. **Document dissenting opinions** if consensus wasn't reached
3. **Commit the ADR** to the repository
4. **Update the ADR index** in this README
5. **Reference the ADR** in related code, documentation, and tasks

### 4. Implementation Phase

1. **Implement the decision** according to the ADR
2. **Update ADR** if implementation reveals new information
3. **Link ADR** to relevant pull requests and issues

### 5. Maintenance Phase

- **Review periodically** to ensure decision remains valid
- **Update status** if decision is superseded, deprecated, or amended
- **Create new ADR** if decision needs to be changed (don't modify old ADRs)

## ADR Statuses

- **Proposed:** Draft under review, decision not yet made
- **Accepted:** Decision made and active
- **Deprecated:** Decision superseded but code may still exist
- **Superseded:** Replaced by a newer ADR (link to new ADR)
- **Rejected:** Proposed but decided against
- **Amended:** Modified by a subsequent ADR (link to amendment)

## ADR Template

Use the template at `0000-template.md` when creating new ADRs. The template includes:

- **Title:** Short, descriptive name for the decision
- **Status:** Current status (Proposed, Accepted, etc.)
- **Context:** Background and problem statement
- **Decision Drivers:** Factors influencing the decision
- **Considered Options:** Alternatives evaluated
- **Decision:** The chosen option and rationale
- **Consequences:** Positive and negative outcomes
- **References:** Related documents, discussions, ADRs

## Naming Convention

ADRs are numbered sequentially and use the format:

```
NNNN-short-title-with-dashes.md
```

Examples:
- `0001-use-markdown-for-documentation.md`
- `0002-adopt-rust-for-agent-runtime.md`
- `0003-implement-capability-based-authorization.md`

## ADR Index

| ADR # | Title | Status | Date | Author |
|-------|-------|--------|------|--------|
| [ADR-001](../../decisions/ops-dedup-todo.md) | Deduplicate TODO Sources (Single SoT) | Accepted | 2025-10-05 | @program |

## Best Practices

### Writing Effective ADRs

1. **Be concise but complete:** Cover all essential information without unnecessary detail
2. **Focus on "why" not "how":** Explain rationale, not implementation details
3. **Document alternatives:** Show what options were considered and why they were rejected
4. **Be honest about trade-offs:** Document both benefits and drawbacks
5. **Update status:** Keep ADRs current by updating status when decisions change

### Common Pitfalls to Avoid

- **Too late:** Writing ADR after implementation is complete
- **Too early:** Writing ADR before sufficient information is available
- **Too detailed:** Including implementation specifics that will change
- **Too vague:** Not providing enough context for future readers
- **Never updated:** Letting ADRs become stale and misleading

### Reviewing ADRs

When reviewing an ADR:

- [ ] Is the context clear and complete?
- [ ] Are all reasonable options documented?
- [ ] Is the decision rationale well-explained?
- [ ] Are consequences (both positive and negative) identified?
- [ ] Are there any unstated assumptions?
- [ ] Does this conflict with existing ADRs?
- [ ] Is the status appropriate?

## ADR Lifecycle Example

**Week 1:** Identify need to choose database for agent state
- Create `0005-select-agent-state-database.md` with status "Proposed"
- Document PostgreSQL, Redis, DynamoDB as options

**Week 2:** Review with team
- Gather feedback on options
- Add more detail on trade-offs
- Update with benchmark results

**Week 3:** Make decision
- Update status to "Accepted"
- Document PostgreSQL as chosen option
- Commit to repository

**Month 1-6:** Implementation
- Reference ADR-005 in related PRs
- Update ADR if new information emerges

**Year 1:** Performance issues
- Create `0023-add-redis-cache-layer.md` to amend ADR-005
- Update ADR-005 status to "Amended" with link to ADR-023

**Year 2:** Major refactor
- Create `0048-migrate-to-distributed-state-store.md`
- Update ADR-005 status to "Superseded" with link to ADR-048

## Tools and Automation

### Creating a New ADR

```bash
# Use the helper script to create a new ADR
./scripts/create-adr.sh "Short title of decision"

# This will:
# - Assign the next available number
# - Copy the template
# - Set creation date
# - Open in your editor
```

### Listing ADRs

```bash
# List all ADRs with status
./scripts/list-adrs.sh

# List only accepted ADRs
./scripts/list-adrs.sh --status accepted
```

### Validating ADRs

```bash
# Check ADR format and completeness
./scripts/validate-adr.sh docs/decisions/adr/0005-example.md

# Validate all ADRs
./scripts/validate-all-adrs.sh
```

## Integration with Workflow

ADRs integrate with our development workflow:

1. **Pull Requests:** Reference relevant ADRs in PR descriptions
2. **Code Comments:** Link to ADRs explaining architectural choices
3. **Documentation:** Reference ADRs in architecture documentation
4. **TODO Tasks:** Link ADRs in `.todo` file for major initiatives
5. **Source of Truth:** Link ADRs in `core/src/orchestration/sot.md`

## Governance

- **ADR Review:** Architecture team reviews all proposed ADRs
- **Decision Authority:**
  - Technical decisions: Engineering Lead
  - Architectural decisions: Chief Architect
  - Strategic decisions: CTO/VP Engineering
- **Dispute Resolution:** Escalate to next level of authority
- **Compliance:** ADRs are part of audit and compliance evidence

## Related Documentation

- [Risk Register](../../risks/register.md) - Track risks identified in ADRs
- [Source of Truth](../../core/src/orchestration/sot.md) - Link executed decisions
- [Contributing Guide](../../../CONTRIBUTING.md) - Development standards
- [Operations Deduplication Decision](../../decisions/ops-dedup-todo.md) - ADR-001

## Questions?

For questions about the ADR process:
- **Process questions:** Contact @program or @docs team
- **Technical questions:** Contact @architecture team
- **Tooling issues:** Contact @platform team

## References

- [Architecture Decision Records (ADR) by Michael Nygard](https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions)
- [ADR GitHub Organization](https://adr.github.io/)
- [Markdown Any Decision Records (MADR)](https://adr.github.io/madr/)

## Version History

| Version | Date | Changes | Author |
|---------|------|---------|--------|
| 1.0 | 2026-01-02 | Initial ADR process documentation | @program |
