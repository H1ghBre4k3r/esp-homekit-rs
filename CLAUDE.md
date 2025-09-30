# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is an ESP32-C6 embedded Rust project implementing a Matter/HomeKit-compatible device using the `rs-matter` framework. The project demonstrates a color-controllable light endpoint with WiFi connectivity and BLE commissioning.

**Target Hardware**: ESP32-C6 (RISC-V)
**Framework**: `rs-matter-embassy` (embedded async runtime)
**Wireless**: WiFi (via `esp-wifi`) + BLE for Matter commissioning

## Build and Flash Commands

### Building
```bash
cargo build --release
```

### Flashing and Monitoring
```bash
cargo run --release
```

The `.cargo/config.toml` configures `espflash` as the runner, so `cargo run` automatically flashes the ESP32-C6 and opens a serial monitor.

### Manual Flash
```bash
espflash flash --monitor --chip esp32c6 target/riscv32imac-unknown-none-elf/release/esp-homekit
```

## Architecture

### Matter Stack Integration

The project uses `EmbassyWifiMatterStack` to integrate Matter protocol with embassy async runtime. Key components:

- **Stack Initialization** (`src/bin/main.rs:82-91`): Statically allocated Matter stack (~35-50KB) with device descriptors and commissioning info
- **Wireless Driver** (`src/bin/main.rs:132-135`): `EspWifiDriver` wraps ESP32 WiFi+BLE peripherals for concurrent operation
- **Persistence** (`src/nvs.rs`): Custom NVS (Non-Volatile Storage) driver implementing `KvBlobStore` trait for Matter state persistence

### Cluster Implementation

The project implements a **Color Control cluster** (Matter specification 0x010C):

1. **Macro-Generated Cluster** (`src/color_control.rs:1`): Uses `rs_matter::import!` macro to generate cluster types from Matter spec
2. **Handler Trait** (`src/lib.rs:30-240`): `MyController` implements `ClusterHandler` with stub implementations for all color control commands
3. **Cluster Configuration** (`src/lib.rs:31-39`): Configures supported attributes, commands, and features for the cluster

### Memory Management

- **Static Allocation**: The `mk_static!` macro (`src/lib.rs:11-18`) uses `static_cell` to allocate on first use without heap fragmentation
- **Heap**: 186KB heap allocated for WiFi/BLE and X.509 certificate handling
- **Bump Allocator**: 15.5KB bump buffer for Matter stack operations

### Data Flow

1. **Device State** → `Dataver` (data version tracking) → Matter stack notifications
2. **Matter Commands** → `ClusterHandler::handle_*` methods → Device state changes
3. **State Changes** → `stack.notify_changed()` → Subscription updates to Matter controllers

### NVS Storage Implementation

The custom NVS driver (`src/nvs.rs`) implements a simplified key-value store:

- Reads ESP32 partition table to locate NVS partition
- Implements `KvBlobStore` trait for rs-matter persistence
- Uses flash offsets: `(key + 1) * buffer_size` for data, `key` offset for validity flag

**Warning**: The current NVS implementation is basic and may have issues (see commit message "do unholy things"). It lacks proper NVS formatting and wear leveling.

## Key Dependencies

- **rs-matter**: Forked version from `H1ghBre4k3r` (non-standard fork)
- **rs-matter-embassy**: Custom integration layer
- **esp-hal**: Version 1.0.0-rc.0 (pre-release, may have breaking changes)
- **esp-wifi**: WiFi+BLE coexistence support

## Development Notes

- **Toolchain**: Nightly Rust with `rust-src` component required for `build-std`
- **Optimization**: Dev builds use `opt-level = "s"` (size optimization) due to memory constraints
- **Recursion**: `recursion_limit = "256"` set in main.rs for macro expansion
- **Endpoint Structure**: Endpoint 0 is reserved for Matter system clusters; user endpoints start at ID 1
