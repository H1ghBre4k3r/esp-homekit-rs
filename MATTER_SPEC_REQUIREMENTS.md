# Matter Specification Requirements for Extended Color Light

This document outlines the Matter 1.2 specification requirements for an Extended Color Light device (Device Type 0x010C). It serves as a reference for implementing full specification compliance.

## Reference Documents

- **Matter 1.2 Application Cluster Specification** (October 2023)
- **Device Library Specification** - Chapter on Lighting Devices
- **Core Specification** - Device Attestation, Commissioning

## Device Type: Extended Color Light (0x010C)

**Revision:** 4  
**Classification:** Simple (endpoint-level device)  
**Scope:** Endpoint

### Device Type Purpose

An Extended Color Light is a lighting device that supports full color control including:
- On/Off switching
- Hue and Saturation (HSV color model)
- XY color coordinates (CIE 1931 color space)
- Color temperature (warm white to cool white)
- Enhanced precision for hue values
- Automated color looping effects

## Required Clusters

### 1. Descriptor Cluster (0x001D)
**Requirement:** MANDATORY  
**Purpose:** Identifies device type and supported clusters

**Implementation Status:** ✅ Handled by rs-matter stack automatically

### 2. OnOff Cluster (0x0006)
**Requirement:** MANDATORY  
**Purpose:** Basic power on/off control

#### Required Features
- None (base functionality only)

#### Required Attributes
| Attribute | ID | Type | Access | Requirement |
|-----------|-----|------|--------|-------------|
| OnOff | 0x0000 | boolean | R | MANDATORY |

#### Required Commands
| Command | ID | Requirement |
|---------|-----|-------------|
| Off | 0x00 | MANDATORY |
| On | 0x01 | MANDATORY |
| Toggle | 0x02 | MANDATORY |

**Implementation Status:** ✅ Fully implemented

### 3. ColorControl Cluster (0x0300)
**Requirement:** MANDATORY  
**Purpose:** Full color control functionality

This is the primary cluster requiring implementation work.

## ColorControl Cluster Detailed Requirements

### Feature Support

Extended Color Light MUST support one or more of these feature sets:

#### Feature: Hue/Saturation (HS)
**Feature Bit:** 0x01  
**Status:** ⚠️ Partially Implemented

**Mandatory if supported:**
- CurrentHue attribute (0x0000)
- CurrentSaturation attribute (0x0001)
- MoveToHue command (0x00)
- MoveHue command (0x01)
- StepHue command (0x02)
- MoveToSaturation command (0x03)
- MoveSaturation command (0x04)
- StepSaturation command (0x05)
- MoveToHueAndSaturation command (0x06)

#### Feature: Enhanced Hue (EHUE)
**Feature Bit:** 0x02  
**Status:** ❌ Not Implemented

**Requires:** Hue/Saturation feature  
**Mandatory if supported:**
- EnhancedCurrentHue attribute (0x4000)
- EnhancedMoveToHue command (0x40)
- EnhancedMoveHue command (0x41)
- EnhancedStepHue command (0x42)
- EnhancedMoveToHueAndSaturation command (0x43)

#### Feature: Color Loop (CL)
**Feature Bit:** 0x04  
**Status:** ❌ Not Implemented

**Requires:** Enhanced Hue feature  
**Mandatory if supported:**
- ColorLoopActive attribute (0x4002)
- ColorLoopDirection attribute (0x4003)
- ColorLoopTime attribute (0x4004)
- ColorLoopStartEnhancedHue attribute (0x4005)
- ColorLoopStoredEnhancedHue attribute (0x4006)
- ColorLoopSet command (0x44)
- StopMoveStep command (0x47)

#### Feature: XY Color (XY)
**Feature Bit:** 0x08  
**Status:** ❌ Not Implemented

**Mandatory if supported:**
- CurrentX attribute (0x0003)
- CurrentY attribute (0x0004)
- MoveToColor command (0x07)
- MoveColor command (0x08)
- StepColor command (0x09)

