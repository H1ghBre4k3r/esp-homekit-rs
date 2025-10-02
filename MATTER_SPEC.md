# Matter Specification Requirements for Extended Color Light

This document outlines the Matter 1.2 specification requirements for implementing a compliant Extended Color Light device (Device Type 0x010D).

**Reference:** Matter 1.2 Device Library Specification & Application Cluster Specification

## Device Type: Extended Color Light (0x010D)

### Classification
- **Device ID:** 0x010D
- **Name:** Extended Color Light
- **Superset Of:** Color Temperature Light (0x010C)
- **Class:** Simple
- **Scope:** Endpoint
- **Revision:** 2 (or higher for Matter 1.2+)

### Description
An Extended Color Light is capable of:
- Being switched on/off
- Adjusting light intensity (brightness)
- Adjusting color via multiple methods:
  - Hue/Saturation (HS)
  - Enhanced Hue
  - XY color coordinates
  - Color Temperature (CT)
  - Color looping
- Can be bound to occupancy sensors for automatic control

## Required Server Clusters

Extended Color Light devices MUST implement the following server clusters on the light endpoint:

| Cluster ID | Cluster Name | Purpose |
|-----------|--------------|---------|
| 0x0003 | Identify | Device identification (blink/alert) |
| 0x0004 | Groups | Group addressing support |
| 0x0005 | Scenes | Scene management |
| 0x0006 | OnOff | On/off control |
| 0x0008 | LevelControl | Brightness/dimming control |
| 0x0300 | ColorControl | Color management |

**Note:** Descriptor cluster (0x001D) is implicitly required for all endpoints.

## Cluster-Specific Requirements

### 1. Identify Cluster (0x0003)

**Purpose:** Allows controllers to identify which physical device they're interacting with.

#### Required Attributes
- `IdentifyTime` (0x0000, uint16) - Seconds remaining in identify mode

#### Required Commands
- `Identify` - Put device in identify mode for specified duration
- `TriggerEffect` (optional but recommended) - Trigger specific visual effects

#### Typical Implementation
- Blink/flash the light when identified
- Continue normal operation while identifying
- Auto-stop after IdentifyTime expires

---

### 2. Groups Cluster (0x0004)

**Purpose:** Enables group addressing - send one command to multiple devices.

#### Required Attributes
- `NameSupport` (0x0000, bitmap8) - Indicates if group names are supported

#### Required Commands
- `AddGroup` - Add device to a group
- `ViewGroup` - Query group membership
- `GetGroupMembership` - List groups device belongs to
- `RemoveGroup` - Remove from a group
- `RemoveAllGroups` - Leave all groups
- `AddGroupIfIdentifying` - Conditional group add

#### Implementation Notes
- Store group membership in NVS (persistence required)
- Support minimum 8 groups (more is better)
- Prerequisite for Scenes cluster

---

### 3. Scenes Cluster (0x0005)

**Purpose:** Store and recall complete device states (snapshots of attributes).

#### Required Attributes
- `SceneCount` (0x0000, uint8) - Number of scenes stored
- `CurrentScene` (0x0001, uint8) - Currently active scene ID
- `CurrentGroup` (0x0002, uint16) - Group of current scene
- `SceneValid` (0x0003, bool) - Is current scene valid
- `NameSupport` (0x0004, bitmap8) - Scene naming support

#### Required Commands
- `AddScene` - Store current state as scene
- `ViewScene` - Retrieve scene definition
- `RemoveScene` - Delete a scene
- `RemoveAllScenes` - Delete all scenes in a group
- `StoreScene` - Update existing scene
- `RecallScene` - Restore device to scene state
- `GetSceneMembership` - List scenes in a group

#### Scene Data to Store
For Extended Color Light, each scene must capture:
- OnOff state
- Current brightness level (LevelControl)
- Color mode and color values (ColorControl):
  - CurrentHue, CurrentSaturation (if HS mode)
  - CurrentX, CurrentY (if XY mode)
  - ColorTemperatureMireds (if CT mode)

#### Implementation Notes
- Store scene data in NVS (persistence required)
- Support minimum 16 scenes per group
- Scenes are group-scoped (requires Groups cluster)
- Must handle transitions when recalling scenes

---

### 4. OnOff Cluster (0x0006)

**Purpose:** Basic on/off control.

#### Required Attributes
- `OnOff` (0x0000, bool) - Current on/off state

#### Required Commands
- `Off` - Turn off
- `On` - Turn on
- `Toggle` - Toggle state

#### Optional Commands (Recommended)
- `OffWithEffect` - Turn off with visual effect
- `OnWithRecallGlobalScene` - Turn on and recall scene
- `OnWithTimedOff` - Turn on for specified duration

#### Implementation Notes
- **Already fully implemented in current codebase** ✅
- State changes trigger subscriptions via data version changes

---

### 5. LevelControl Cluster (0x0008)

