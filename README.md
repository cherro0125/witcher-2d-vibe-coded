# Witcher: Szkoła Dzika

A small Witcher-inspired 2D RPG prototype in Rust, built with [ggez](https://ggez.rs/). You
play Gerard z Rumii, a witcher of the School of the Boar, exploring a world with combat,
dialogue, quests, and alchemy.

"Vibe-coded" — built quickly with heavy AI-assisted pair-programming rather than a slow,
from-scratch design pass, as an experiment in how far that workflow can carry a small game
before it needs a human rewrite.

## Systems

~3,000 lines across self-contained modules:

- `player` / `monsters` / `combat` — stats, turn/action resolution, encounters
- `signs` — Witcher-style magic signs
- `inventory` / `alchemy` — item management and potion crafting
- `world` / `quests` / `dialogue` — exploration state and branching quest/dialogue data
- `rendering` / `ui` — the ggez draw loop and on-screen UI

## Running it

```sh
cargo run
```

## Stack

Rust, [ggez](https://ggez.rs/) (2D game framework), `rand`.
