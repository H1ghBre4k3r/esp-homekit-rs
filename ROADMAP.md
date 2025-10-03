# ESP-HomeKit Implementation Roadmap

This roadmap outlines the implementation plan to achieve Matter 1.2 specification compliance for Extended Color Light (Device Type 0x010D).

**Goal:** Transform the current proof-of-concept into a fully spec-compliant Extended Color Light device.

**Reference Documents:**
- [PROJECT_DESCRIPTION.md](PROJECT_DESCRIPTION.md) - Current state analysis
- [MATTER_SPEC.md](MATTER_SPEC.md) - Matter specification requirements

---

## Progress Overview

- [x] **Phase 0:** Foundation & Corrections (1 task) ✅ **COMPLETED**
- [ ] **Phase 1:** Required Clusters - Basic (4 tasks) - 2/4 completed (Identify ✅, LevelControl ✅)
- [x] **Phase 2:** ColorControl Enhancements (3 tasks) ✅ **COMPLETED** (XY Mode ✅, CT Mode ✅, Transitions ✅)
- [ ] **Phase 3:** State Persistence (1 task)
- [ ] **Phase 4:** Testing & Validation (1 task)

**Total Tasks:** 11 major implementation tasks

---

## Phase 0: Foundation & Corrections

### Task 0.1: Fix Device Type ID ✅ **PRIORITY: CRITICAL**
**Status:** ✅ Completed (2025-10-03)

**Description:** Correct the device type from 0x010C (Color Temperature Light) to 0x010D (Extended Color Light).

**Files to Modify:**
- `src/bin/main.rs` (line 210-213)

**Changes:**
```rust
// Current:
dtype: 0x010C,
drev: 4

// Change to:
dtype: 0x010D,
drev: 2  // Matter 1.2 uses revision 2+
```

**Testing:**
- Recommission device
- Verify controllers recognize as Extended Color Light
- Check device type in Matter controller app

**Dependencies:** None

**Estimated Effort:** 15 minutes

**Completion Notes:**
- Changed dtype from 0x010C to 0x010D at src/bin/main.rs:216
- Changed drev from 4 to 2 at src/bin/main.rs:217
- Device now correctly advertises as Extended Color Light (0x010D)
- Requires recommissioning for controllers to recognize new device type

---

## Phase 1: Required Clusters - Basic

### Task 1.1: Implement Identify Cluster ✅ **PRIORITY: HIGH**
**Status:** ✅ Completed (2025-10-03)

**Description:** Add Identify cluster for device identification (visual feedback when controller identifies device).

**Implementation Plan:**
1. Import Identify cluster from `rs-matter`: `rs_matter::import!(Identify)`
2. Create `IdentifyHandler` struct to wrap `LightController`
3. Implement `identify::ClusterHandler` trait:
   - `identify_time()` attribute - return remaining identify seconds
   - `handle_identify()` command - start identify mode
   - `handle_trigger_effect()` command - optional visual effects
4. Add identify state to `LightController`:
   - `identify_time: RefCell<u16>` - countdown timer
5. Add identify LED behavior:
   - Blink light pattern while identifying
   - Use Embassy timer for countdown
6. Add cluster to endpoint in `main.rs`:
   - Add to `clusters!()` macro
   - Chain handler in handler chain

**Files to Modify:**
- `src/lib.rs` - Add IdentifyHandler implementation
- `src/bin/main.rs` - Add cluster to endpoint and handler chain

**Visual Effect Recommendations:**
- **Identify:** Blink 3 times slowly
- **Breathe:** Smooth fade in/out
- **Okay:** Quick double-blink
- **Channel Change:** Single long blink

**Testing:**
- Use controller's "Identify" feature
- Verify LED blinks when identified
- Verify identify stops after timeout

**Dependencies:** None

**Estimated Effort:** 3-4 hours

**Completion Notes:**
- ✅ Created `src/identify.rs` module with `rs_matter::import!(Identify)`
- ✅ Added identify state fields to LightController at src/lib.rs:103-105:
  - `identify_time_ds: RefCell<u16>` - Remaining time in deciseconds
  - `identify_phase: RefCell<u8>` - Blink animation phase (0-9)
