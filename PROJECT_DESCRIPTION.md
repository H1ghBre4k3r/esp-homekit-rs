# Project Description: ESP32-C6 Matter Smart LED Controller

## Overview

This project implements a Matter-compatible (formerly Project CHIP/HomeKit) smart LED controller using an ESP32-C6 RISC-V microcontroller and Rust. The device presents itself as an **Extended Color Light** (Device Type 0x010C, Revision 4) that can be controlled via any Matter controller such as Google Home, Amazon Alexa, Apple Home, or other Matter-compatible systems.

## Hardware

- **MCU:** ESP32-C6 (RISC-V 32-bit)
- **LED:** WS2812/SK6812 RGB smart LED (single LED) connected to GPIO8
- **Communication:** WiFi (2.4GHz) + BLE (Bluetooth Low Energy for commissioning)
- **Memory:** 
  - 186KB heap allocation for WiFi/BLE stack and x509 parsing
  - 15.5KB bump allocator for Matter stack futures
  - 20KB Embassy task arena
- **Storage:** NVS (Non-Volatile Storage) partition for Matter credentials and state persistence

## Current Architecture

### Core Components

1. **Main Application** (`src/bin/main.rs`)
   - Initializes ESP32-C6 peripherals (WiFi, BLE, RMT, RNG, timers)
   - Creates Matter stack using `EmbassyWifiMatterStack`
   - Configures Node structure with Root endpoint (0) and Light endpoint (1)
   - Runs concurrent WiFi + BLE commissioning (coex mode)
   - Demo loop toggles light state every 5 seconds

2. **LightController** (`src/lib.rs`)
   - Manages device state: on/off, hue, saturation
   - Controls WS2812 LED via RMT peripheral
   - Implements two Matter cluster handlers:
     - `on_off::ClusterHandler` - Basic on/off control
     - `color_control::ClusterHandler` - Color control (HSV)
   - HSV to RGB color conversion
   - Hardware quirk: R/G channels are swapped in LED hardware

3. **ColorControl Cluster** (`src/color_control.rs`)
   - Imports Matter ColorControl cluster specification
   - Uses `rs_matter::import!` macro to generate cluster definition

4. **NVS Storage** (`src/nvs.rs`)
   - Persistent storage implementation using ESP32 NVS partition
   - Implements `KvBlobStore` trait for rs-matter-embassy
   - Stores commissioning credentials, device certificates, and state
   - Auto-detects NVS partition from partition table

### Matter Stack Details

- **Protocol Stack:** rs-matter-embassy (forked version)
- **Transport:** WiFi primary, BLE for commissioning
- **Endpoints:**
  - **Endpoint 0:** Root endpoint with system clusters (hidden, managed by stack)
  - **Endpoint 1:** Extended Color Light device
    - Descriptor cluster (device type metadata)
    - OnOff cluster (basic power control)
    - ColorControl cluster (hue, saturation, and color modes)

- **Cluster Handler Chain:** Uses `EpClMatcher` to route requests to specific endpoint/cluster pairs

## Current Implementation Status

### ✅ Fully Implemented Features

#### OnOff Cluster (0x0006)
- ✅ On command
- ✅ Off command
- ✅ Toggle command
- ✅ OffWithEffect command (simplified)
- ✅ OnWithRecallGlobalScene command (simplified)
- ✅ OnWithTimedOff command (simplified)
- ✅ OnOff attribute (readable)

#### ColorControl Cluster (0x0300) - Partial
- ✅ MoveToHueAndSaturation command (sets hue and saturation immediately)
- ✅ Color mode attributes (returns CurrentXAndCurrentY)
- ✅ Enhanced color mode attribute
- ✅ Color capabilities attribute (reports all features)
- ✅ Number of primaries attribute
- ✅ Options attribute (readable)

### ❌ Missing/Incomplete Features

