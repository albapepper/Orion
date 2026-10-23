# Orion — Aesthetic Vision

**Status:** sponsor presentation and first-build direction, not certification
of integration, persistence, or production readiness. **Date:** October 23, 2026.

This vision applies the governing product intent in
[NORTH_STAR.md](../NORTH_STAR.md). Its detailed proposals are subordinate to
that document, which governs both aesthetic and mechanical work.

## Applying the North Star — one coherent kit

> “The plugins are the climbing equipment that allow the user to summit the
> mountain. They all have different jobs, different roles, but they all work
> together to summit.” — User

The user is the climber; plugins are purpose-built equipment. This confirmed
concept comes from the root North Star. The summit is the user's mortgage
goal: moving a loan forward through clear facts, coordinated review, and
deliberate decisions.

Each plugin needs a bounded purpose. Complementary capabilities share control
conventions, state labels, keyboard behavior, and loan, borrower, and stage
context. Equipment must be dependable, available when relevant, and quiet
otherwise. The user chooses the next move; contextual assistance alone does
not authorize consequential actions.

Visual identity and mechanical cohesion reinforce each other: strong lettering,
yellow on gray, matched glass rails, and consistent controls make distinct
tools feel like one kit. Orion should feel composed, capable, and comfortable
through a long day. These are design goals, not verified reliability or
security claims, and do not prescribe a new plugin architecture. Borrow
equipment's practical clarity, not brand identities or literal climbing scenery.

## Confirmed direction and proposed starting choices

**Confirmed by the user:**

- Yellow on gray, with a soft-gray light theme.
- A sophisticated, easy-to-use identity that avoids corporate blandness and
  generic SaaS styling.
- Matching, slightly transparent glassy rails: navigation left, chat right.
- One right rail with an AI / Slack toggle, not separate AI and Slack rails.
- Tabs in the central workspace between the rails.
- A prominent yellow ORION wordmark atop the left rail, with big, block-built
  terminal-style letters.
- The climbing-equipment concept above governs appearance and mechanics;
  plugins have distinct jobs within one coherent kit.

**Proposals:** solid central surface, selective yellow, equal rail widths,
mostly square controls, condensed display type with readable system text.
Dimensions, colors, timings, and tab organization need testing. Light leads;
the existing dark theme needs separate validation.

## Repository starting point

[App.tsx](../src/App.tsx) composes `NavTray`, the routed `workspace`, and
`ChatBox`. [App.css](../src/App.css) uses a 248px / flexible / 348px grid,
compact breakpoints, matching translucent rails, and stone-like texture.
Replace its gray-green palette and earlier brown/copper overrides with
neutral grays, yellow, and less decoration in the future visual build.

[README](../README.md) establishes keyboard access, generous space, and focus
on one loan; retain those principles. Its palette, widths, and navigation
descriptions lag the code. This vision develops the North Star's direction.

`NavTray` now holds app navigation and recent loans. `PizzaTracker` holds loan
stages centrally, with tab semantics; borrower selectors add another local
tab level. There is no general open-workspace tab manager. `ChatBox` currently
renders AI only; no Slack rail component or integration was found in the
inspected source. The shared toggle and Slack shell below are future work.

## Shell hierarchy and proportions

Use three stable regions without a spanning header. Align rail headers and
central tabs in an 88–104px top band, giving the wordmark breathing room.

At 1440px, try 280px rails and an 880px center. Matching material remains
required if testing favors unequal widths. Try 20px rail padding, 24–32px
central padding, and 16–24px between groups. Rails and center scroll
independently, with visible boundaries.

The center reads in order: workspace tabs, `LoanHeader` identity and lock
state, loan stages, then work. Underwriter review stays in the center.

At narrower desktop widths, reduce shared rail widths modestly and stack
review below entry before squeezing form labels. Use a contained horizontal
scroll region for genuinely wide tables. Test around the existing 1180px
breakpoint and at 200% zoom: fixed columns must not strand controls offscreen.
If the three-column layout cannot remain usable, prototype an explicit compact
chat drawer with focus restoration; do not silently remove navigation or
force a fourth column.

## Palette and rail material

Assess these swatches in rendered context.