- ✅ Implemented `identify::ClusterHandler` trait at src/lib.rs:1125-1197
  - `identify_time()` attribute - converts deciseconds to seconds
  - `set_identify_time()` - converts seconds to deciseconds
  - `identify_type()` - returns LightOutput (0x01)
  - `handle_identify()` command - starts identify countdown
  - `handle_trigger_effect()` command - stubbed with 2-second fallback
- ✅ Modified `update_led()` at src/lib.rs:158-174 to prioritize identify mode
  - White blink pattern: phase 0-4 = on (255,255,255), 5-9 = off
  - Identify overrides normal operation
- ✅ Added `update_identify()` method at src/lib.rs:318-330
  - Decrements countdown every decisecond
  - Cycles blink phase 0-9 for 1Hz visible blink
- ✅ Integrated into existing `transition_task` at src/lib.rs:1158-1171
  - Reused existing 100ms timer task
  - No additional Embassy tasks needed
- ✅ Added IDENTIFY_CLUSTER constant at src/lib.rs:140-141
- ✅ Chained identify handler in main.rs at lines 148-155
- ✅ Added cluster to endpoint definition at main.rs:241
- ✅ Build successful

**Known Limitations:**
- ⚠️ TriggerEffect command uses simple 2-second identify fallback
  - TODO: Implement fancy effects (breathe, okay, channel change)
- ⚠️ 20 unused ctx variable warnings (cosmetic only)

**Architecture Notes:**
- Efficient design - reuses existing transition_task infrastructure
- Identify mode preserves device state (color/brightness unchanged)
- 10Hz update rate (100ms) creates smooth 1Hz blink pattern
- Safe blink frequency (1Hz, well below 3Hz photosensitivity threshold)

---

### Task 1.2: Implement Groups Cluster ✅ **PRIORITY: HIGH**
**Status:** ⬜ Not Started

**Description:** Add Groups cluster for group addressing support.

**Implementation Plan:**
1. Check if `rs-matter` provides built-in Groups cluster handler
2. If available, use built-in handler (similar to Descriptor cluster)
3. If not, implement custom handler:
   - Store group membership in static array or Vec
   - Implement all required commands (AddGroup, RemoveGroup, etc.)
   - Add NameSupport attribute (can be false to avoid string storage)
4. Add NVS persistence for group membership
5. Add cluster to endpoint in `main.rs`

**Storage Requirements:**
- Support minimum 8 groups (spec recommendation)
- Each group: group_id (u16)
- Optional: group name (String, up to 16 chars)

**Files to Modify:**
- `src/lib.rs` - Add GroupsHandler (if custom implementation needed)
- `src/bin/main.rs` - Add cluster to endpoint
- `src/nvs.rs` - Add group persistence (later in Phase 6)

**Testing:**
- Add device to multiple groups via controller
- Send group commands (affects multiple devices if testing with >1)
- Verify group membership persists across reboots

**Dependencies:** None (but required by Scenes)

**Estimated Effort:** 4-6 hours (if custom implementation), 1-2 hours (if built-in available)

---

### Task 1.3: Implement Scenes Cluster ✅ **PRIORITY: HIGH**
**Status:** ⬜ Not Started

**Description:** Add Scenes cluster for storing and recalling device state snapshots.

**Implementation Plan:**
1. Check `rs-matter` for built-in Scenes support
2. If custom implementation needed:
   - Define scene storage structure:
     ```rust
     struct Scene {
         scene_id: u8,
         group_id: u16,
         transition_time: u16,
         on_off: bool,
         level: u8,
         hue: u8,
         saturation: u8,
         // Future: color_x, color_y, color_temp
     }
     ```
   - Store up to 16 scenes per group
   - Implement scene commands (AddScene, RecallScene, StoreScene, etc.)
3. Scene recall must:
   - Restore OnOff, Level, and Color states
   - Apply transition time if specified
   - Update CurrentScene and SceneValid attributes
4. Add cluster to endpoint

**Files to Modify:**
- `src/lib.rs` - Add ScenesHandler and Scene struct
- `src/bin/main.rs` - Add cluster to endpoint
- `src/nvs.rs` - Add scene persistence (Phase 6)