#### Feature: Color Temperature (CT)
**Feature Bit:** 0x10  
**Status:** ❌ Not Implemented

**Mandatory if supported:**
- ColorTemperatureMireds attribute (0x0007)
- ColorTempPhysicalMinMireds attribute (0x400B)
- ColorTempPhysicalMaxMireds attribute (0x400C)
- MoveToColorTemperature command (0x0A)
- MoveColorTemperature command (0x4B)
- StepColorTemperature command (0x4C)

### Always Mandatory Attributes

These attributes are required regardless of feature support:

| Attribute | ID | Type | Access | Default | Description |
|-----------|-----|------|--------|---------|-------------|
| ColorMode | 0x0008 | enum8 | R | 0x01 | Current color mode (HS=0, XY=1, CT=2) |
| Options | 0x000F | bitmap8 | RW | 0x00 | Behavior options |
| NumberOfPrimaries | 0x0010 | uint8/null | R | null | Number of color primaries |
| EnhancedColorMode | 0x4001 | enum8 | R | 0x01 | Enhanced color mode (includes EHUE=3) |
| ColorCapabilities | 0x400A | bitmap16 | R | - | Bitmap of supported features |

**Current Implementation Issues:**
- ColorMode returns XY but device doesn't support XY
- Options is readable but not writable
- Missing getter methods for several attributes

### Command Requirements

#### 1. MoveToHue (0x00)
**Status:** ❌ Not Implemented (todo!)

**Parameters:**
- Hue (uint8): Target hue 0-254
- Direction (enum8): Shortest/Longest/Up/Down path
- TransitionTime (uint16): Time in 1/10 seconds
- OptionsMask (bitmap8): Execution options
- OptionsOverride (bitmap8): Option overrides

**Behavior:**
- Transition hue from current to target over specified time
- Respect direction parameter for path around color wheel
- Honor OptionsMask/OptionsOverride for ExecuteIfOff behavior
- Update CurrentHue during transition
- Update RemainingTime during transition
- Set ColorMode to 0 (Hue/Saturation)

#### 2. MoveHue (0x01)
**Status:** ❌ Not Implemented (todo!)

**Parameters:**
- MoveMode (enum8): Stop=0, Up=1, Down=3
- Rate (uint8): Hue units per second

**Behavior:**
- Continuously change hue at specified rate
- Stop when reaching 0 or 254 (or wrap around)
- Can be stopped with StopMoveStep command or MoveMode=Stop

#### 3. StepHue (0x02)
**Status:** ❌ Not Implemented (todo!)

**Parameters:**
- StepMode (enum8): Up=1, Down=3
- StepSize (uint8): Size of hue change
- TransitionTime (uint8): Time in 1/10 seconds
- OptionsMask, OptionsOverride

**Behavior:**
- Change hue by StepSize in specified direction
- Transition over TransitionTime
- Clamp at 0/254 or wrap

#### 4. MoveToSaturation (0x03)
**Status:** ❌ Not Implemented (todo!)

**Parameters:**
- Saturation (uint8): Target saturation 0-254
- TransitionTime (uint16): Time in 1/10 seconds
- OptionsMask, OptionsOverride

**Behavior:**
- Transition saturation from current to target
- Update CurrentSaturation during transition

#### 5. MoveSaturation (0x04)
**Status:** ❌ Not Implemented (todo!)

**Parameters:**
- MoveMode (enum8): Stop/Up/Down
- Rate (uint8): Saturation units per second

**Behavior:**
- Continuously adjust saturation
- Clamp at 0-254 range

#### 6. StepSaturation (0x05)
**Status:** ❌ Not Implemented (todo!)

**Parameters:**
- StepMode, StepSize, TransitionTime
- OptionsMask, OptionsOverride

**Behavior:**
- Step saturation by amount over time