| Role | Proposal | Intended use |
| --- | --- | --- |
| Shell gray | `#D9DBD7` | Soft surrounding field |
| Work surface | `#F0F1ED` | Solid central reading and editing surface |
| Secondary gray | `#E4E6E1` | Inputs and quiet group separation |
| Main ink | `#252824` | Body text, values, primary labels |
| Secondary ink | `#565C54` | Supporting text that still needs reading |
| Structural edge | `#858C82` | Boundaries where separation matters |
| Brand yellow | `#E8C928` | Wordmark, selected markers, deliberate actions |
| Deep yellow/ochre | `#716000` | Candidate accent text on light surfaces |
| Graphite | `#343A32` | Optional backing behind the yellow wordmark |

Use yellow for identity, selection, or a primary action, with dark ink on
yellow buttons. Avoid yellow body text on pale gray, yellow placeholders,
or faint yellow-only focus rings. Test a graphite wordmark inset if needed
for legibility, retaining the shared rail material around it.

These are unmeasured proposals, not WCAG compliance claims. Verify actual
text/background pairs, control edges, focus indicators, and disabled states;
transparency changes the effective background. Keep errors, warnings, and
completion distinguishable by words and symbols as well as color. Brand
yellow must not make every selected item look like a warning.

Give both rails one material recipe: gray fill near 88% opacity, approximately
16–22px backdrop blur, one subtle inner highlight, and a crisp inward edge.
Keep their tint, opacity, blur, header rules, and control states identical.
Text and controls remain fully opaque. Remove decorative flecks and veins
from the first trial so the glass reads through restrained depth.

Provide an opaque gray version with the same geometry and boundaries when
blur is unsupported, transparency is reduced, or contrast needs demand it.
Offer an explicit opaque-surfaces setting where platform preferences cannot
be detected reliably. In forced-color modes, allow system colors and visible
borders to carry structure. Readability must never depend on a blur effect.

## Typography and the ORION wordmark

Use three roles:

- **Display:** the already installed Barlow Condensed, initially 600/700,
  for short section titles and selected equipment-like labels. Keep uppercase
  brief and avoid widely tracked tiny text.
- **Reading and controls:** the current system UI stack, around 15–16px for
  forms and conversation, with supporting text generally 13–14px. Labels
  remain visible above or beside fields, never only in placeholders.
- **Numbers and keys:** the existing monospace stack for loan identifiers,
  amounts where alignment helps, and shortcut hints. Align monetary columns;
  keep long notes in proportional text.

Build ORION as original block lettering on a consistent rectangular grid:
broad strokes, open counters, deliberate gaps, and a recognizably terminal
construction. Prototype a five- or seven-unit letter height, then assess
the whole name at roughly 210–240px wide. This is a custom wordmark, not merely
the word typed in monospace. Keep it yellow and prominent above navigation;
place company identity quietly below, away from its letterforms.

Use original inline vector blocks, accessible as “Orion,” not individually
announced. Avoid imitation CLI mascots, outdoor brand marks, distressed or
extruded letters, and animated typing. Use installed resources and system
fallbacks; no asset downloads or new font licensing are assumed.

## Tabs, navigation, and contextual focus

Propose a shallow central workspace tab strip for open app views or loans.
Each loan tab identifies the loan and borrower; selecting it restores that
loan's stage. Beneath it, retain a quieter stage row based on
`screens/loan/stages.ts` and `PizzaTracker`, rather than duplicating stages in
the left rail. Borrower selectors remain local to the form. Start with a
small number of tabs and an explicit overflow list; full names remain
available without relying on clipped text or mouse-only tooltips.

Active tabs join visually to the work surface, with a short yellow marker
and strong label. Inactive tabs are gray and legible. Hover, selection, and
keyboard focus must look different. Use mostly square boundaries and 2–4px
corner relief where useful; avoid a row of floating pills. Left navigation
keeps app destinations, recent loans, persona, and utilities in stable groups.

Place a labeled **AI / Slack** segmented selector at the top of the right
rail. Switching changes its contents without moving the shell, opening a
second rail, sending a message, or changing the active loan. Preserve each
mode's draft and scroll position. The initial Slack view should explicitly
say “Slack shell — not connected,” with no implied live channels, unread
counts, or enabled sending. Provider integration is a separate workstream.

AI context should identify the active loan and stage, or the app view.
Preserve screen dividers, but test loan switches too: today's `ChatBox`
divider tracks screen changes, not loan identity.
Do not suggest that switching to Slack shares loan context automatically.
Show actual configured AI provider/model status rather than unconditional
“Local” or “data stays on this device” copy.

## Keyboard and motion

Retain discoverable shortcuts and the legend in `KeyLegend`. Keep existing
Ctrl-modified page/stage commands and recent-loan access. Proposed `/`
behavior outside editors selects AI and focuses its composer; ordinary typing
must retain the slash character. Escape dismisses the nearest transient UI
or returns from chat to the invoking control without clearing drafts.