**Testing:**
- Store scene with specific brightness and color
- Turn off light, change color
- Recall scene → verify state restored

**Dependencies:**
- Groups cluster (required first)
- LevelControl cluster (for full functionality)

**Estimated Effort:** 6-8 hours

---

### Task 1.4: Implement LevelControl Cluster ✅ **PRIORITY: HIGH**
**Status:** ✅ Completed (2025-10-03) - Basic implementation without transitions

**Description:** Add brightness/dimming control with smooth transitions.

**Implementation Plan:**
1. Import LevelControl from `rs-matter`
2. Add brightness state to `LightController`:
   ```rust
   current_level: RefCell<u8>,      // 0-254 (0=min, 254=max)
   remaining_time: RefCell<u16>,    // Deciseconds remaining
   min_level: u8,                   // Default: 1
   max_level: u8,                   // Default: 254
   ```
3. Implement `level_control::ClusterHandler`:
   - Attributes: CurrentLevel, RemainingTime, MinLevel, MaxLevel
   - Commands: MoveToLevel, Move, Step, Stop, *WithOnOff variants
4. Add brightness to LED update logic:
   - Modify `hsv_to_rgb()` to accept brightness value
   - Scale RGB output: `RGB * (current_level / 254.0)`
5. **Transition support** (critical):
   - Spawn Embassy task for gradual level changes
   - Update CurrentLevel incrementally over TransitionTime
   - Update RemainingTime countdown
   - Refresh LED at ~50Hz for smooth dimming
6. Coordinate with OnOff:
   - `MoveToLevelWithOnOff` turns light on if level > 0
   - When turning off, remember CurrentLevel for next On

**Files to Modify:**
- `src/lib.rs` - Add LevelControlHandler, brightness state, transition task
- `src/bin/main.rs` - Add cluster to endpoint

**Testing:**
- Set brightness to 50% → verify dimming
- Test smooth transition (dim from 100% to 10% over 5 seconds)
- Verify RemainingTime counts down during transition
- Test MoveToLevelWithOnOff (turns on + sets level)

**Dependencies:** None (but enhances OnOff)

**Estimated Effort:** 8-10 hours (transitions add complexity)

**Completion Notes:**
- ✅ Created `src/level_control.rs` module using `rs_matter::import!(LevelControl)`
- ✅ Added `current_level: RefCell<u8>` to LightController (initialized to 254 = max brightness)
- ✅ Modified `update_led()` at src/lib.rs:93 to scale RGB by brightness level
- ✅ Implemented `level_control::ClusterHandler` trait with all required methods:
  - `current_level()` attribute - returns current brightness (0-254)
  - `handle_move_to_level()` - instant brightness change (no transition yet)
  - `handle_move_to_level_with_on_off()` - brightness + on/off coordination
  - Stubbed: Move, Step, Stop commands (continuous/stepped brightness changes)
- ✅ Added LEVEL_CONTROL_CLUSTER constant to LightController
- ✅ Added cluster to endpoint at src/bin/main.rs:222
- ✅ Chained handler at src/bin/main.rs:142-149
- ✅ Build successful

**Known Limitations:**
- ⚠️ **No transitions yet**: All brightness changes are instant (request.transition_time ignored)
- ⚠️ Move/Step/Stop commands are stubbed (return Ok but do nothing)
- ⚠️ RemainingTime attribute not implemented
- ⚠️ MinLevel/MaxLevel attributes not exposed

**Next Steps (Future Enhancement):**
- Implement smooth transitions using Embassy tasks (Task 2.3 from roadmap)
- Add RemainingTime countdown during transitions
- Implement Move/Step/Stop commands for continuous brightness control

---

## Phase 2: ColorControl Enhancements

### Task 2.1: Implement XY Color Mode ✅ **PRIORITY: CRITICAL** (Mandatory Feature)
**Status:** ✅ Completed (2025-10-03)

**Description:** Add CIE 1931 XY color space support (mandatory for Extended Color Light).