#### 7. MoveToHueAndSaturation (0x06)
**Status:** ✅ Implemented (but without transition)

**Parameters:**
- Hue (uint8)
- Saturation (uint8)
- TransitionTime (uint16)
- OptionsMask, OptionsOverride

**Current Implementation:**
- Sets hue and saturation immediately
- Ignores TransitionTime parameter
- Updates LED output
- Notifies Matter stack of change

**Missing:**
- Transition/animation support
- OptionsMask/OptionsOverride handling
- RemainingTime updates

#### 8-10. XY Color Commands
**Status:** ❌ Not Implemented

- MoveToColor (0x07)
- MoveColor (0x08)
- StepColor (0x09)

**Requirements:**
- Support CIE 1931 XY color space
- Convert XY to RGB for LED output
- Update CurrentX, CurrentY attributes
- Set ColorMode to 1 (XY)

#### 11-13. Color Temperature Commands
**Status:** ❌ Not Implemented

- MoveToColorTemperature (0x0A)
- MoveColorTemperature (0x4B)
- StepColorTemperature (0x4C)

**Requirements:**
- Support color temperature in mireds (1,000,000 / Kelvin)
- Typical range: 2000K-6500K (153-500 mireds)
- Convert temperature to RGB
- Update ColorTemperatureMireds attribute
- Set ColorMode to 2 (ColorTemperature)

**Note:** Single RGB LED may not accurately represent color temperature. This feature may be marked as unsupported in feature map.

#### 14-16. Enhanced Hue Commands
**Status:** ❌ Not Implemented

- EnhancedMoveToHue (0x40)
- EnhancedMoveHue (0x41)
- EnhancedStepHue (0x42)
- EnhancedMoveToHueAndSaturation (0x43)

**Requirements:**
- 16-bit hue value (0-65535) instead of 8-bit
- Smoother color transitions
- Update EnhancedCurrentHue attribute
- Set EnhancedColorMode to 3

#### 17. ColorLoopSet (0x44)
**Status:** ❌ Not Implemented

**Parameters:**
- UpdateFlags (bitmap8): Which fields to update
- Action (enum8): Deactivate/ActivateFromStart/ActivateFromCurrent
- Direction (enum8): Decrement/Increment
- Time (uint16): Seconds for full loop
- StartHue (uint16): Starting enhanced hue

**Requirements:**
- Automatically cycle through colors
- Update ColorLoopActive, Direction, Time attributes
- Continue looping until deactivated
- Can pause and resume

#### 18. StopMoveStep (0x47)
**Status:** ❌ Not Implemented

**Parameters:**
- OptionsMask, OptionsOverride

**Requirements:**
- Stop any ongoing Move or ColorLoop command
- Leave color at current value
- Clear RemainingTime

### Options Attribute Behavior

**Options Attribute (0x000F):** bitmap8

**Bit 0: ExecuteIfOff**
- When set: Commands execute even if OnOff is false
- When clear: Commands ignored if device is off
- Default: 0 (commands ignored when off)

**OptionsMask/OptionsOverride Pattern:**

All color commands include OptionsMask and OptionsOverride parameters:
```
EffectiveOptions = (Options & ~OptionsMask) | (OptionsOverride & OptionsMask)
```

This allows temporary overrides without changing the stored Options attribute.

**Current Status:** Not implemented, ExecuteIfOff always assumed false.

### Transition Timing Requirements

**TransitionTime Parameter:**
- Units: 1/10 second (tenths of a second)
- 0x0000 = immediate
- 0xFFFF = use device default (typically 4 seconds)

**RemainingTime Attribute:**
- Reports time left in current transition
- Units: 1/10 second
- Updated continuously during transition
- 0 when no transition active

**Implementation Requirements:**
- Use Embassy timers for transitions
- Interpolate values smoothly
- Update attributes periodically
- Support cancellation via StopMoveStep

**Current Status:** No transition support, all changes are immediate.

