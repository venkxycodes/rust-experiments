# Vendored productivity skills

Source: https://github.com/venkxycodes/cursor-productivity-skills
Revision: `ca9f61131540d32008f4e8dd484014367f385bfc`
License: MIT; the upstream notice is preserved in [LICENSE](LICENSE).

The seven skill directories are unmodified copies, including their supporting
references, scripts, and agent metadata. They are installed locally for Codex in
this repository; no user-global installation is required.

Selected: `codebase-design`, `design-an-interface`, `code-simplification`, `deslop`,
`architecture-review`, `adversarial-review`, and `summarize-diff`. These support
Rust module/API design, behavior-preserving maintenance, and review. The source
collection does not contain a Rust-specific skill.

Excluded: frontend/UI and dashboard skills, Python/Django skills, database and
migration skills, HTML/UI prototyping, grilling, prompting, and prevention
workflows. They are not needed for the current Rust experimentation scope.

Optional references to excluded skills or external tools do not install them.
Use the applicable fallbacks described in each skill and the Rust checks in the
root AGENTS.md. In particular, repo-intel is optional.

To update, review a new upstream revision, install only these selected directories,
preserve the license, and update this revision record. Do not run the upstream
bootstrap installer, which installs the entire collection.