#### ColorControl Cluster - Commands (19 unimplemented)
1. ❌ `MoveToHue` - Move to specific hue with transition time
2. ❌ `MoveHue` - Continuously move hue up/down at rate
3. ❌ `StepHue` - Step hue by amount at rate
4. ❌ `MoveToSaturation` - Move to specific saturation with transition
5. ❌ `MoveSaturation` - Continuously move saturation at rate
6. ❌ `StepSaturation` - Step saturation by amount at rate
7. ❌ `MoveToColor` - Move to XY color coordinates
8. ❌ `MoveColor` - Continuously move color at rate
9. ❌ `StepColor` - Step color by amount
10. ❌ `MoveToColorTemperature` - Set color temperature in mireds
11. ❌ `EnhancedMoveToHue` - 16-bit hue precision version
12. ❌ `EnhancedMoveHue` - 16-bit continuous hue movement
13. ❌ `EnhancedStepHue` - 16-bit hue stepping
14. ❌ `EnhancedMoveToHueAndSaturation` - 16-bit precision hue+sat
15. ❌ `ColorLoopSet` - Automatic color cycling
16. ❌ `StopMoveStep` - Cancel ongoing transitions
17. ❌ `MoveColorTemperature` - Continuously adjust temperature
18. ❌ `StepColorTemperature` - Step temperature by amount
19. ❌ `SetOptions` - Write to options attribute

#### ColorControl Cluster - Attributes (Missing Reads)
- ❌ `CurrentHue` (0x0000) - Current hue value 0-254
- ❌ `CurrentSaturation` (0x0001) - Current saturation 0-254
- ❌ `RemainingTime` (0x0002) - Time left in transition
- ❌ `CurrentX` (0x0003) - Current X coordinate (CIE 1931)
- ❌ `CurrentY` (0x0004) - Current Y coordinate
- ❌ `ColorTemperatureMireds` (0x0007) - Current color temp
- ❌ `EnhancedCurrentHue` (0x4000) - 16-bit current hue
- ❌ Various optional attributes (primaries, white point, color points, etc.)

**Note:** All commands currently use `todo!()` macro, causing panic if invoked.

### Critical Issues

#### 1. **Test Credentials in Production**
The device currently uses **test/demo credentials** from rs-matter:
```rust
use rs_matter_embassy::matter::dm::devices::test::{
    TEST_DEV_ATT,    // Test Device Attestation
    TEST_DEV_COMM,   // Test Device Commissioning data
    TEST_DEV_DET     // Test Device Details
};
```

**Problems:**
- All devices share the same vendor ID, product ID, and certificates
- Not suitable for production or commercial deployment
- Cannot uniquely identify devices
- Security vulnerability (shared private keys)
- Won't pass Matter certification

#### 2. **Missing Transition Support**
Most color commands in Matter spec require smooth transitions over time:
- No transition time implementation
- No animation/interpolation for color changes
- Commands that specify `transitionTime` parameter are ignored
- This breaks expected user experience (colors snap instantly)

#### 3. **Incomplete Color Space Support**
- Only HSV (Hue-Saturation-Value) is implemented
- Missing XY color coordinate support (CIE 1931 color space)
- Missing color temperature support (warm/cool white)
- ColorMode reports XY but doesn't actually support it

#### 4. **No Persistent State for Color**
- OnOff state persists through Matter stack
- Hue and saturation values are NOT persisted
- Device loses color settings on reboot
- Should implement startup color attributes

#### 5. **Hardware-Specific LED Quirk**
- R and G channels are swapped at hardware level
- Workaround in code (line 87 of lib.rs)
- Not documented in hardware specs
- Could confuse future maintainers

## Matter Specification Compliance

### Device Type: Extended Color Light (0x010C)

According to Matter 1.2 Application Cluster Specification:

**Required Clusters:**
- ✅ Descriptor (implemented by rs-matter)
- ✅ OnOff (fully implemented)
- ⚠️ ColorControl (partially implemented)

**Required ColorControl Features for Extended Color Light:**
- ⚠️ Hue/Saturation (HS) - Partial
- ❌ XY coordinates - Not implemented
- ❌ Color Temperature (CT) - Not implemented
- ❌ Enhanced Hue - Not implemented
- ❌ Color Loop - Not implemented

### Specification Requirements Not Met

