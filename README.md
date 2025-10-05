<div align="center">

# waybar-autohide

Minimal Rust program to auto-show/hide waybar based on cursor position in Hyprland.

</div>

## Build

```bash
cargo build --release
```

## Install

```bash
cp target/release/waybar-autohide ~/.local/bin/
```

## Setup

Add to `~/.config/hypr/hyprland.conf`

```conf
exec-once = waybar
exec-once = waybar-autohide
```

## Requirements

- Hyprland
- Waybar
- Rust (for building)
