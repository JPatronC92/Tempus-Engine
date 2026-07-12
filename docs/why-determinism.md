# Why Determinism Matters

## The Problem

In financial services, insurance, lending, and compliance, **rule evaluation must be deterministic**: the same input must always produce the same output, regardless of when or where it runs.

Non-deterministic systems create:

- **Audit failures** — regulators demand reproducible decisions
- **Debugging nightmares** — "it worked yesterday" is not acceptable
- **Compliance risk** — unexplainable decisions violate regulations like ECOA, GDPR Article 22

## How Tempus Engine Guarantees Determinism

### 1. Pure Functions

Every evaluation is a pure function: `f(rule, context) → result`. No global state, no side effects, no randomness.

### 2. Versioned Rules

`RuleDefinition` carries a `version` field. When you pin a rule version, you get the same logic forever. Rule changes create new versions, not mutations.

### 3. No Hidden Dependencies

The engine has no database connections, no network calls, no file I/O during evaluation. All data comes through the context parameter.

### 4. Reproducible Batches

`execute_batch_detailed()` returns per-context results. Even if one context fails to parse, other contexts still evaluate correctly. The batch result is deterministic and complete.

## Design Principle

> **If you can't explain it, you can't ship it.**

Tempus Engine is designed so that every decision can be traced back to a specific rule version, a specific input, and a specific output. This is the foundation for explainability, auditability, and trust.
