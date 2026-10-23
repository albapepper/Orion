# Orion — North Star

This is the governing product intent for all future Orion work, aesthetic
and mechanical. These principles describe what we must build toward; they
are not claims that the current implementation already satisfies them.
They do not override security requirements, repository execution instructions,
or the user's authorization for a particular task.

## The core concept

> “The plugins are the climbing equipment that allow the user to summit the
> mountain. They all have different jobs, different roles, but they all work
> together to summit.” — User

The user is the climber. Plugins are focused equipment. The summit is the
user's real mortgage-work goal: moving a loan forward through clear facts,
coordinated work, and deliberate decisions. **Different capabilities, one
coherent kit. Use only what is needed.**

## Principles for every change

- **The user controls the climb.** Make the goal, current context, available
  actions, and their consequences understandable. AI supports the user's
  judgment; relevance alone is not permission to act. Neither maximal feature
  count nor maximal AI autonomy is the objective.
- **Give each tool a bounded purpose.** A plugin should have a clear job,
  explicit responsibilities, and understandable limits. Add capability when
  it serves a real need; reuse what already performs the job before adding
  another tool or abstraction.
- **Let the parts cooperate without getting in each other's way.** Share
  coherent contracts, data meanings, loan context, and interaction conventions.
  Preserve continuity across tools without silently widening access or scope.
  Avoid duplicated responsibilities, conflicting state, obstructive panels,
  and competing demands for attention. Equipment should be available when
  relevant and quiet otherwise.
- **Keep the kit as simple as the work allows.** Remove unnecessary steps,
  choices, dependencies, and decoration. Preserve necessary safeguards,
  permissions, validation, auditability, and recovery. Simplicity must not
  conceal consequential actions or erase useful information.
- **Treat mortgage work with precision.** Make amounts, identities, save
  state, locks, errors, and limitations clear. Design for dependable behavior,
  explicit failure, and recovery; verify these qualities before claiming them.
  Distinguish demonstrations and shells from connected, working capabilities.
- **Make customer experience part of the mechanism.** Appearance, usability,
  accessibility, and engineering quality belong to the same design. Shared
  controls should look and behave consistently. A sophisticated workstation
  must also be legible, keyboard-accessible, comfortable, and easy to trust
  through observable behavior.

## Confirmed visual commitments

Orion has its own technical climbing-equipment character: purposeful,
precise, and distinctive, avoiding corporate blandness and generic SaaS
styling. The influence guides construction and use, not copied logos or
literal climbing scenery.

- Yellow on gray, with a soft-gray light theme.
- The same subtly transparent, glassy treatment on the left navigation rail
  and right chat rail, with tabs in the central workspace.
- A prominent yellow ORION wordmark atop the left rail, built from big,
  blocky terminal-style lettering.
- One chat rail toggling between AI and a Slack shell. Connection status
  must be truthful; a shell does not establish Slack integration.

[Aesthetic Vision](docs/AESTHETIC_VISION.md) develops this intent into visual
and interaction proposals. Its dimensions, swatches, and implementation
sequence remain testable details subordinate to this North Star. Where older
appearance notes differ, these confirmed commitments guide future design.
This document does not prescribe a new plugin architecture or certify
security, provider readiness, or production readiness.

## Change-review questions

Before accepting a product, design, or engineering change, ask:

- **Necessity and purpose:** What customer need does this serve, and is its
  responsibility clear? Can an existing part serve it?
- **Interoperability:** Does it fit shared contracts and conventions while
  preserving context, boundaries, and user control?
- **Customer clarity:** Will the user understand its state and next action?
  Does it help progress without competing with or obstructing other work?
- **Accessibility:** Can people perceive, understand, and operate it with
  keyboard access and appropriate visual alternatives?
- **Reliability:** What evidence supports its behavior, including failure
  and recovery? Are safeguards and readiness claims accurate?
- **Complexity:** What can be omitted? Does this strengthen the coherent
  kit, or merely add capability, autonomy, or decoration?