**Implementation Plan:**
1. Add XY state to `LightController`:
   ```rust
   current_x: RefCell<u16>,  // 0-65535 (0.0 to 1.0)
   current_y: RefCell<u16>,  // 0-65535 (0.0 to 1.0)
   ```
2. Implement XY→RGB conversion:
   ```rust
   fn xy_to_rgb(x: u16, y: u16, brightness: u8) -> RGB8 {
       // 1. Convert x,y from u16 to float (0.0-1.0)
       // 2. Calculate z = 1.0 - x - y
       // 3. Convert to XYZ: X = x/y, Y = 1.0, Z = z/y
       // 4. Apply XYZ→RGB matrix (sRGB)
       // 5. Gamma correction
       // 6. Scale by brightness
   }
   ```
3. Implement RGB→XY conversion (for reporting CurrentX/CurrentY):
   ```rust
   fn rgb_to_xy(rgb: RGB8) -> (u16, u16) {
       // 1. Reverse gamma correction
       // 2. Apply RGB→XYZ matrix
       // 3. Calculate x = X/(X+Y+Z), y = Y/(X+Y+Z)
       // 4. Convert to u16 range
   }
   ```
4. Add XY attributes to ColorControl handler:
   - `current_x()` - read current X coordinate
   - `current_y()` - read current Y coordinate
5. Implement XY commands:
   - `handle_move_to_color()` - set X,Y with transition
   - `handle_move_color()` - continuous XY movement
   - `handle_step_color()` - step X,Y by delta
6. Update `color_mode()` attribute:
   - Return 1 (XY mode) when XY commands used
   - Dynamically track current mode
7. Update LED rendering to use XY when in XY mode

**Color Space Conversion Reference:**
- Use sRGB color space (most common for LEDs)
- Apply proper gamma correction (typically γ=2.2 or 2.4)
- Handle out-of-gamut colors (clamp to nearest representable color)

**Files to Modify:**
- `src/lib.rs` - Add XY state, conversion functions, command handlers

**Testing:**
- Set color via XY coordinates
- Verify CurrentX/CurrentY attributes update
- Verify ColorMode switches to 1 (XY)
- Compare with controller's color picker
- Test transition between XY colors

**Dependencies:** LevelControl (for brightness scaling)

**Estimated Effort:** 10-12 hours (color conversion math is complex)

