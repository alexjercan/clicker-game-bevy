# Expand the core gameplay and progression loop

- STATUS: OPEN
- PRIORITY: 70
- TAGS: gameplay, iteration-3

## User facts

- The current click-to-gain-XP loop is not interesting enough.
- Future gameplay can include a buy menu, buildings, automatic farming, click-tool radius upgrades, and additional worlds.
- Keep this work separate from the current automation and architecture task.

## Agent findings

- Current progression has one XP resource, one increasing threshold, and skill points used only to place random tiles.
- Tile kinds are visual/random but do not yet create distinct production choices.
- There is no currency economy, purchase catalog, building ownership, passive production, tool upgrade model, world progression, save model, or offline policy.
- Adding all proposed systems at once would make balancing and verification unclear. The first implementation needs one approved vertical slice and explicit numeric defaults.

## Decisions needed

- Choose the primary economy resource and the purpose of XP and skill points.
- Choose the first purchasable building and which tile kinds can host it.
- Define active-click output, passive production rate, costs, scaling curves, and caps.
- Define whether click radius means axial area, tool area, multi-tile harvesting, or another mechanic.
- Define world unlock conditions, whether worlds coexist, and what persists between worlds.
- Define save, offline progress, and failure policies before persistent progression is added.

## Delivery

- Write a small gameplay loop specification with explicit resources, player actions, sinks, unlocks, and progression goals.
- Deliver one vertical slice: earn a resource, open a buy menu, buy one building or upgrade, observe automatic production, and see the result in the HUD.
- Keep tile ownership in `clicker_tile`; put economy and progression rules in `clicker_gameplay`; keep presentation in `clicker_ui`.
- Add public gameplay actions and observable snapshots for every automated interaction.
- Add click-tool range only after its coordinate rule and input feedback are approved.
- Add additional worlds only after ownership, transition, persistence, RNG, and camera behavior are explicit.
- Avoid speculative compatibility layers and unused generalized economy frameworks.

## Verification

- Unit-test costs, production, upgrade curves, caps, and unlock rules.
- Use focused Bevy tests for purchases, insufficient funds, placement restrictions, production ticks, and world transitions.
- Add bounded autopilot examples for the approved vertical slice and deterministic seed behavior.
- Inspect rendered buy-menu, range feedback, building placement, and world-transition flows separately.
- Verify native and wasm builds and any save migration policy introduced by the task.

## Done when

- The approved loop has a meaningful active choice and a meaningful passive reward.
- Purchase and production rules are deterministic and visible through public snapshots.
- Automation can complete the loop through public gameplay actions.
- UI and rendered evidence clearly communicate costs, ownership, output, and unavailable actions.