1. **Attribute Reporting:** Many required attributes return dummy values or aren't readable
2. **Transition Timing:** All transitions should honor `transitionTime` parameter
3. **Options Mask/Override:** Not properly handled in commands
4. **Color Mode Management:** Should change based on last command used
5. **Boundary Checking:** Min/max values for color temp, coordinates not enforced
6. **Startup Behavior:** Should recall last color state or use startup attributes

## Technical Stack

### Dependencies
- **esp-hal** 1.0.0-rc.0 - Hardware abstraction layer
- **esp-wifi** 0.15.0 - WiFi + BLE stack (coex mode)
- **embassy-executor** 0.7.0 - Async runtime
- **embassy-net** 0.7.0 - Network stack (DHCP, TCP, UDP)
- **rs-matter** (forked) - Matter protocol implementation
- **rs-matter-embassy** (forked) - Embassy integration for Matter
- **esp-hal-smartled** - WS2812 LED driver via RMT
- **smart-leds** - RGB color definitions

### Build Configuration
- **Target:** riscv32imac-unknown-none-elf (bare metal RISC-V)
- **Optimization:** Size-optimized ("s") for both dev and release
- **LTO:** Fat LTO enabled for release
- **Memory:** No heap allocation except for WiFi/BLE and x509
- **Toolchain:** Nightly Rust (rust-toolchain.toml)

## Memory Constraints

The ESP32-C6 has limited memory:
- **Static/BSS:** Very limited (no exact number, but triggering overflow issues)
- **Heap:** 186KB explicitly allocated
- **Stack:** Embassy task arena 20KB
- **Bump allocator:** 15.5KB for Matter futures

**Strategy:**
- Use `static_cell` for large allocations
- Minimize stack usage in async functions
- `mk_static!` macro for safe static allocation
- Avoid dynamic allocation where possible

## Known Limitations

1. **Single LED:** Only controls one RGB LED (easily extensible to strips)
2. **No Groups:** Matter group/scene functionality not implemented
3. **No Bindings:** Cannot bind to other Matter devices
4. **Demo Loop:** Automatic 5-second toggle (for testing only)
5. **Forked Dependencies:** Uses custom rs-matter forks, not upstream
6. **No OTA:** No over-the-air firmware update support
7. **Fixed Features:** Reports all ColorControl features but implements few

## Security Considerations

1. **Test Certificates:** Using shared test certificates (CRITICAL)
2. **No Device-Specific Keys:** Every device has same identity
3. **No Vendor Certification:** Cannot pass Matter certification
4. **Shared Secrets:** Commissioning data is not unique
5. **NVS Security:** No encryption on persistent storage

## Future Extensibility

The codebase is well-structured for extensions:
- Adding more endpoints (additional lights, sensors, etc.)
- Implementing additional clusters (LevelControl, Groups, Scenes)
- Supporting LED strips (change from single LED to array)
- Adding physical buttons (GPIO interrupts)
- Power monitoring/reporting
- Environmental sensors integration

## Code Quality Notes

**Strengths:**
- Clean separation of concerns (controller, storage, main)
- Good use of Rust type system and traits
- Proper async/await with Embassy
- Static allocation for memory safety
- Logging for debugging

**Areas for Improvement:**
- Many `todo!()` placeholders
- Limited error handling (many `.unwrap()` calls)
- Hardcoded GPIO pin numbers
- Magic numbers (heap size, bump size, colors)
- Missing documentation comments
- Test credentials hardcoded
- No configuration system

## Project Maturity

**Current Stage:** Early prototype / proof-of-concept

**Production Readiness:** ❌ Not ready
- Missing critical Matter spec compliance
- Using test credentials
- Incomplete feature set
- Limited error handling
- No testing infrastructure

**Required for Production:**
1. Unique device credentials per unit
2. Complete ColorControl implementation
3. Proper error handling throughout
4. Commissioning flow testing
5. Multi-controller compatibility testing
6. Matter certification process
7. OTA update mechanism
8. Factory reset functionality
9. Configuration storage
10. Manufacturing test procedures
