# Code of Conduct

This Code of Conduct outlines the expectations for participation in the MyBrigitte project. By participating, you agree to abide by these principles and procedures.

## Purpose

We want MyBrigitte to be a welcoming, respectful, and productive environment for everyone — contributors, maintainers, and users. This document defines our core values, project norms, technical standards, and how we handle conflicts.

---

## 1. Core Values

- **Transparency:** If you are stuck or cannot finish a task, say it immediately. It is better to ask for help early than to block the team late.
- **Respect:** We critique code, not people. All feedback in Code Reviews must be constructive.
- **Reliability:** We commit to the tasks we take. "Ghosting" (disappearing without notice) is a serious breach of this agreement.

---

## 2. Project Management & Organization

- **Methodology:** We use an Agile workflow on GitHub Projects.
- **Sprints:** We work in **1-week sprints**.
- **Daily Meeting:** Every **Day at 10:30**
  - _Agenda:_ Demo of finished tasks, retro of the week, assignment of new tasks.
  - _Attendance:_ Mandatory. Absence must be notified 24h in advance.
- **Updates:** We regularly send updates in the group chat about the progress of our tasks.

> If you cannot attend or meet a deadline, update the ticket and inform the team as soon as possible.

---

## 3. Technical Standards (The "Architect" Rule)

- **Language:** All code, comments, variables, and commits are in **English**.
- **Linter:** No code is pushed if the component checks (rustfmt/Clippy for Rust, ESLint/Prettier for clients) fails.
- **Documentation:** Document new API endpoints and their request and response formats. The README must be updated if setup steps change.
- **No "Spaghetti":** Keep business logic separate from Axum handlers. UI components must be reusable.

This ensures consistency, maintainability, and easier onboardings.

---

## 4. Git Workflow & Version Control

- **Branching Strategy:** We follow Gitflow principles:
  - `main`: Production-ready (only updated via PR).
  - `dev`: Integration branch (always stable).
  - `feat/branch-name`: New features.
  - `fix/name`: Bug fixes.
- **Commit Convention:** We follow **Conventional Commits**. Examples:
  - `feat(auth): add microsoft oauth`
  - `fix(login): resolve crash on login`
  - `docs: update readme`
- **Pull Requests (PR):**
  - **No direct pushes** to `main` or `dev`.
  - A PR requires **at least 2 approvals** from other teammates to be merged.
  - The CI pipeline (GitHub Actions) must pass (Build + Lint).

Include a clear description, testing steps, and a checklist in every PR.

---

## 5. Conflict Resolution

If a disagreement or technical deadlock arises, follow this process:

1. **Discussion:** Organize a synchronous call to discuss pros/cons.
2. **Vote:** If no consensus is reached, the team votes. Majority wins.
3. **Performance Issues:** If a member consistently misses deadlines or ghosts:
   - _Step 1:_ Private 1-on-1 check-in to understand blockers.
   - _Step 2:_ Team meeting to re-assign workload.
   - _Step 3:_ If behavior continues, escalate to the pedagogical team or project leads.

The goal is to resolve issues respectfully and quickly while supporting team members.

---

## Reporting and Enforcement

If you experience or witness behavior that violates this Code of Conduct, please report it to the maintainers. You can:

- Contact the repository owner(s) or project leads directly in private channels.

Reports will be handled confidentially. Maintainers will investigate and may take actions ranging from a private warning to removal from the project depending on the severity.

---

## Acknowledgements

This Code of Conduct draws on common community standards and is intended to keep the MyBrigitte project inclusive and productive for contributors of all backgrounds.

---

By participating in this project, you agree to respect these guidelines and help maintain a positive, collaborative community.
