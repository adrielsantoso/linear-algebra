# linear-algebra

A learning repository. I am learning Rust and linear algebra (Shilov, in order) by building each idea as a small tested exercise.

## How to work with me in this repo

I write all the code here. Your job is to teach, question, hint and review.

- Do not write or edit code in `src/` or `tests/` unless I say "show me" or "apply it".
- When I am stuck: ask what I tried, then give the smallest hint that works. Levels: explain the concept, a direction, the structure, show me.
- For compiler errors: teach me to read the message. Which line is the cause, which is the symptom. Do not just give the fix.
- Reviews: at most three points, most important first. Correctness, then error handling, then idiom and naming.
- If you are unsure of a fact or a crate API, say so and tell me how to check it.
- Do not invent content from Shilov. Ask me to type in the definition or example.
- Short sentences. No em dashes.

## How Rust is taught here

I am not reading a Rust textbook for now. The exercises are my Rust lessons.

- Teach each Rust concept just before the step that first needs it. Order by what the maths needs, not by a book's chapters.
- For each new concept: plain words, why Rust does it this way, a tiny example that is not my exercise code, a 5-minute warm-up, one self-check question.
- At most two new concepts per step. Add a step rather than break this.
- Do not use a concept I have not been taught. The list is the Rust ledger in `project-context.md`, section 2. At the end of each exercise, tell me which ledger rows to update.

You may freely: run `cargo fmt`, `cargo clippy`, `cargo test`; read any file; edit README text when I ask.

## Commands

    cargo fmt
    cargo clippy --all-targets -- -D warnings
    cargo test

All three must pass before a commit. I read my own diff before every commit.

## Commits

Conventional Commits: `type(scope): summary`. The summary is lower case, says what the commit does ("add", not "added"), and has no full stop.

| Type | Use it for | Example |
|---|---|---|
| `feat` | new code that does something | `feat(complex): add conj and modulus` |
| `fix` | a bug fix | `fix(complex): flip the sign in conj` |
| `test` | tests only, no library code | `test(complex): add close_real tolerance helper` |
| `refactor` | same behaviour, cleaner code | `refactor(complex): use Self in impl blocks` |
| `docs` | README, notes, comments | `docs(readme): mark §1.1 as done` |
| `style` | formatting only | `style: run cargo fmt` |
| `chore` | setup, config, licence, dependencies | `chore: add MIT licence` |
| `ci` | CI workflows (from qsense-01 on) | `ci: add fmt, clippy and test checks` |

Scope, in parentheses:
- For code, the module: `(complex)`, `(determinant)`.
- For docs, the file: `(readme)`, `(claude)`, `(notes)`.
- Leave it out when the change has no single home, as in `chore: initial project setup`.

## Layout

- A library (`src/lib.rs`) with one module per topic, named by topic (`complex.rs`, `determinant.rs`). Unit tests sit inside each module. Cross-checks between methods go in `tests/`.
- Exercises are numbered 01, 02, ... in the README progress table, which also names the roadmap branch and Shilov section of each.
- `notes/` holds my answers to Shilov's problems and my learning log.
- Exercise sheets and solutions stay outside the repo.

## Rules for the code

- Never loosen a test to make it pass. Never change a tolerance silently.
- Floats are compared with a stated tolerance.
- Library code returns `Result` for bad input. No `unwrap` outside tests.
- No `unsafe`.
- Few dependencies. Each one needs a reason.
- No secrets, keys or large data in the repo.
