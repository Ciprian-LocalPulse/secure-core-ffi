# Manifesto

**security-core-ffi** — a project by Ciprian Ștefan Pleșca, independent researcher

## Why this exists

Cryptographic mistakes are rarely made by people who don't understand cryptography. They are made by people who understand it perfectly well, working under deadline, in a language that doesn't stop them from reusing a buffer, forgetting to free a key, or copy-pasting a nonce-handling routine from one part of a codebase to another where the assumptions no longer hold.

This project starts from a simple conviction: security-critical logic should be written **once**, in a language whose compiler refuses to let entire categories of mistakes compile, and every other part of a system should consume that logic through a contract small enough to read in one sitting — not reimplement it from scratch in whatever language happens to be convenient that week.

## What we believe

**Correctness is not a feature, it is the baseline.** A cryptographic library that is fast, elegant, and occasionally wrong is not a cryptographic library — it is a liability with good documentation. Every design decision in this project has been made in favor of eliminating a failure mode, even when a more "flexible" alternative existed.

**A small, auditable surface beats a large, convenient one.** Five functions are easier to reason about, test exhaustively, and audit line by line than fifty. This project will resist growing its public API for the sake of convenience if doing so widens the space of ways it can be misused.

**Boundaries should be stated, not implied.** What this library guarantees, and what it explicitly does not guarantee, are both documented — not as a disclaimer buried in fine print, but as a first-class part of the architecture. A system builder should never have to *assume* what a security core does; they should be able to *read* it.

**Open documentation is not optional.** Code without an explanation of *why* it was written that way is a black box that happens to compile. This project publishes its architecture, threat model, and known limitations publicly, in the same repository as the code, so that anyone — a reviewer, a contributor, a skeptic — can verify the reasoning, not just trust it.

## Why independent

This project is written and maintained independently, without institutional funding, corporate sponsorship, or academic affiliation. That independence is not incidental — it means every design decision here was made because it was judged to be correct, not because it satisfied a roadmap, a client requirement, or a funding body's priorities.

It also means this work has real costs — time, infrastructure, the ordinary expenses of sustained technical work — carried without external support. If this project is useful to you, in your own systems or simply as documentation you learned something from, you can help sustain it: see [DONATE.md](DONATE.md).

## What this project is not

This is not a general-purpose cryptography library, and it will not try to become one. It does not aim to support every cipher, every mode, or every platform. It aims to do one thing — authenticated encryption, correctly, across language boundaries — and to document that one thing well enough that trusting it does not require blind faith.

## An invitation

If you read the [architecture](docs/en/Architecture.md), find a gap in the threat model, or see a place where the documentation claims more than the code delivers, that is exactly the kind of scrutiny this project was built to invite. Open an issue. Point out the flaw. A security core that cannot survive being questioned in public was never secure to begin with.

---

*Ciprian Ștefan Pleșca — contact@agentflow-enterprise.com*