### Color Mode Management

**ColorMode Attribute Values:**
- 0x00: CurrentHueAndCurrentSaturation
- 0x01: CurrentXAndCurrentY
- 0x02: ColorTemperatureMireds

**EnhancedColorMode Attribute Values:**
- 0x00: CurrentHueAndCurrentSaturation
- 0x01: CurrentXAndCurrentY
- 0x02: ColorTemperatureMireds
- 0x03: EnhancedCurrentHueAndCurrentSaturation

**Rules:**
- ColorMode changes automatically based on last command used
- Hue/Sat commands → mode 0
- XY commands → mode 1
- Temperature commands → mode 2
- Enhanced commands → enhanced mode 3

**Current Status:** Reports mode 1 (XY) but only implements HS commands.

## Device Attestation & Commissioning Requirements

### Test Credentials Problem

**Current Implementation:**
```rust
use rs_matter_embassy::matter::dm::devices::test::{
    TEST_DEV_ATT,    // Test Device Attestation
    TEST_DEV_COMM,   // Test Device Commissioning
    TEST_DEV_DET     // Test Device Details
};
```

### What These Contain

#### TEST_DEV_DET (Device Details)
- Vendor ID
- Product ID
- Device name
- Serial number
- Manufacturing date
- Hardware version
- Software version

**Problem:** All devices report identical details.

#### TEST_DEV_COMM (Commissioning Data)
- Discriminator (12-bit value for BLE pairing)
- Setup PIN code / QR code data
- Commissioning flow type
- Discovery capabilities

**Problem:** All devices use same setup code, not unique per device.

#### TEST_DEV_ATT (Device Attestation)
- Device Attestation Certificate (DAC)
- Product Attestation Intermediate (PAI)
- Certificate Declaration (CD)
- Private key for attestation

**Problem:** All devices share same certificates and private keys - major security issue.

### Required for Production

Each device MUST have unique:

1. **Device Attestation Certificate (DAC)**
   - Generated per-device during manufacturing
   - Contains device's unique public key
   - Signed by Product Attestation Intermediate (PAI)

2. **Attestation Private Key**
   - Unique per device
   - Stored securely (ideally in secure element)
   - Never shared between devices
   - Used to prove device authenticity

3. **Commissioning Discriminator**
   - 12-bit value (0-4095)
   - Should be unique per device or batch
   - Used during BLE discovery

4. **Setup PIN Code**
   - 8-digit decimal (technically 27-bit)
   - Should be unique per device
   - Printed on device/packaging (QR code)
   - Used to authenticate commissioning

5. **Vendor ID (VID)**
   - Assigned by CSA (Connectivity Standards Alliance)
   - Requires membership and certification
   - Test VID: 0xFFF1 (DO NOT USE IN PRODUCTION)

6. **Product ID (PID)**
   - Assigned by vendor for each product
   - Unique per product model

7. **Serial Number**
   - Unique identifier per physical unit
   - Used for warranty, support, tracking

### Certificate Chain

```
[Root CA - Owned by CSA]
    ↓ signs
[Product Attestation Authority (PAA) - Owned by Vendor/DCL]
    ↓ signs
[Product Attestation Intermediate (PAI) - Owned by Vendor]
    ↓ signs
[Device Attestation Certificate (DAC) - Unique per device]
```

### Manufacturing Requirements

For production, each device needs:

1. **During Manufacturing:**
   - Generate unique key pair (per device)
   - Create Certificate Signing Request (CSR)
   - Sign CSR with PAI to create DAC
   - Program DAC + private key into secure storage
   - Generate unique discriminator
   - Generate or assign unique setup PIN
   - Print setup QR code on label
   - Store commissioning data in NVS

2. **Secure Storage:**
   - ESP32-C6 flash encryption for credentials
   - Or use external secure element (ATECC608, etc.)
   - Protect private keys from extraction

