Got it — you’d like a **Product Requirements Document (PRD)** that an agent could use as guidance for analyzing a codebase and suggesting LOC (lines of code) reductions without breaking functionality. Here’s a structured draft:

---

# Product Requirements Document (PRD)

**Title:** Codebase Line Reduction Assistant
**Objective:** Analyze a large codebase and recommend strategies to reduce lines of code (LOC) while maintaining the same functionality, readability, and maintainability.

---

## 1. Problem Statement

Large codebases often accumulate redundant, verbose, or outdated code. This increases technical debt, makes onboarding harder, and slows down development. Reducing LOC through safe, structured refactoring improves maintainability, reduces bugs, and lowers long-term costs. The goal is not *just fewer lines* but *better code quality*.

---

## 2. Goals & Non-Goals

### Goals

* Identify opportunities to **reduce LOC** without changing functionality.
* Recommend **refactorings** that improve clarity, maintainability, and testability.
* Support **multi-language projects** (common cases: Java, Python, TypeScript/JavaScript, C++).
* Provide **actionable, context-specific suggestions** with examples.

### Non-Goals

* Blindly minimize LOC at the cost of readability.
* Rewrite code in different languages or frameworks.
* Remove business logic.

---

## 3. Key Features / Requirements

The agent should be able to:

### 3.1 Code Analysis

* **Detect duplication**: Identify repeated code blocks across files/classes.
* **Find unused code**: Detect dead functions, variables, imports, or obsolete features.
* **Identify verbose patterns**: Long if-else chains, repetitive loops, manual wiring.
* **Check library usage**: Spot custom utilities that can be replaced with standard libraries.
* **Analyze abstractions**: Find opportunities to consolidate with design patterns or generic functions.

### 3.2 Refactoring Suggestions

The agent should recommend:

1. **Refactor & DRY** – Extract repeated code into reusable functions or modules.
2. **Use built-in features** – Replace manual implementations with standard libraries/framework methods.
3. **Higher-level abstractions** – Apply declarative style (pipelines, annotations, configuration).
4. **Modern language features** – Lambdas, comprehensions, destructuring, pattern matching, type inference.
5. **Configuration vs. code** – Move hardcoded logic into configs/environment variables.
6. **Code generation/macros** – Suggest boilerplate reduction using tooling.
7. **Dead code removal** – Flag unused classes, imports, and variables.
8. **Functional style (where appropriate)** – Replace imperative loops with `map/filter/reduce`.
9. **Modularization** – Break large files into smaller, focused modules for clarity.

### 3.3 Automation / Tooling Integration

* Integrate with **linters** (ESLint, Pylint, Checkstyle).
* Integrate with **static analysis tools** (SonarQube, PMD, Flake8).
* Use **IDE refactoring APIs** where available.
* Provide **safe automatic refactors** where risk is low (e.g., unused imports).

---

## 4. Success Metrics

* **LOC Reduction %**: Lines of code reduced without loss of functionality.
* **Duplication Reduction %**: Amount of duplicate code removed.
* **Complexity Reduction**: Measured via cyclomatic complexity and maintainability index.
* **Developer Adoption**: % of suggestions accepted/applied by developers.

---

## 5. Risks & Mitigations

* **Risk:** Reduced LOC but harder readability.

  * *Mitigation:* Prioritize readability; reject over-compressed one-liners.
* **Risk:** Breaking functionality.

  * *Mitigation:* Require passing test suite before and after changes.
* **Risk:** Over-aggressive dead code removal.

  * *Mitigation:* Check references via static + dynamic analysis.

---

## 6. Deliverables

* **Automated analysis report**: Ranked list of LOC reduction opportunities.
* **Refactoring recommendations**: With examples before/after.
* **Optional automated patches**: For trivial fixes (unused imports, repeated literals).

---

## 7. Example Workflow

1. Agent scans codebase.
2. Generates a report:

   * "Function `parseUserData` duplicated in 3 places. Suggest extracting into utility module."
   * "20 unused imports in `UserService.java` can be removed."
   * "Nested `if-else` chain in `PaymentProcessor.js` can be replaced with a lookup map."
3. Provides code snippets with suggested changes.
4. Developer reviews, approves, and applies changes.


