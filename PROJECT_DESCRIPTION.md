# ESP-HomeKit Project Description

## Overview

ESP32-C6 Matter-compatible smart LED controller implementing an Extended Color Light device using Rust (`no_std` embedded) with `rs-matter` and `esp-hal` frameworks.

**Target Hardware:** ESP32-C6 RISC-V microcontroller
**Protocol:** Matter (Thread/WiFi + BLE commissioning)
**Current Device Type:** 0x010C (claimed as Extended Color Light, but this is actually Color Temperature Light ID)
**Correct Device Type:** 0x010D (Extended Color Light per Matter 1.2 spec)

## Project Structure

### Core Files

- **`src/bin/main.rs`** (222 lines) - Main application entry point
  - Initializes ESP32-C6 peripherals (WiFi, BLE, RMT for LED control, RNG)
  - Creates `EmbassyWifiMatterStack` for Matter protocol
  - Configures Matter Node with root endpoint (0) + light endpoint (1)
  - Runs concurrent WiFi+BLE commissioning (coex mode)
  - Demo loop: toggles light every 5 seconds to demonstrate state changes

- **`src/lib.rs`** (455 lines) - LightController implementation
  - Manages on/off state, HSV color (hue 0-255, saturation 0-255)
  - Implements `on_off::ClusterHandler` (fully functional)
  - Implements `color_control::ClusterHandler` (mostly stubbed)
  - Controls WS2812/SK6812 LED via RMT peripheral through `esp-hal-smartled`
  - HSV→RGB conversion with hardware-specific channel swap (R↔G at line 88)

- **`src/color_control.rs`** (4 lines) - ColorControl cluster import
  - Re-exports Matter ColorControl cluster via `rs_matter::import!` macro

- **`src/nvs.rs`** (29 lines) - Non-Volatile Storage persistence
  - Wraps ESP32 NVS partition for Matter stack state
  - Implements `KvBlobStore` trait for `rs-matter-embassy`
  - Stores commissioning credentials and device state

- **`src/credentials.rs`** (32 lines) - Dynamic credential generation
  - Derives discriminator (12-bit, 0-4095) from device MAC address
  - Derives password (1-99,999,999) from MAC address
  - Generates unique credentials per device without hardcoding

### Memory Configuration

- **Heap:** 186 KB (for WiFi/BLE stack and x509 cert parsing)
- **Bump Allocator:** 20,000 bytes for Matter stack futures
- **Embassy Task Arena:** 20 KB (`task-arena-size-20480`)
- **Optimization:** `opt-level = "s"` (size optimization) even for dev builds (unoptimized Rust too slow for embedded)

## Currently Implemented Features

### ✅ Functional Clusters

#### 1. OnOff Cluster (0x0006) - **COMPLETE**
- ✅ `On` command (src/lib.rs:395)
- ✅ `Off` command (src/lib.rs:405)
- ✅ `Toggle` command (src/lib.rs:415)
- ✅ `OffWithEffect` command (src/lib.rs:426, simplified to regular off)
- ✅ `OnWithRecallGlobalScene` command (src/lib.rs:436, simplified to regular on)
- ✅ `OnWithTimedOff` command (src/lib.rs:445, simplified to regular on)
- ✅ `OnOff` attribute read (src/lib.rs:388)
- ✅ State change notifications via `dataver.changed()`

#### 2. ColorControl Cluster (0x0300) - **PARTIAL**
- ✅ `MoveToHueAndSaturation` command (src/lib.rs:247) - **ONLY FULLY IMPLEMENTED COMMAND**
  - Sets hue and saturation instantly (no transition animation)
  - Updates LED immediately
  - Triggers data version change for subscriptions
- ✅ Basic attribute reads:
  - `color_mode()` (src/lib.rs:152) - hardcoded to `CurrentXAndCurrentY` (incorrect, should reflect actual mode)
  - `enhanced_color_mode()` (src/lib.rs:170) - same as color_mode
  - `options()` (src/lib.rs:159) - returns 1
  - `number_of_primaries()` (src/lib.rs:163) - returns 1
  - `color_capabilities()` (src/lib.rs:177) - returns all features bits
- ✅ Internal state: `hue` and `saturation` (u8, 0-255 range)
- ✅ HSV→RGB color conversion (src/lib.rs:108)

#### 3. Descriptor Cluster (0x001D)
- ✅ Provided by `rs-matter` via `desc::DescHandler` (main.rs:146)

### ❌ Missing Required Clusters (Per Matter Spec for Extended Color Light 0x010D)

1. **Identify Cluster (0x0003)** - NOT IMPLEMENTED
   - Required for device identification (blink/alert functionality)
   - Common across most Matter device types

2. **Groups Cluster (0x0004)** - NOT IMPLEMENTED
   - Required for group addressing
   - Prerequisite for Scenes cluster

3. **Scenes Cluster (0x0005)** - NOT IMPLEMENTED
   - Required for scene management
   - Stores/recalls attribute states per scene

4. **LevelControl Cluster (0x0008)** - NOT IMPLEMENTED
   - Required for brightness control
   - Missing attributes: `CurrentLevel`, `RemainingTime`, `MinLevel`, `MaxLevel`
   - Missing commands: `MoveToLevel`, `Move`, `Step`, `Stop`, `MoveToLevelWithOnOff`, etc.

### ⚠️ Incomplete ColorControl Implementation

#### Missing Required Features
Per Matter 1.2 spec, Extended Color Light requires:
- ❌ **XY color mode** (MANDATORY) - not implemented, only HS mode exists
- ❌ **Color Temperature (CT) mode** (MANDATORY) - not implemented