**Purpose:** Brightness/dimming control with smooth transitions.

#### Required Attributes
- `CurrentLevel` (0x0000, uint8, nullable) - Current brightness (0-254, null=unknown)
- `RemainingTime` (0x0001, uint16) - Transition time remaining (deciseconds)
- `MinLevel` (0x0002, uint8) - Minimum brightness (default: 1)
- `MaxLevel` (0x0003, uint8) - Maximum brightness (default: 254)
- `OnLevel` (0x0011, uint8, nullable) - Brightness when turned on (null=restore)

#### Optional Attributes (Recommended)
- `Options` (0x000F, bitmap8) - Behavior options
- `StartUpCurrentLevel` (0x4000, uint8, nullable) - Power-on brightness

#### Required Commands
- `MoveToLevel` - Set brightness to specific level with transition
- `Move` - Continuously change brightness (up/down)
- `Step` - Change brightness by relative amount
- `Stop` - Stop ongoing transition
- `MoveToLevelWithOnOff` - Set level and turn on if needed
- `MoveWithOnOff` - Move brightness and control on/off
- `StepWithOnOff` - Step brightness and control on/off
- `StopWithOnOff` - Stop with on/off handling

#### Implementation Notes
- **NOT IMPLEMENTED** ❌
- Must support smooth transitions (no instant jumps unless transition=0)
- Update `RemainingTime` during transitions
- Coordinate with OnOff cluster
- When off, brightness changes prepare for next on event (depending on Options)

---

### 6. ColorControl Cluster (0x0300)

**Purpose:** Color management with multiple color space support.

#### Required Features for Extended Color Light
Per Matter spec, Extended Color Light MUST support:
- **XY** (bit 1) - CIE 1931 XY color space - **MANDATORY**
- **CT** (bit 4) - Color Temperature in mireds - **MANDATORY**

Optional features (nice to have):
- **HS** (bit 0) - Hue/Saturation (currently implemented)
- **EHUE** (bit 2) - Enhanced Hue (16-bit precision)
- **CL** (bit 3) - Color Loop

#### Required Attributes (Base)
- `CurrentHue` (0x0000, uint8) - Hue value 0-254 (if HS feature)
- `CurrentSaturation` (0x0001, uint8) - Saturation 0-254 (if HS feature)
- `RemainingTime` (0x0002, uint16) - Transition time remaining (deciseconds) - **MANDATORY REPORTING**
- `CurrentX` (0x0003, uint16) - CIE X coordinate (if XY feature) - **REQUIRED**
- `CurrentY` (0x0004, uint16) - CIE Y coordinate (if XY feature) - **REQUIRED**
- `ColorTemperatureMireds` (0x0007, uint16) - Color temp in mireds (if CT feature) - **REQUIRED**
- `ColorMode` (0x0008, enum8) - Current active color mode (HS=0, XY=1, CT=2) - **MANDATORY**
- `Options` (0x000F, bitmap8) - Behavior options - **MANDATORY** (writable)
- `NumberOfPrimaries` (0x0010, uint8, nullable) - Number of color primaries - **MANDATORY**
- `EnhancedColorMode` (0x4001, enum8) - Enhanced color mode (if EHUE feature)
- `ColorCapabilities` (0x400A, bitmap16) - Supported color features - **MANDATORY**

#### Optional Attributes (for CT mode)
- `ColorTempPhysicalMinMireds` (0x400B, uint16) - Minimum supported CT
- `ColorTempPhysicalMaxMireds` (0x400C, uint16) - Maximum supported CT

#### Required Commands - Hue/Saturation (if HS feature)
- `MoveToHue` (0x00) - Set hue with transition
- `MoveHue` (0x01) - Continuously change hue
- `StepHue` (0x02) - Change hue by step
- `MoveToSaturation` (0x03) - Set saturation with transition
- `MoveSaturation` (0x04) - Continuously change saturation
- `StepSaturation` (0x05) - Change saturation by step
- `MoveToHueAndSaturation` (0x06) - Set both with transition

#### Required Commands - XY Color (if XY feature) **MANDATORY**
- `MoveToColor` (0x07) - Set XY coordinates with transition
- `MoveColor` (0x08) - Continuously move in XY space
- `StepColor` (0x09) - Change XY by step

#### Required Commands - Color Temperature (if CT feature) **MANDATORY**
- `MoveToColorTemperature` (0x0A) - Set color temp with transition
- `MoveColorTemperature` (0x4B) - Continuously change color temp
- `StepColorTemperature` (0x4C) - Change color temp by step

#### Required Commands - Enhanced Hue (if EHUE feature)
- `EnhancedMoveToHue` (0x40) - 16-bit hue with transition
- `EnhancedMoveHue` (0x41) - Continuously change enhanced hue
- `EnhancedStepHue` (0x42) - Step enhanced hue
- `EnhancedMoveToHueAndSaturation` (0x43) - Set enhanced hue + saturation