3. **Certificate Management:**
   - Obtain vendor PAI from DCL (Device Certificate Library)
   - Maintain secure key management system
   - Implement certificate revocation if needed

### Development vs Production

**Development/Testing:**
- Use TEST credentials OK
- Single device or small team
- No public deployment

**Production:**
- MUST use unique credentials
- Required for certification
- Required for commercial sale
- Legal requirement in some jurisdictions

## Persistence Requirements

### What Must Persist Across Reboots

1. **Commissioning State:**
   - ✅ Fabric information (handled by Matter stack)
   - ✅ Access Control Lists (ACLs)
   - ✅ Network credentials (WiFi SSID/password)

2. **Device State:**
   - ✅ OnOff state (persisted via Matter stack)
   - ❌ CurrentHue value
   - ❌ CurrentSaturation value
   - ❌ CurrentX, CurrentY (if implemented)
   - ❌ ColorTemperatureMireds (if implemented)

3. **Configuration:**
   - ❌ Options attribute value
   - ❌ StartUpColorTemperatureMireds attribute (if supported)

**Current Status:** Only commissioning state persists. Color settings reset to defaults on reboot.

### Startup Behavior

**StartUpColorTemperatureMireds Attribute (0x4010):**
- Optional attribute
- Sets color temperature on startup
- Null = restore previous value
- If supported, must persist

**Recommended Implementation:**
- Store color state (hue, sat, or XY) in NVS
- Restore on boot before commissioning completes
- Use Matter stack's persist mechanism

## Certification Requirements

To pass Matter certification:

1. ✅ Implement all mandatory clusters
2. ❌ Implement all mandatory attributes for claimed features
3. ❌ Implement all mandatory commands for claimed features
4. ❌ Pass interoperability tests with major controllers
5. ❌ Use production device attestation credentials
6. ❌ Pass security audit
7. ❌ Pass RF testing (WiFi compliance)
8. ❌ Submit to certified products database

**Current Status:** Not certifiable - missing features, using test credentials.

## Summary of Gaps

### Critical (Blocking Production)
1. ❌ Test credentials must be replaced with unique device credentials
2. ❌ Unique setup codes per device
3. ❌ Manufacturing process for credential generation

### High Priority (Spec Compliance)
1. ❌ Implement missing HS color commands (MoveToHue, MoveHue, etc.)
2. ❌ Add transition timing support with Embassy timers
3. ❌ Implement CurrentHue, CurrentSaturation attribute reads
4. ❌ Fix ColorMode reporting to match actual implementation
5. ❌ Implement Options attribute write handler
6. ❌ Add proper OptionsMask/OptionsOverride handling
7. ❌ Persist color state across reboots

### Medium Priority (Enhanced Features)
1. ❌ XY color space support (if claiming XY feature)
2. ❌ Color temperature support (if claiming CT feature)
3. ❌ Enhanced hue (16-bit precision)
4. ❌ RemainingTime attribute during transitions

### Low Priority (Advanced Features)
1. ❌ Color loop functionality
2. ❌ StopMoveStep command
3. ❌ StartUpColorTemperatureMireds attribute

### Optional (Nice to Have)
1. Level Control cluster integration (brightness)
2. Groups cluster (control multiple lights)
3. Scenes cluster (save/recall settings)
4. Identify cluster (make device blink for identification)
5. OTA update cluster

## Recommended Feature Set

Given hardware limitations (single RGB LED), recommended feature support:

**Claim These Features:**
- ✅ Hue/Saturation (HS) - bit 0x01
- ✅ Enhanced Hue (EHUE) - bit 0x02 (adds smooth transitions)

**Do NOT Claim:**
- ❌ Color Loop (CL) - bit 0x04 (nice-to-have)
- ❌ XY - bit 0x08 (can convert from HS)
- ❌ Color Temperature (CT) - bit 0x10 (RGB LED can't accurately represent)

This allows full compliance with minimal implementation while being honest about capabilities.