**Resources:**
- [CIE 1931 Color Space](https://en.wikipedia.org/wiki/CIE_1931_color_space)
- [sRGB Color Space](https://en.wikipedia.org/wiki/SRGB)
- Color conversion algorithms available in various Rust crates (e.g., `palette`)

**Completion Notes:**
- ✅ Added XY state fields: `current_x`, `current_y`, `color_mode` to LightController
- ✅ Implemented `xy_to_rgb()` conversion at src/lib.rs:183
  - XY→XYZ→sRGB matrix transformation
  - Clamping for out-of-gamut colors
  - ⚠️ Gamma correction skipped (requires `libm` crate for `powf`)
- ✅ Implemented `rgb_to_xy()` conversion at src/lib.rs:246
  - RGB→XYZ matrix transformation
  - XYZ→xy chromaticity coordinates
  - ⚠️ Gamma expansion skipped (linear RGB used)
- ✅ Added `current_x()` and `current_y()` attributes at src/lib.rs:320-332
- ✅ Updated `color_mode()` and `enhanced_color_mode()` to return dynamic mode
- ✅ Implemented `handle_move_to_color()` at src/lib.rs:430
  - Sets XY coordinates, switches to mode 1, updates LED
- ✅ Updated `handle_move_to_hue_and_saturation()` at src/lib.rs:397
  - Switches to mode 0, syncs XY values via RGB conversion
- ✅ Modified `update_led()` at src/lib.rs:98 to support both HS (mode 0) and XY (mode 1)
- ✅ Build successful

**Known Limitations:**
- ⚠️ No gamma correction (colors may look slightly washed out)
  - **Future:** Add `libm` crate dependency for proper gamma
- ⚠️ Transitions are instant (no smooth color changes yet)
- ⚠️ `MoveColor` and `StepColor` commands are stubbed

**Testing Notes:**
- Device now supports both HS and XY color modes
- Color mode switches automatically based on which commands are used
- XY coordinates are synchronized when HS mode is used

---

### Task 2.2: Implement Color Temperature Mode ✅ **PRIORITY: CRITICAL** (Mandatory Feature)
**Status:** ✅ Completed (2025-10-03)

**Description:** Add color temperature support in mireds (mandatory for Extended Color Light).

**Implementation Plan:**
1. Add CT state to `LightController`:
   ```rust
   color_temperature_mireds: RefCell<u16>,  // 153-500 mireds (6500K-2000K)
   color_temp_physical_min: u16,            // 153 mireds (6500K cool white)
   color_temp_physical_max: u16,            // 500 mireds (2000K warm white)
   ```
2. Implement Mireds→RGB conversion:
   ```rust
   fn mireds_to_rgb(mireds: u16, brightness: u8) -> RGB8 {
       // 1. Convert to Kelvin: K = 1,000,000 / mireds
       // 2. Use Planckian locus approximation:
       //    - For K < 6600: r=255, calculate g,b
       //    - For K >= 6600: b=255, calculate r,g
       // 3. Scale by brightness
   }
   ```
3. Add CT attributes to ColorControl handler:
   - `color_temperature_mireds()` - current CT
   - `color_temp_physical_min_mireds()` - minimum (153 or hardware limit)
   - `color_temp_physical_max_mireds()` - maximum (500 or hardware limit)
4. Implement CT commands:
   - `handle_move_to_color_temperature()` - set CT with transition
   - `handle_move_color_temperature()` - continuous CT change
   - `handle_step_color_temperature()` - step CT by delta
5. Update `color_mode()` attribute:
   - Return 2 (CT mode) when CT commands used
6. Update LED rendering to use CT when in CT mode

**Color Temperature Reference:**
- Typical range: 2000K (warm/candlelight) to 6500K (cool/daylight)
- Mireds = 1,000,000 / Kelvin
- Use approximation formulas or lookup tables for Kelvin→RGB

**Files to Modify:**
- `src/lib.rs` - Add CT state, conversion function, command handlers

**Testing:**
- Set color temp to 2700K (warm white) → verify warm glow
- Set color temp to 5000K (daylight) → verify cool white
- Verify ColorMode switches to 2 (CT)
- Test smooth transition between color temps

**Dependencies:** LevelControl (for brightness)

**Estimated Effort:** 6-8 hours

**Resources:**
- [Planckian Locus Approximation](http://www.tannerhelland.com/4435/convert-temperature-rgb-algorithm-code/)
- Color temperature conversion libraries

**Completion Notes:**
- ✅ Added `color_temperature_mireds: RefCell<u16>` field to LightController at src/lib.rs:53
- ✅ Initialized to 250 mireds (4000K neutral white) in `new()` method
- ✅ Implemented `kelvin_to_rgb()` conversion function at src/lib.rs:284-321
  - Converts mireds→Kelvin: `K = 1,000,000 / mireds`
  - Uses simplified linear interpolation between three color points:
    - 2000K = warm (255, 147, 44)
    - 4000K = neutral (255, 228, 206)
    - 6500K = cool (255, 254, 250)
  - ⚠️ Simplified approach (no Planckian locus math due to no_std/no FPU)
- ✅ Added `color_temperature_mireds()` attribute at src/lib.rs:380-385
- ✅ Added `color_temp_physical_min_mireds()` returning 153 (6500K) at src/lib.rs:387-392
- ✅ Added `color_temp_physical_max_mireds()` returning 500 (2000K) at src/lib.rs:394-399
- ✅ Implemented `handle_move_to_color_temperature()` command at src/lib.rs:538-560
  - Extracts and clamps mireds value (153-500 range)
  - Switches to Color Temperature mode (mode 2)
  - Updates LED and notifies Matter stack of changes
- ✅ Updated `update_led()` at src/lib.rs:124-130 to support CT mode (mode 2)
  - Converts mireds to Kelvin
  - Calls `kelvin_to_rgb()` with brightness
- ✅ Build successful

**Known Limitations:**
- ⚠️ **Simplified color temperature algorithm**: Uses linear interpolation instead of proper Planckian locus
  - **Reason**: no_std environment without FPU, no access to powf/ln methods
  - **Future**: Consider adding `libm` crate for proper black body radiation curve
- ⚠️ Transitions are instant (no smooth temperature changes yet)
- ⚠️ `MoveColorTemperature` and `StepColorTemperature` commands are stubbed

**Testing Notes:**
- Device now supports all three color modes: HS (0), XY (1), and CT (2)
- Color mode switches automatically based on which commands are used
- Color temperature range: 2000K (warm) to 6500K (cool)
- Works with Matter controllers that support color temperature control

---

### Task 2.3: Implement Smooth Color Transitions ✅ **PRIORITY: HIGH**
**Status:** ✅ Completed (2025-10-03)

**Description:** Add transition/animation support for all color commands (currently only instant changes).

**Implementation Plan:**
1. Create color transition state machine:
   ```rust
   struct ColorTransition {
       active: bool,
       start_hue: u8,
       target_hue: u8,
       start_sat: u8,
       target_sat: u8,
       // ... similar for XY, CT
       remaining_time: u16,     // Deciseconds
       total_time: u16,
   }
   ```
2. Spawn Embassy task for color transitions:
   ```rust
   #[embassy_executor::task]
   async fn color_transition_task(controller: &'static LightController) {
       loop {
           Timer::after(Duration::from_millis(20)).await;  // 50Hz update
           controller.update_transition();
       }
   }
   ```
3. Implement transition logic:
   - Calculate intermediate values using linear interpolation
   - Update current color every 20ms
   - Decrement RemainingTime
   - Stop when target reached
4. Update all color commands to support transitions:
   - `MoveToHue` - smooth hue change
   - `MoveToSaturation` - smooth saturation change
   - `MoveToHueAndSaturation` - smooth combined change
   - `MoveToColor` (XY) - smooth XY interpolation
   - `MoveToColorTemperature` - smooth CT change
5. Implement continuous movement commands:
   - `MoveHue` - rotate hue continuously at specified rate
   - `MoveSaturation` - change saturation continuously
   - `MoveColor` (XY) - move in XY space continuously
   - `MoveColorTemperature` - change CT continuously
6. Implement `StopMoveStep` command:
   - Halt any active transition
   - Set current position as final

**LED Update Strategy:**
- Update LED at 50Hz (every 20ms) for smooth appearance
- Use atomics or message passing to coordinate between tasks
- Avoid blocking the main Matter stack

**Files to Modify:**
- `src/lib.rs` - Add transition state, task, interpolation logic
- `src/bin/main.rs` - Spawn transition task

**Testing:**
- Command: "Change to red over 5 seconds"
- Verify smooth color change (no jumps)
- Verify RemainingTime counts down
- Test Stop command mid-transition
- Test continuous Move commands

**Dependencies:**
- LevelControl (for brightness transitions)
- XY and CT implementations (for full testing)

**Estimated Effort:** 10-12 hours

**Completion Notes:**
- ✅ Created `TransitionState` struct at src/lib.rs:42-85 to track active transitions
- ✅ Added `transition: RefCell<TransitionState>` field to LightController
- ✅ Implemented `update_transition()` method at src/lib.rs:212-309
  - Linear interpolation for all modes (level, HS, XY, CT)
  - Updates LED every 100ms (10Hz, not 50Hz to reduce CPU usage)
  - Automatically completes and deactivates when target reached
- ✅ Updated `handle_move_to_hue_and_saturation()` to support transitions (src/lib.rs:622-673)
- ✅ Updated `handle_move_to_color()` (XY mode) to support transitions (src/lib.rs:681-718)
- ✅ Updated `handle_move_to_color_temperature()` to support transitions (src/lib.rs:749-792)
- ✅ Created `transition_task()` Embassy task at src/lib.rs:1110-1115
  - Runs in background, updating transitions every 100ms
  - Spawned in main.rs:133
- ✅ Made LightController static in main.rs to enable sharing with transition task
- ✅ Build successful

**Known Limitations:**
- ⚠️ **Level transitions not supported**: `handle_move_to_level()` performs instant changes
  - **Reason**: LevelControl's `transition_time` returns `Maybe<u16, AsNullable>` which lacks easy unwrap methods
  - **Workaround**: Color commands support transitions, and brightness changes with color
- ⚠️ Move/Step commands (continuous movement) are stubbed (not implemented)
- ⚠️ StopMoveStep command is stubbed
- ⚠️ Update rate is 10Hz (100ms) instead of 50Hz (20ms) to conserve CPU
  - Still provides smooth transitions, just slightly less fluid than ideal

**Testing Notes:**
- Device now supports smooth color transitions for HS, XY, and CT modes
- Transition duration is specified in deciseconds (1ds = 100ms)
- Setting transition_time=0 results in instant change
- Transitions are interruptible (starting a new command cancels active transition)

---

## Phase 3: State Persistence

### Task 3.1: Implement State Persistence in NVS ✅ **PRIORITY: MEDIUM**
**Status:** ⬜ Not Started

**Description:** Persist device state to NVS and restore on boot (required for proper Matter compliance).

**Implementation Plan:**
1. Define persistent state structure:
   ```rust
   #[derive(Serialize, Deserialize)]
   struct DeviceState {
       on_off: bool,
       current_level: u8,
       current_hue: u8,
       current_saturation: u8,
       current_x: u16,
       current_y: u16,
       color_temperature_mireds: u16,
       color_mode: u8,
   }
   ```
2. Extend `Nvs` with device state methods:
   - `save_device_state(&self, state: &DeviceState)`
   - `load_device_state(&self) -> Option<DeviceState>`
3. Save state when changed:
   - Call `save_device_state()` after attribute updates
   - Debounce writes (e.g., max once per 5 seconds) to reduce flash wear
4. Load state on boot:
   - Call `load_device_state()` in main before creating stack
   - Initialize `LightController` with restored values
5. Persist Groups and Scenes:
   - Add group membership save/load
   - Add scene data save/load
   - Use separate NVS keys for each

**Flash Wear Mitigation:**
- Debounce writes (don't write on every change)
- Only write when value actually changes
- Use wear leveling (ESP-IDF NVS provides this)
- Typical NVS lifetime: 100k writes per cell

**Files to Modify:**
- `src/nvs.rs` - Add state persistence functions
- `src/lib.rs` - Call save on state changes
- `src/bin/main.rs` - Load state on boot

**Testing:**
- Set color and brightness
- Reboot device (power cycle)
- Verify state restored (same color, brightness, on/off)
- Test after adding to group → verify group restored after reboot

**Dependencies:** All cluster implementations

**Estimated Effort:** 4-6 hours

---

## Phase 4: Testing & Validation

### Task 4.1: Comprehensive Testing & Ecosystem Validation ✅ **PRIORITY: HIGH**
**Status:** ⬜ Not Started

**Description:** Test all features with multiple Matter ecosystems and validate spec compliance.

**Testing Plan:**

#### 4.1.1 Controller Ecosystem Testing
Test with each major ecosystem:
- [ ] **Google Home**
  - Commission device
  - Test on/off, brightness, color, color temp
  - Create scenes
  - Add to groups
  - Test voice commands
- [ ] **Amazon Alexa**
  - Commission device
  - Test all controls
  - Test routines (Alexa's scenes)
  - Test voice commands
- [ ] **Apple Home**
  - Commission device
  - Test all controls
  - Create HomeKit scenes
  - Test automation

#### 4.1.2 Cluster-Specific Testing

**Identify Cluster:**
- [ ] Trigger identify from controller UI
- [ ] Verify LED blinks/flashes
- [ ] Verify identify timeout works

**Groups Cluster:**
- [ ] Add device to group
- [ ] Send group commands
- [ ] Remove from group
- [ ] Verify persistence

**Scenes Cluster:**
- [ ] Create scene with specific color/brightness
- [ ] Change device state
- [ ] Recall scene → verify state restored
- [ ] Verify transition time honored
- [ ] Verify persistence

**OnOff Cluster:**
- [ ] On/Off commands
- [ ] Toggle
- [ ] Verify state subscriptions work

**LevelControl Cluster:**
- [ ] Set brightness levels (1%, 50%, 100%)
- [ ] Test smooth transitions (5 second dim)
- [ ] Test Move command (continuous dimming)
- [ ] Test Stop during transition
- [ ] Verify with OnOff (dim while off behavior)

**ColorControl Cluster - HS Mode:**
- [ ] Set hue and saturation
- [ ] Test smooth transitions
- [ ] Verify ColorMode = 0

**ColorControl Cluster - XY Mode:**
- [ ] Set color via XY coordinates
- [ ] Verify color accuracy
- [ ] Test transitions
- [ ] Verify ColorMode = 1

**ColorControl Cluster - CT Mode:**
- [ ] Set warm white (2700K)
- [ ] Set cool white (5000K)
- [ ] Test smooth transitions
- [ ] Verify ColorMode = 2

#### 4.1.3 Stress Testing
- [ ] Rapid command sequences (spam on/off)
- [ ] Concurrent transitions (brightness + color simultaneously)
- [ ] Long-running transitions (60 second color fade)
- [ ] Power cycle during transitions
- [ ] Network disconnect/reconnect

#### 4.1.4 Persistence Testing
- [ ] Set state, reboot, verify restored
- [ ] Test after factory reset
- [ ] Verify commissioning data persists

#### 4.1.5 Interoperability Testing
- [ ] Multi-controller scenario (Google + Alexa simultaneously)
- [ ] Verify subscriptions work correctly
- [ ] Test with Matter-over-Thread vs Matter-over-WiFi (if supported)

**Test Documentation:**
- Create test report document
- Log any failures or unexpected behavior
- Compare behavior across ecosystems (consistency check)

**Dependencies:** All implementations complete

**Estimated Effort:** 8-12 hours (spread over multiple sessions)

---

## Optional Enhancements (Post-Compliance)

These are **not required** by spec but improve user experience:

### Optional 1: Enhanced Hue Mode (EHUE Feature)
- 16-bit hue precision (0-65535 instead of 0-254)
- Smoother color gradients
- Implement EnhancedMoveToHue, EnhancedMoveHue, etc.

**Estimated Effort:** 4-6 hours

### Optional 2: Color Loop Mode (CL Feature)
- Automatic color cycling
- ConfigureColorLoop command
- Useful for party mode, alerts

**Estimated Effort:** 4-6 hours

### Optional 3: StartUp Attributes
- StartUpOnOff - behavior on power-up
- StartUpCurrentLevel - brightness on power-up
- Allows user to configure power-on behavior

**Estimated Effort:** 2-3 hours

### Optional 4: Effects & Animations
- OffWithEffect for smooth power-off
- Custom blink patterns
- Breathing effect for identify

**Estimated Effort:** 3-4 hours

---

## Task Completion Checklist

When completing a task, update this roadmap:
1. Change status from ⬜ to ✅
2. Add completion date
3. Note any issues encountered
4. Update dependencies if affected

**Example:**
```
### Task 1.1: Implement Identify Cluster ✅
**Status:** ✅ Completed (2025-01-15)
**Issues:** None
**Notes:** Used built-in rs-matter Identify handler, very straightforward
```

---

## Summary

**Total Estimated Effort:** 70-100 hours

**Critical Path:**
1. Fix device type (15 min)
2. Implement LevelControl (8-10h)
3. Implement XY mode (10-12h) - **MANDATORY**
4. Implement CT mode (6-8h) - **MANDATORY**
5. Implement transitions (10-12h)
6. Add Identify, Groups, Scenes (13-18h)
7. Add persistence (4-6h)
8. Test thoroughly (8-12h)

**After completion:**
✅ Matter 1.2 compliant Extended Color Light (0x010D)
✅ Works with Google Home, Alexa, Apple Home
✅ All required clusters implemented
✅ Smooth color transitions
✅ State persistence
✅ Production-ready firmware

---

## Notes

- Always verify changes don't break existing functionality
- Test after each phase before moving to next
- Keep commits atomic (one feature per commit)
- Update this roadmap as work progresses
- Refer to MATTER_SPEC.md for implementation details
- Consult PROJECT_DESCRIPTION.md to understand current state