#### Required Commands - Color Loop (if CL feature)
- `ColorLoopSet` (0x44) - Configure/start color looping

#### Required Commands - Universal
- `StopMoveStep` (0x47) - Stop any ongoing color transition

#### Color Mode Behavior
The `ColorMode` attribute must dynamically reflect which color command was most recently used:
- Use `MoveToHue*` or `MoveSaturation*` → `ColorMode` = 0 (HS)
- Use `MoveToColor` or `StepColor` → `ColorMode` = 1 (XY)
- Use `MoveToColorTemperature*` → `ColorMode` = 2 (CT)

When mode changes, the device should maintain visual appearance (convert between color spaces).

#### Transition Requirements
All "MoveTo*" commands include a transition time parameter:
- `TransitionTime` in 1/10 second units (deciseconds)
- 0 = instant change
- 0xFFFF = use default transition time
- Must smoothly animate over specified duration
- Update `RemainingTime` attribute during transition
- Support concurrent transitions (level + color)

#### Implementation Notes
- **Current Status:** Only `MoveToHueAndSaturation` implemented, no XY or CT support ❌
- **Missing:** XY color space (MANDATORY), Color Temperature (MANDATORY)
- **Missing:** All transition/animation logic
- **Missing:** Proper ColorMode tracking
- Must convert between HS, XY, and CT color spaces
- Must persist current color state in NVS

## Color Space Conversion Requirements

### HSV ↔ XY Conversion
Extended Color Light must convert between HSV (internal) and CIE 1931 XY:
- Controllers may send XY commands → convert to HSV for LED output
- When reporting CurrentX/CurrentY → convert from HSV

**CIE XY to RGB conversion:**
1. XY values are uint16 (0-65535 representing 0.0-1.0)
2. Convert to RGB using CIE 1931 color matching functions
3. Apply gamma correction for LED
4. Set LED RGB values

**RGB/HSV to CIE XY conversion:**
1. Convert to linear RGB (reverse gamma)
2. Apply RGB to XYZ transformation matrix
3. Calculate x = X/(X+Y+Z), y = Y/(X+Y+Z)
4. Scale to uint16 range

### Color Temperature ↔ RGB Conversion
Color temperature in mireds (micro reciprocal degrees):
- Mireds = 1,000,000 / Kelvin
- Typical range: 153 mireds (6500K, cool) to 500 mireds (2000K, warm)

**CT to RGB:**
1. Convert mireds to Kelvin: K = 1,000,000 / mireds
2. Use Planckian locus approximation or lookup table
3. Calculate RGB from blackbody curve
4. Apply brightness scaling

## State Persistence Requirements

The following state MUST be persisted in NVS and restored on reboot:

### OnOff Cluster
- `OnOff` state (if `StartUpOnOff` attribute set to restore previous)

### LevelControl Cluster
- `CurrentLevel`
- `StartUpCurrentLevel` (if implemented)

### ColorControl Cluster
- `CurrentHue` and `CurrentSaturation` (if HS mode)
- `CurrentX` and `CurrentY` (if XY mode)
- `ColorTemperatureMireds` (if CT mode)
- `ColorMode` (current active mode)
- `Options` (user configuration)

### Groups Cluster
- Group membership list

### Scenes Cluster
- All stored scenes (scene ID, group ID, attribute snapshots)

## Interoperability Requirements

### Ecosystem Support
Extended Color Light must work with:
- Google Home (Matter support)
- Amazon Alexa (Matter support)
- Apple Home (Matter support)
- Samsung SmartThings (Matter support)
- Other Matter controllers

### Testing Recommendations
1. Commission device with multiple ecosystems
2. Test all color modes (HS, XY, CT)
3. Verify smooth transitions
4. Test group and scene functionality
5. Verify state persistence across reboots
6. Test concurrent operations (brightness + color change)

## Compliance Summary

To be Matter 1.2 compliant as Extended Color Light (0x010D):

✅ **Must Have:**
1. All 6 server clusters (Identify, Groups, Scenes, OnOff, LevelControl, ColorControl)
2. XY color mode support (mandatory feature)
3. Color Temperature mode support (mandatory feature)
4. Smooth transitions for all MoveTo commands
5. State persistence in NVS
6. Proper ColorMode tracking

⚠️ **Should Have:**
1. Hue/Saturation mode (optional but recommended)
2. Enhanced Hue mode (optional)
3. Color Loop mode (optional)
4. All optional OnOff commands (effects, timed off)
5. Startup configuration attributes

❌ **Current Implementation Status:**
- Missing: Identify, Groups, Scenes, LevelControl clusters
- Missing: XY and CT color modes (mandatory)
- Missing: Transition/animation logic
- Missing: State persistence for attributes
- Device type ID incorrect (0x010C instead of 0x010D)

**Next Steps:** See ROADMAP.md for implementation plan.
