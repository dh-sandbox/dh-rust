## Code Quality

This project uses Qlty for code quality. Config: `.qlty/qlty.toml`

- `qlty fmt` - auto-format
- `qlty check --fix --level=low` - lint + auto-fix
- `qlty check path/to/file.ts` - check specific files
- `qlty smells path/to/file-or-dir` - inspect relevant code for duplication and maintainability issues
- `qlty smells --all` - run a full-project smells analysis when needed

Before committing, confirm `qlty check --level=low` passes with no issues.
Before implementing a new module, feature, or larger refactor, run `qlty smells` on the relevant file or directory to learn existing patterns and avoid duplication.
Use `qlty smells --all` for broader reviews when the task affects multiple parts of the codebase.
