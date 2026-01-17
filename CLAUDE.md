# Claude Instructions for Paradiddle.rs

This file contains evolving instructions and guidelines for AI agents (e.g., Claude Code) working on the **Paradiddle.rs** project.  Update this file whenever the agent makes mistakes or when new patterns emerge.  These instructions are designed to support a multi‑workspace, AI‑assisted workflow.

## General Workflow

1. **Plan First:** Always operate in **plan mode**.  Before making changes, outline your approach in natural language.  This helps identify missing context and ensures transparency for reviewers.

2. **Research Before Coding:** For each task, search for existing open‑source Rust code that solves similar problems.  Clone relevant repositories into the `./_refs/` directory (which is git‑ignored) and extract edge cases, test patterns, and design insights【768676599730584†screenshot】.  Do *not* import these packages as dependencies; instead, learn from their code.

3. **Update Project Artifacts:** Maintain the following documents:
   - `docs/milestone1/journal.md` – log your actions, decisions and challenges.
   - `docs/milestone1/roadmap.md` – list tasks with priorities `P0` (urgent), `P1` (important), and `P2` (nice to have).  When you complete a task or discover new work, update this roadmap accordingly【225839041741822†screenshot】.
   - `CLAUDE.md` (this file) – record recurring mistakes, style guidelines and additional instructions.

4. **Pull Request Flow:** Each workspace should work towards creating a pull request (PR).  Ensure that PRs include:
   - A summary of what was done and why.
   - A description of test coverage and a link to the coverage report.
   - Questions to the reviewing agent requesting scenarios or edge cases that might be missing【298425262083167†screenshot】.

5. **Coding Style:** Follow the Rust 2021 edition guidelines and adopt idiomatic patterns where possible.  Read through sibling repositories (e.g., the original `paradiddle` project) to understand the preferred coding style, architecture choices and comment patterns【179483756323432†screenshot】.

6. **Human Reviews:** When human feedback is required, expect a transcript that contains comments about the PR.  Parse these transcripts, extract actionable items and create subtasks in the roadmap at the appropriate priority【298425262083167†screenshot】.

## Phase 1 Goals

During Milestone 1, focus on the tasks outlined in `docs/milestone1/roadmap.md`.  Implement a basic event system, a dependency injection container, window abstractions and a simple demonstration using the `ratatui` library【6955392274892†L521-L533】【6955392274892†L759-L770】.  Ensure all code compiles and include tests where appropriate.  Use the `_refs` directory to pull in examples of event systems (e.g., from `vscode`), DI containers (e.g., `shaku`), and simple TUI demos.