Tabs use arrow-key focus, Enter/Space activation,
and a visible focus outline distinct from selection. Closing a tab returns
focus to an adjacent tab. Changing chat mode leaves focus on its selector;
Tab enters its contents. Notifications and incoming messages never steal
focus. Preserve native text editing and review `App.tsx` alongside
`lib/fieldNav.ts`: current Home handling and README wording differ, so resolve
and document that behavior during implementation rather than assuming parity.

Try 100–150ms color/opacity transitions for hover and selection. Keep rail
geometry fixed; avoid sliding panels during ordinary switching, spring
motion, pulsing accents, or decorative loading loops. Respect reduced motion
with immediate state changes and no animated scrolling. Text status must
explain loading, saving, or failure even when motion is absent.

## Representative workspace: Income Analysis

Use [IncomeScreen](../src/screens/loan/IncomeScreen.tsx) and its shared
[LoanListScreen](../src/screens/loan/LoanListScreen.tsx) for the first concept.
Show loan identity and lock state, the Income stage, and borrower selection
where relevant. Present income type, monthly amount, prior-year monthly
amount, employer, years on job, verification, and notes in a readable work
area. Keep its existing total-monthly and two-year-average summaries plainly
labeled; appearance must not imply newly validated underwriting calculations.

Within `LoanColumns`, entry leads and `UnderwriterPanel` remains adjacent
when space permits. These complementary tools share loan context while
retaining distinct roles. Keep Pending / Verified / Flagged, review notes,
role restrictions, and lock explanations readable. Tasks and contacts support
the work below.

Show demo non-persistence, save errors, and empty states honestly. Use existing
demo data for a future review; do not invent completed saves, connected Slack,
live provider results, or screenshots as evidence of readiness. Pilot and
provider gates remain in [PILOT_CHECKLIST](PILOT_CHECKLIST.md) and the existing
execution documents, independent of aesthetic approval.

## Anti-patterns

Avoid mountain silhouettes, topographic wallpaper, climbing illustrations,
faux ruggedness, and copied logos. Avoid gratuitous gradients, luminous glass
cards everywhere, dashboard tiles for every number, giant greeting copy,
all-monospace paragraphs, and an assistant that visually outranks the loan.
Do not hide lock, save, demo, or connection states to make a presentation
look cleaner. Do not add a separate Slack sidebar or make yellow the only
way to locate focus, selection, or a problem.

## Small first-build sequence

1. Compose the shell, original wordmark, palette, and type roles in the
   existing `App.tsx` / `App.css` structure. Consolidate superseded style
   overrides during that implementation, not in this document task.
2. Apply one shared rail material to `NavTray` and the chat container; build
   the AI / Slack shell selector and honest disconnected state.
3. Establish central workspace tabs and their focus behavior, then style
   Income Analysis through shared loan components. Keep existing permissions
   and persistence behavior outside visual changes.
4. Review the same composition at presentation size, compact width, and
   enlarged text; test opaque surfaces, keyboard use, and reduced motion
   before extending the treatment to other screens.

## Visual and interaction acceptance checklist

**Design-review test:** What job does this plugin or visual choice serve?
Does it strengthen the coherent kit and help the user progress? Capability
or decoration alone is insufficient.

- [ ] Yellow ORION reads immediately; borrower identity remains the center's
      primary heading. The result feels original without borrowed logos.
- [ ] Rails visibly share one material; AI and Slack occupy one right rail.
- [ ] Tabs, stage navigation, borrower selectors, and keyboard focus are
      distinguishable; switching preserves the intended work context.
- [ ] Body text, amounts, errors, and read-only information remain legible;
      rendered contrast is measured before accessibility claims are made.
- [ ] No essential controls disappear at compact widths or 200% zoom;
      scroll regions and overflow controls remain keyboard reachable.
- [ ] Every core action is reachable without a mouse; focus is visible and
      restored predictably, with no lost drafts or unexpected sends.
- [ ] Opaque and reduced-motion variants retain the same usable hierarchy.
- [ ] Demo, locks, save failures, provider identity, and disconnected Slack
      are truthful; visual approval is separate from backend readiness.

## Decisions still to test visually

Compare equal rails with wider chat; yellow-on-gray with a graphite wordmark
inset; five-unit with seven-unit lettering; square controls with corner relief.
Test long borrower names, tab overflow, and the Income table at compact widths.
Favor Orion's character and clarity of the working loan.
