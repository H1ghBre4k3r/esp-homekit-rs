# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

ESP32-C6 Matter (HomeKit) smart LED controller using `rs-matter` and `esp-hal`. Implements a Matter-compatible light device with on-off and color control clusters, controllable via Google Home, Alexa, or other Matter controllers.

**Target Hardware:** ESP32-C6 RISC-V microcontroller
**Language:** Rust (no_std embedded)
**Key Framework:** Matter protocol via `rs-matter-embassy`

## Build & Flash Commands

**Build for release:**
```bash
cargo build --release
```

**Build and flash to device:**
```bash
cargo run --release
```

The cargo runner is configured to automatically flash and monitor via `espflash`. The target is `riscv32imac-unknown-none-elf` (RISC-V bare metal).

**Note:** Release builds use `opt-level = "s"` (optimize for size) and LTO. Dev builds also use `opt-level = "s"` because unoptimized Rust is too slow for embedded.

## Architecture

### Core Components

**`src/bin/main.rs`** - Main application entry point
- Initializes ESP32-C6 peripherals (WiFi, BLE, RMT, RNG)
- Creates `EmbassyWifiMatterStack` for Matter protocol handling
- Configures Matter Node with root endpoint + light endpoint (ID=1)
- Runs concurrent commissioning (WiFi + BLE coex mode)
- Demo loop toggles light every 5 seconds

**`src/lib.rs`** - LightController implementation
- `LightController` struct manages on-off state, HSV color (hue, saturation), and LED output
- Implements both `on_off::ClusterHandler` and `color_control::ClusterHandler`
- Controls WS2812/SK6812 smart LED via RMT peripheral through `esp-hal-smartled`
- HSV to RGB conversion with hardware-specific channel swapping (R and G swapped)
- `mk_static!` macro for stack-safe static allocation

**`src/color_control.rs`** - Matter ColorControl cluster definition
- Re-exports from `rs-matter` ColorControl cluster spec via `rs_matter::import!` macro

**`src/nvs.rs`** - Non-Volatile Storage persistence
- `Nvs` struct wraps ESP32 NVS partition for Matter stack state persistence
- Implements `KvBlobStore` trait for `rs-matter-embassy`
- Uses ESP-IDF partition table to locate NVS region
- Simple key-value blob storage for commissioning credentials and device state

### Matter Stack Integration

The Matter stack (`EmbassyWifiMatterStack`) handles:
- WiFi networking via `embassy-net` with DHCP
- BLE commissioning via `esp-wifi` coex mode
- Matter protocol state machine
- Cluster attribute subscriptions and notifications

**Endpoints:**
- Endpoint 0: Root endpoint (system clusters)
- Endpoint 1: Extended Color Light device (0x010C, revision 4)
  - Descriptor cluster
  - OnOff cluster
  - ColorControl cluster (full feature set)

**Cluster Handlers Chain:**
Handlers are chained using `EpClMatcher` to route requests to specific endpoint/cluster combinations. The chain is built with `EmptyHandler` as base, then `.chain()` calls for each cluster.

### Memory Management

- **Heap:** 186KB allocated via `esp-alloc` for WiFi/BLE and x509 certificate parsing
- **Bump allocator:** 15,500 bytes (`BUMP_SIZE`) for Matter stack futures
- Static allocation via `mk_static!` macro (wrapper around `static_cell`) to avoid stack overflow
- Embassy executor with 20KB task arena (`task-arena-size-20480`)

## Development Notes

### Adding New Matter Clusters

1. Define cluster in `src/lib.rs` using `rs_matter::dm::Cluster`
2. Implement `ClusterHandler` trait on `LightController` (or new controller struct)
3. Add cluster to endpoint definition in `NODE` constant in `main.rs`
4. Chain cluster handler in `main.rs` using `EpClMatcher` and `Async(handler)`
5. Call `stack.notify_changed()` after state mutations to trigger Matter subscriptions

### Custom Dependencies

Uses forked versions of Matter libraries:
- `rs-matter-embassy` from https://github.com/H1ghBre4k3r/rs-matter-embassy.git
- `rs-matter` from https://github.com/H1ghBre4k3r/rs-matter.git
- `esp-hal-smartled` from esp-rs community repo

### LED Hardware Quirk

The connected LED has swapped R/G channels. RGB conversion compensates in `update_led()` at src/lib.rs:87.

### Build Configuration

- Nightly toolchain required (specified in `rust-toolchain.toml`)
- `rust-src` component required for `build-std`
- Force frame pointers enabled for backtraces (impacts performance)
- Stack protector enabled for all functions
- Custom linker script `linkall.x` (from `esp-hal`)
- `build.rs` provides helpful linker error messages

### Incomplete Color Control Features

Most `color_control` cluster commands are stubbed with `todo!()`. Only `handle_move_to_hue_and_saturation` is fully implemented. Implementing other commands requires adding animation/transition logic with embassy timers.
