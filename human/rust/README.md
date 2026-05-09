A Rust client to play StarCraft II against a bot on [https://match.superskill.me](https://match.superskill.me/)

## Prerequisites

- Windows - The client is tested to run on Windows
- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- [StarCraft II](https://starcraft2.com) installed at `C:\Program Files (x86)\StarCraft II`
- StarCraft II version Base75689 - Start a replay from any match from AI Arena to get this version

## Run

```sh
cargo run --release
```

The program will launch StarCraft II, download map `LeyLinesAIE_v3`, and join a game for you to play as the race you selected.

## Configuration

The application will ask for SC2 path and the race of the player.
SC2 version is fixed to Base75689.
SC2 map is fixed to LeyLinesAIE_v3.
Other configuration values are compiled into `src/config.rs`:

| Setting | Default |
|---|---|
| SC2 path | `C:\Program Files (x86)\StarCraft II` |
| SC2 port | `10001` |
| Player name | `Human` |

Edit `src/config.rs` and rebuild to change these values.
