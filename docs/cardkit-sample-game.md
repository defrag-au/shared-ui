# Black Flag – Leviathan Hunt  
## Game Design & Mechanics Summary (Working Draft)

---

## 1. Design Goal & Positioning

**Leviathan Hunt** is a fast-paced PvPvE (and solo-capable) game designed to complement the existing Black Flag ecosystem.

It focuses on short, high-tension sessions (5–15 minutes) that generate memorable moments through:
- pressure
- loss of control
- dramatic reversals

The game is **pirate-first** in tone and explicitly avoids:
- long-form economic optimisation
- ship combat (already covered elsewhere)
- slow engine-building loops

Leviathan Hunt is intended as a **high-energy, replayable side pillar** of the ecosystem rather than a replacement for existing gameplay.

---

## 2. Core Fantasy

Pirates do not directly fight the sea monster.

Instead, they **deploy dangerous autonomous forces**—machines, rituals, or constructs—that relentlessly attack the monster every round.

These forces:
- cannot be told to stop
- cannot be perfectly controlled
- create escalating pressure

The player’s role is to:
- deploy
- sabotage
- redirect
- overload
- and sometimes destroy these forces

The skill of the game is **controlling damage**, not dealing it.

---

## 3. Entry Requirement & Deck Minting

### Pirate Requirement
- **One Black Flag pirate NFT is required to play**
- Without a pirate, the player cannot find or engage sea monsters

The pirate anchors the deck and defines:
- maximum active engines
- baseline resilience
- a once-per-hunt override ability

### Partner Assets
Assets from outside Black Flag:
- do **not** grant raw power
- do **not** replace the pirate

Instead, partner assets:
- unlock alternative engine types
- unlock alternative control effects
- change volatility, flavour, and failure modes

### Deck Minting
- The player’s wallet is profiled at game start
- A predefined deck is assembled (not freeform deckbuilding)
- Deck identity is stable for the duration of the hunt

---

## 4. Card Types

The game uses **only two card types**.

### Engines
- Persistent entities deployed onto the table
- Automatically deal damage to the monster every round
- Cannot choose not to attack
- Create unavoidable forward momentum
- May be destroyed, overloaded, or sabotaged

Examples:
- Harpoon Batteries
- Clockwork Rams
- Cursed Net Arrays
- Arcane Depth Charges

### Control Cards
- Targeted interventions that manipulate engines
- Can target:
  - your own engines
  - opponent engines
  - (occasionally) monster state
- Do **not** deal damage directly

Common effects:
- throttle
- overload
- redirect
- jam
- destroy

There is **no separate failsafe category** — self-sabotage is simply control applied to friendly engines.

---

## 5. Core Tension: Damage Control

Damage is inevitable.

Engines guarantee that the monster will die.  
The game is not about *how much* damage you deal, but **when damage happens and who benefits from it**.

Players must often:
- delay their own progress
- destroy their own engines
- force inefficient outcomes

Preventing an opponent from winning is often more important than progressing yourself.

---

## 6. Win Condition

There is **only one win condition**:

> **Deal the killing blow to the monster.**

- Total damage dealt does not matter
- Contribution does not matter
- Only the final point of damage counts

This creates:
- rivalry without direct combat
- greed-driven mistakes
- memorable last-moment reversals

---

## 7. Hand Pressure & Forced Play

Hand pressure is a core mechanic.

- **Draw rate:** 2 cards per turn
- **Maximum hand size:** 5 cards
- **No discard action**

If a player exceeds the hand limit:
- they must **immediately play cards** until they are back within the limit

Consequences:
- hoarding is impossible
- drawing cards is dangerous
- engines and control cards will be forced into play
- loss of control is systemic, not optional

---

## 8. Turn Structure

Each turn follows this structure:

1. Draw 2 cards
2. Play cards in any order:
   - deploy engines
   - play control cards
3. If hand size exceeds 5 at any point, immediately play cards until compliant
4. End of round:
   - all engines attack automatically
   - damage is applied
   - the monster reacts

There are no action points, phases, or attack declarations.

Attacking is ambient and unavoidable.

---

## 9. Monster Role

The monster is:
- a shared threat
- a countdown clock
- an active participant

Monster behaviour:
- reacts to cumulative damage
- punishes over-deployment
- destroys engines
- destabilises the board state

The monster is always more powerful than any individual player and exists to prevent stable or solved play.

---

## 10. Solo & PvPvE Play

### Solo
- The player races against escalating damage
- The challenge is killing the monster **without losing control of timing**

### PvPvE
- Multiple pirates deploy engines simultaneously
- Players compete indirectly through timing and sabotage
- No direct player elimination
- Rivalry emerges naturally from shared pressure

---

## 11. Design Pillars

- Loss of control is **systemic**, not random
- Every turn escalates
- Self-sabotage is often correct
- Skill is expressed through timing, restraint, and nerve
- Games are short, intense, and story-generating
- The sea is always more dangerous than the players

---

## 12. Current Prototype Focus

The immediate prototype scope is intentionally small:

- 3 Engine cards
- 3 Control cards
- 1 Sea Monster with a simple reaction table

Goals:
- validate hand pressure
- validate forced play
- validate kill-shot timing
- observe emergent tension

Out of scope for this phase:
- numerical balance
- content breadth
- progression systems
- UI polish

---

## Status

This document represents the **current design lock** for Leviathan Hunt.  
Future iterations should preserve these core dynamics unless intentionally redesigning the game’s identity.
