# 🖱️ Rust Clicker Game

This project is a simple clicker-style game developed using the Rust programming language and the Macroquad game engine. The player clicks on a boss to deal damage and earns points to upgrade their hero.

## 🎮 Game Features

- Clickable boss with increasing difficulty
- Hero can be upgraded to deal more damage
- Each boss defeat increases level and reward
- Upgrade cost increases over time
- Live stats: Boss HP, level, hero damage, and points

## 🚀 Installation & Running

1. Make sure Rust is installed on your system. If not:  
   [https://rustup.rs/](https://rustup.rs/)

2. Add `macroquad` to your `Cargo.toml`:

```toml
[dependencies]
macroquad = "0.3"
