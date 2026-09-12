---
name: code-review
description: Reviews pull requests and code changes for defects, security, and quality
  using parallel independent review passes with confidence scoring. Use when a PR
  or code change needs review.
---

Provide a code review for the given pull request or code change.

To do this, follow these steps precisely:

1. **Eligibility check**: Determine if the change (a) is closed, (b) is a draft, (c) does not need a code review (automated change, very simple and obviously fine), or (d) already has a review from you earlier. If so, do not proceed.
2. **Project conventions**: Collect the relevant project instruction files (root `AGENTS.md`/`CLAUDE.md` if present, plus any in directories whose files the change modified). List paths only, not contents.
3. **Summary**: Review the change (diff or PR) and produce a concise summary of what changed.
4. **Parallel independent reviews**: Launch several independent review passes (parallel subagents or manual passes), each with a distinct lens:
   a. **Pass #1 — Conventions**: Audit the changes against project conventions (AGENTS.md/CLAUDE.md). Note: conventions are guidance for writing code, not all instructions apply during review.
   b. **Pass #2 — Shallow bugs**: Read the file changes only; scan for obvious bugs. Avoid extra context beyond the changes. Focus on large bugs, skip nitpicks and likely false positives.
   c. **Pass #3 — History**: Read `git blame` and history of modified code to find bugs in light of historical context.
   d. **Pass #4 — Prior changes**: Look at previous PRs/changes that touched these files; check whether their comments/decisions also apply here.
   e. **Pass #5 — Comments**: Read code comments in modified files; ensure the change complies with guidance expressed in comments.
5. **Confidence scoring**: For each issue from step 4, score confidence that it is real (not a false positive). For issues flagged against conventions, double-check the convention actually calls it out. Scale:
   - **0** — Not confident at all: false positive, or pre-existing issue.
   - **25** — Somewhat confident: might be real, but unverified; stylistic unless explicitly in the conventions.
   - **50** — Moderately confident: verified real, but nitpick or rare; not very important.
   - **75** — Highly confident: double-checked, very likely hit in practice; directly impacts functionality or is explicitly in the conventions.
   - **100** — Absolutely certain: confirmed real, occurs frequently; evidence directly confirms.
6. **Filter**: keep only issues scored >= 80. If none qualify, report no issues.
7. **Re-check eligibility** (repeat step 1) in case the change moved while reviewing.
8. **Report**: present findings as the final output. If commenting on the PR, keep it brief, no emojis, cite and link code/files.

Examples of false positives (for steps 4 and 5):

- Pre-existing issues
- Something that looks like a bug but is not actually a bug
- Pedantic nitpicks a senior engineer wouldn't raise
- Issues a linter, typechecker, or compiler would catch (imports, type errors, broken tests, formatting). Assume these run separately in CI.
- General quality issues (test coverage, generic security, documentation) unless explicitly required by project conventions
- Issues called out in conventions but explicitly silenced in code (e.g. lint-ignore comments)
- Functionality changes that are likely intentional or directly related to the broader change
- Real issues on lines the author did not modify

Notes:

- Do not attempt to build or typecheck the app — that runs separately and is not part of this review.
- Make a todo list first.
- Cite and link each finding (e.g. if referring to a convention file, link it).
- Final report format (example, 3 issues found):

---

### Code review

Found 3 issues:

1. <brief description> (AGENTS.md says "<...>")
   <link to file and line with full commit sha + line range, e.g. repo/blob/<sha>/path#L4-L7>

2. <brief description> (some/other/AGENTS.md says "<...>")
   <link with full sha + line range>

3. <brief description> (bug due to <file and code snippet>)
   <link with full sha + line range>

---

Or, if no issues found:

---

### Code review

No issues found. Checked for bugs and project-convention compliance.

---

- Link format: `https://github.com/<owner>/<repo>/blob/<full-sha>/<path>#L<start>-L<end>` — requires the full commit sha, repo name matching the reviewed repo, and at least 1 line of context around the commented line (e.g. commenting lines 5-6 → link `L4-7`).

<!-- Source: Anthropic claude-plugins-official (code-review), adapted for opencode -->
