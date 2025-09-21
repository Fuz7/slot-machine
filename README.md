# 🎰 Slot Machine Game (Rust)

A simple slot machine game written in **Rust** for a university project.  
The core mechanic is reaching a **goal** within a set number of rolls. Each time the goal is reached, the next goal **scales up** in difficulty.

---

## 📖 Overview
- **Language:** Rust  
- **Type:** Small university project  
- **Genre:** Slot machine / casual  
- **Mechanic:** Roll reels → reach goal → scale next goal  

---

## 🚀 How to Run
```bash
# Clone repo
git clone https://github.com/Fuz7/slot-machine.git
cd slot-machine

# Run with Cargo
cargo run
```
---
## 🛠️ Features (Current)

- Roll reels with RNG

- Track progress toward a goal

- Goal scaling after each success

- Basic terminal output

---
## 🏗️ Planned / Potential Features

#### Gameplay

- Adjustable difficulty levels

- Payouts and multipliers

- Player credits (betting system)

- Different reel symbols (🍒, 🔔, 7️⃣, etc.)

- Bonus rounds or jackpots

#### Systems

- Configurable game settings (config.json or Rust struct)

- Modular scaling system (linear, exponential, custom)

- Save/load progress

#### UI

- Pause menu

- Main menu

- Game over screen

- Graphical interface (instead of terminal output)

#### Technical

Unit tests for core mechanics

- Benchmarking RNG performance

- Documentation for modules (entities/, core/, scenes/)

---
## 📂 Project Structure

```arduino
# draft
src/
├── main.rs             # entry point
├── core/               # game loop, rules, config
│   ├── config.rs
│   └── scaling.rs
├── entities/           # game objects
│   └── slot_machine.rs
└── scenes/             # game states/screens
    ├── gameplay.rs
    ├── menu.rs
    └── pause.rs
```

---
## 🎓 About

This project was created as part of a university coursework assignment to explore:

- Rust programming fundamentals

- Modular code organization

- Game loops and state management

- Using external crates (e.g., rand, serde)


---
## License 
*"for educational use only"*