Current implementation only supports **HS (Hue/Saturation)** which is OPTIONAL.

#### Missing Required Attributes
- ❌ `CurrentX` (0x0003, uint16) - for XY color mode
- ❌ `CurrentY` (0x0004, uint16) - for XY color mode
- ❌ `ColorTemperatureMireds` (0x0007, uint16) - for CT mode
- ❌ `RemainingTime` (0x0002, uint16) - transition time remaining (MANDATORY reporting)
- ❌ Proper `ColorMode` implementation - currently hardcoded incorrectly

#### Stubbed Commands (15+ with `todo!()`)
All of these log the command but panic with `todo!()`:

**Hue/Saturation Commands:**
- ❌ `MoveToHue` (src/lib.rs:193)
- ❌ `MoveHue` (src/lib.rs:202)
- ❌ `StepHue` (src/lib.rs:211)
- ❌ `MoveToSaturation` (src/lib.rs:220)
- ❌ `MoveSaturation` (src/lib.rs:229)
- ❌ `StepSaturation` (src/lib.rs:238)

**XY Color Commands:**
- ❌ `MoveToColor` (src/lib.rs:268)
- ❌ `MoveColor` (src/lib.rs:277)
- ❌ `StepColor` (src/lib.rs:286)

**Color Temperature Commands:**
- ❌ `MoveToColorTemperature` (src/lib.rs:295)
- ❌ `MoveColorTemperature` (src/lib.rs:358)
- ❌ `StepColorTemperature` (src/lib.rs:367)

**Enhanced Hue Commands:**
- ❌ `EnhancedMoveToHue` (src/lib.rs:304)
- ❌ `EnhancedMoveHue` (src/lib.rs:313)
- ❌ `EnhancedStepHue` (src/lib.rs:322)
- ❌ `EnhancedMoveToHueAndSaturation` (src/lib.rs:331)

**Other Commands:**
- ❌ `ColorLoopSet` (src/lib.rs:340)
- ❌ `StopMoveStep` (src/lib.rs:349)

**Writable Attribute:**
- ❌ `set_options()` (src/lib.rs:184)

## Known Issues & Problems

### 1. **Device Type Mismatch** (Critical)
- **Current:** 0x010C (Color Temperature Light)
- **Claimed:** "Extended Color Light" in comments/docs
- **Required:** 0x010D (Extended Color Light per Matter 1.2)
- **Impact:** May cause certification/interoperability issues

### 2. **Missing Mandatory Clusters** (Critical)
Violates Matter spec - Extended Color Light MUST implement:
- Identify
- Groups
- Scenes
- LevelControl (brightness control)

Without these, the device is non-compliant and may be rejected by controllers.

### 3. **Missing Mandatory ColorControl Features** (Critical)
Extended Color Light requires **both** XY and CT color modes:
- No XY color space support (only HSV internally)
- No color temperature support
- No conversion between color modes

### 4. **Transition/Animation Not Implemented** (High Priority)
All color commands should support transition times (smooth fades), but currently:
- Only instant state changes implemented
- `RemainingTime` attribute not tracked
- No timer-based gradual transitions
- Controllers expect smooth color changes

### 5. **ColorMode Attribute Incorrect** (Medium Priority)
- Hardcoded to return `CurrentXAndCurrentY` (src/lib.rs:156)
- Should dynamically reflect actual current mode (HS, XY, or CT)
- Should update when mode-specific commands are used

### 6. **State Persistence Incomplete** (Low Priority)
- NVS storage exists but only used by Matter stack internals
- Color state (hue, saturation) not persisted across reboots
- OnOff state not persisted

### 7. **Hardware-Specific Quirk** (Documented)
- LED has swapped R/G channels, compensated in code (src/lib.rs:87)
- RGB8 { r: g, g: r, b } - this is hardware-specific, not ideal for portability

### 8. **Commissioning Stability** (Resolved)
- Added 100ms delay after WiFi init (main.rs:102) to fix "BleHost(Hci(Invalid HCI Command Parameters))" errors
- Seems stable now

## Dependencies

### Custom Forks
- `rs-matter-embassy` from H1ghBre4k3r fork (rev 5099a13c)
- `rs-matter` from H1ghBre4k3r fork (rev 3faa3b4)
- `esp-hal-smartled` from esp-rs community repo (rev 339e8fc8)

### Key Dependencies
- `esp-hal` = 1.0.0-rc.0 (ESP32-C6 HAL)
- `esp-wifi` = 0.15.0 (WiFi + BLE coex)
- `embassy-executor` = 0.7.0 (async runtime)
- `embassy-net` = 0.7.0 (networking stack)
- `smart-leds` = 0.4.0 (LED trait abstractions)

## Build & Flash

**Build:**
```bash
cargo build --release
```

**Flash & Monitor:**
```bash
cargo run --release
```

**Target:** `riscv32imac-unknown-none-elf` (RISC-V bare metal)

**Toolchain:** Nightly Rust (specified in `rust-toolchain.toml`)

## Summary

This is a **proof-of-concept** Matter light controller with:
- ✅ Working WiFi+BLE commissioning
- ✅ Basic on/off control
- ✅ Basic hue/saturation color control
- ✅ Smart LED hardware integration
- ⚠️ **NOT Matter spec-compliant** for Extended Color Light
- ❌ Missing 4 required clusters
- ❌ Missing 2 required ColorControl features
- ❌ 15+ unimplemented commands

**Next Steps:** See ROADMAP.md for implementation plan to achieve spec compliance.
