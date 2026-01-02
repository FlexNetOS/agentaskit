# Contributing

- Follow SoT updates and evidence rules
- Run cross-reference before PR
- Keep .todo entries valid

## TODO Management

The single source of truth for all TODO items is:
- `/home/user/agentaskit/agentaskit-production/.todo`

All tasks, priorities, and tracking should reference this file. Legacy TODO files have been archived to `agentaskit-production/archive/legacy_todos/` for historical reference.

When adding new tasks:
1. Follow the format specified in `.todo`
2. Include: owner, deps, acc (acceptance criteria), evid (evidence), REF, tags, and context
3. Ensure CI validation passes (OPS-TODO-VALIDATE)
4. Keep entries actionable and measurable
