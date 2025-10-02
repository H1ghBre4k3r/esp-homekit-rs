# ESP32-C6 Matter Smart LED Controller - Development Roadmap

**Last Updated:** 2024-10-02  
**Project Status:** Early Development - Not Production Ready

## Overview

This roadmap focuses on implementing missing Matter specification requirements and replacing test credentials with proper production credentials. The goal is to achieve full Matter 1.2 compliance for an Extended Color Light device.

## Guiding Principles

1. **Specification Compliance First:** Implement required features per Matter spec, not new features
2. **Security:** Replace test credentials with unique device credentials
3. **Minimal Changes:** Surgical, focused changes to existing code
4. **Test After Each Phase:** Verify functionality before moving to next phase
5. **Reference This Document:** Always check this roadmap before starting new work

## Project Phases

### Phase 0: Foundation & Cleanup ✅ COMPLETED
**Goal:** Establish project documentation and understanding

- [x] Create PROJECT_DESCRIPTION.md - Documents current state
- [x] Create MATTER_SPEC_REQUIREMENTS.md - Documents specification requirements
- [x] Create ROADMAP.md - This document
- [x] Analyze missing features and compliance gaps

**Status:** ✅ Completed 2024-10-02

---

### Phase 1: Device Credentials Infrastructure 🔄 IN PROGRESS
**Goal:** Replace test credentials with proper device attestation system

**Priority:** CRITICAL (blocking production use)

#### 1.1: Understand Current Credential System
- [ ] Document how TEST_DEV_ATT, TEST_DEV_COMM, TEST_DEV_DET are used
- [ ] Identify all locations where test credentials are referenced
- [ ] Research rs-matter's credential API and required data structures
- [ ] Review ESP32-C6 secure storage options (flash encryption, eFuse)

#### 1.2: Design Credential Storage System
- [ ] Define data structures for device-specific credentials
- [ ] Design NVS schema for credentials (separate from Matter stack state)
- [ ] Plan manufacturing data provisioning flow
- [ ] Document key generation and certificate signing process

#### 1.3: Implement Credential Generation Tools
- [ ] Create build script to generate unique device credentials
- [ ] Implement certificate signing (DAC from PAI)
- [ ] Generate unique discriminators (12-bit values)
- [ ] Generate unique setup PIN codes (27-bit/8-digit)
- [ ] Create QR code generation for commissioning

#### 1.4: Implement Credential Storage
- [ ] Add credential storage module (extend nvs.rs)
- [ ] Implement secure read/write for private keys
- [ ] Store DAC, PAI, attestation key
- [ ] Store discriminator and setup PIN
- [ ] Store vendor/product ID, serial number

#### 1.5: Replace Test Credentials in Code
- [ ] Create DeviceDetails struct (replaces TEST_DEV_DET)
- [ ] Create CommissioningData struct (replaces TEST_DEV_COMM)
- [ ] Create DeviceAttestation struct (replaces TEST_DEV_ATT)
- [ ] Load credentials from NVS at startup
- [ ] Update EmbassyWifiMatterStack initialization
- [ ] Remove imports of test:: module

#### 1.6: Test & Validate
- [ ] Verify device commissions with unique credentials
- [ ] Test with Google Home
- [ ] Test with Alexa
- [ ] Verify QR code commissioning works
- [ ] Check that different devices have different identities

**Estimated Effort:** 5-8 days  
**Dependencies:** None  
**Risk:** Medium (depends on rs-matter API understanding)

---

### Phase 2: ColorControl Cluster - Basic HS Commands 🔜 NEXT
**Goal:** Implement mandatory Hue/Saturation commands per Matter spec

**Priority:** HIGH (required for spec compliance)

#### 2.1: Add Missing Attribute Reads
- [ ] Implement `current_hue()` - return current hue value from state
- [ ] Implement `current_saturation()` - return current saturation
- [ ] Add `remaining_time()` attribute (return 0 for now)
- [ ] Fix `color_mode()` to return correct value (0 for HS, not XY)
- [ ] Verify attribute reads via Matter controller

#### 2.2: Implement MoveToHue Command (0x00)
- [ ] Parse command parameters (hue, direction, transitionTime)
- [ ] Implement immediate mode (transitionTime = 0)
- [ ] Handle direction parameter (shortest, longest, up, down)
- [ ] Update current_hue state
- [ ] Update LED output
- [ ] Set color_mode to 0 (HS)
- [ ] Test with controller

#### 2.3: Implement MoveToSaturation Command (0x03)
- [ ] Parse command parameters (saturation, transitionTime)
- [ ] Implement immediate mode
- [ ] Update current_saturation state
- [ ] Update LED output
- [ ] Test with controller

#### 2.4: Implement StepHue Command (0x02)
- [ ] Parse parameters (stepMode, stepSize, transitionTime)
- [ ] Calculate new hue value (current ± stepSize)
- [ ] Handle wrapping at 0/254 boundary
- [ ] Update state and LED
- [ ] Test stepping up and down

#### 2.5: Implement StepSaturation Command (0x05)
- [ ] Parse parameters (stepMode, stepSize, transitionTime)
- [ ] Calculate new saturation (clamp at 0/254)
- [ ] Update state and LED
- [ ] Test stepping up and down

#### 2.6: Fix MoveToHueAndSaturation (0x06)
- [ ] Already implemented but needs OptionsMask/OptionsOverride support
- [ ] Add proper parameter validation
- [ ] Handle direction parameter if needed
- [ ] Verify transition time is ignored (document limitation)

#### 2.7: Implement Options Attribute Write
- [ ] Add `set_options()` handler
- [ ] Store options value in LightController
- [ ] Implement ExecuteIfOff behavior
- [ ] Test that commands respect ExecuteIfOff setting

#### 2.8: Add OptionsMask/OptionsOverride Support
- [ ] Create helper function for effective options calculation
- [ ] Apply to all color commands
- [ ] Test override behavior

**Estimated Effort:** 4-6 days  
**Dependencies:** None (can start after Phase 1.5)  
**Risk:** Low (straightforward implementation)

---

### Phase 3: Transition Timing System 🔜 UPCOMING
**Goal:** Add smooth color transitions over time using Embassy timers

**Priority:** HIGH (required for proper spec compliance and UX)

#### 3.1: Design Transition Architecture
- [ ] Design async transition task pattern
- [ ] Plan how to handle concurrent transitions
- [ ] Define interpolation strategy (linear, ease-in-out?)
- [ ] Determine update frequency (10Hz? 20Hz?)

#### 3.2: Implement Transition Infrastructure
- [ ] Create TransitionState struct (start, end, duration, elapsed)
- [ ] Add transition task spawner
- [ ] Implement linear interpolation for hue values
- [ ] Implement linear interpolation for saturation
- [ ] Handle color wheel wrapping for hue

#### 3.3: Implement RemainingTime Tracking
- [ ] Add remaining_time field to LightController
- [ ] Update during transitions
- [ ] Return from remaining_time() attribute
- [ ] Clear when transition completes

#### 3.4: Add Transition Support to Commands
- [ ] Update MoveToHue to use transitions
- [ ] Update MoveToSaturation to use transitions
- [ ] Update MoveToHueAndSaturation to use transitions
- [ ] Update StepHue to use transitions
- [ ] Update StepSaturation to use transitions

#### 3.5: Implement Move Commands (Continuous)
- [ ] Implement MoveHue (0x01) - continuous hue change
- [ ] Implement MoveSaturation (0x04) - continuous saturation change
- [ ] Handle stop mode (moveMode = 0)
- [ ] Handle rate parameter (units per second)
- [ ] Test continuous movement and stopping

#### 3.6: Test Transition System
- [ ] Test smooth transitions at various durations
- [ ] Verify RemainingTime accuracy
- [ ] Test interrupting one transition with another
- [ ] Test that LED updates smoothly
- [ ] Measure CPU usage during transitions

**Estimated Effort:** 5-7 days  
**Dependencies:** Phase 2 completed  
**Risk:** Medium (async coordination complexity)

---

### Phase 4: Enhanced Hue Support 🔜 UPCOMING
**Goal:** Implement 16-bit precision hue for smoother color control

**Priority:** MEDIUM (nice to have, enhances UX)

#### 4.1: Add Enhanced Hue State
- [ ] Add `enhanced_hue` field to LightController (u16)
- [ ] Keep both 8-bit and 16-bit hue in sync
- [ ] Update hsv_to_rgb to use enhanced precision
- [ ] Implement `enhanced_current_hue()` attribute

#### 4.2: Implement EnhancedMoveToHue (0x40)
- [ ] Parse 16-bit hue parameter
- [ ] Implement with transition support
- [ ] Update enhanced_current_hue
- [ ] Set enhanced_color_mode to 3

#### 4.3: Implement EnhancedMoveToHueAndSaturation (0x43)
- [ ] Parse 16-bit hue + 8-bit saturation
- [ ] Implement with transition support
- [ ] Update state and LED
- [ ] Test smooth color transitions

#### 4.4: Implement EnhancedMoveHue (0x41)
- [ ] Implement continuous 16-bit hue movement
- [ ] Handle rate parameter (16-bit units per second)
- [ ] Test smooth continuous transitions

#### 4.5: Implement EnhancedStepHue (0x42)
- [ ] Parse 16-bit step size
- [ ] Implement stepping with transitions
- [ ] Test stepping at various sizes

#### 4.6: Update Feature Reporting
- [ ] Set feature bit 0x02 (Enhanced Hue) in ColorCapabilities
- [ ] Update featureMap attribute
- [ ] Verify controllers recognize enhanced hue support

**Estimated Effort:** 3-4 days  
**Dependencies:** Phase 3 completed (needs transitions)  
**Risk:** Low (extension of existing code)

---

### Phase 5: Color State Persistence 🔜 UPCOMING
**Goal:** Persist color settings across reboots

**Priority:** MEDIUM (good UX, not spec-required)

#### 5.1: Design State Persistence
- [ ] Define NVS keys for color state
- [ ] Decide when to persist (on change? periodic? on shutdown?)
- [ ] Handle persistence failures gracefully

#### 5.2: Implement State Save
- [ ] Save current_hue to NVS on change
- [ ] Save current_saturation to NVS on change
- [ ] Save enhanced_current_hue to NVS
- [ ] Add debouncing (don't save on every change)

#### 5.3: Implement State Restore
- [ ] Load color state from NVS at startup
- [ ] Apply loaded state to LED
- [ ] Handle missing/corrupted state gracefully
- [ ] Use sensible defaults if no saved state

#### 5.4: Test Persistence
- [ ] Set colors, reboot, verify colors restored
- [ ] Test with various color values
- [ ] Test first boot (no saved state)
- [ ] Test corrupted NVS recovery

**Estimated Effort:** 2-3 days  
**Dependencies:** Phase 2 completed  
**Risk:** Low

---

### Phase 6: StopMoveStep Command 🔜 UPCOMING
**Goal:** Allow canceling ongoing transitions and continuous movements

**Priority:** MEDIUM (required if Move commands implemented)

#### 6.1: Implement StopMoveStep (0x47)
- [ ] Add transition cancellation mechanism
- [ ] Stop any ongoing Move operations
- [ ] Clear RemainingTime
- [ ] Leave color at current value
- [ ] Handle OptionsMask/OptionsOverride

#### 6.2: Test Stop Functionality
- [ ] Start MoveHue, then StopMoveStep, verify stops
- [ ] Start MoveToHue transition, then stop, verify
- [ ] Test with various move/transition combinations

**Estimated Effort:** 1-2 days  
**Dependencies:** Phase 3 completed  
**Risk:** Low

---

### Phase 7: Optional Features (Lower Priority) 📋 BACKLOG

These are optional features that may or may not be implemented:

#### 7.1: XY Color Space Support
**Priority:** LOW (can convert from HS client-side)

- [ ] Add current_x, current_y fields (u16)
- [ ] Implement XY to RGB conversion
- [ ] Implement MoveToColor (0x07)
- [ ] Implement MoveColor (0x08)
- [ ] Implement StepColor (0x09)
- [ ] Update color_mode to support mode 1 (XY)
- [ ] Set feature bit 0x08

**Estimated Effort:** 4-5 days  
**Value:** Medium (broader compatibility)

#### 7.2: Color Temperature Support
**Priority:** LOW (RGB LED can't accurately represent)

- [ ] Add color_temperature_mireds field (u16)
- [ ] Implement Mireds to RGB conversion
- [ ] Define physical min/max mireds (153-500 for 2000K-6500K)
- [ ] Implement MoveToColorTemperature (0x0A)
- [ ] Implement MoveColorTemperature (0x4B)
- [ ] Implement StepColorTemperature (0x4C)
- [ ] Update color_mode to support mode 2 (CT)
- [ ] Set feature bit 0x10

**Estimated Effort:** 4-5 days  
**Value:** Low (hardware limitation)

#### 7.3: Color Loop
**Priority:** LOW (cool demo feature)

- [ ] Add color_loop state fields (active, direction, time, start_hue, stored_hue)
- [ ] Implement ColorLoopSet (0x44)
- [ ] Create async color loop task
- [ ] Implement loop timing and direction
- [ ] Handle activation/deactivation
- [ ] Set feature bit 0x04

**Estimated Effort:** 3-4 days  
**Value:** Low (nice-to-have)

#### 7.4: Additional Clusters
**Priority:** VERY LOW (out of scope for now)

- Level Control cluster (brightness control)
- Groups cluster (control multiple devices)
- Scenes cluster (save/recall settings)
- Identify cluster (blink for identification)
- OTA Update cluster (firmware updates)

---

## Testing & Validation Checklist

After each phase, verify:

- [ ] Code compiles without warnings
- [ ] Device boots and commissions successfully
- [ ] No panics or unwrap failures in common use cases
- [ ] Commands work via Google Home (if available)
- [ ] Commands work via Alexa (if available)
- [ ] Commands work via chip-tool CLI (if available)
- [ ] Memory usage is acceptable (no heap exhaustion)
- [ ] LED responds correctly to commands
- [ ] Matter subscriptions work (attribute updates pushed to controller)
- [ ] Persistence works across reboots

## Development Workflow

1. **Before Starting a Task:**
   - Review this roadmap
   - Confirm task is marked for current phase
   - Check dependencies are completed
   - Read relevant specification section

2. **During Development:**
   - Make minimal, surgical changes
   - Add logging for debugging
   - Handle errors gracefully (avoid unwrap)
   - Update this roadmap as you go

3. **After Completing a Task:**
   - Mark task as complete [x]
   - Test thoroughly
   - Commit changes with descriptive message
   - Update this roadmap's status

4. **After Completing a Phase:**
   - Update phase status (✅ COMPLETED)
   - Add completion date
   - Run full test suite
   - Consider creating git tag

## Current Status Summary

| Phase | Status | Progress | Blocker |
|-------|--------|----------|---------|
| Phase 0: Foundation | ✅ COMPLETED | 100% | - |
| Phase 1: Credentials | 🔄 IN PROGRESS | 0% | Need rs-matter API research |
| Phase 2: HS Commands | 🔜 NEXT | 0% | Phase 1 completion |
| Phase 3: Transitions | 📋 PLANNED | 0% | Phase 2 completion |
| Phase 4: Enhanced Hue | 📋 PLANNED | 0% | Phase 3 completion |
| Phase 5: Persistence | 📋 PLANNED | 0% | Phase 2 completion |
| Phase 6: StopMoveStep | 📋 PLANNED | 0% | Phase 3 completion |
| Phase 7: Optional | 📋 BACKLOG | 0% | Earlier phases |

## Success Criteria

### Minimum Viable Product (MVP)
- ✅ Phase 0: Documentation
- ⬜ Phase 1: Unique device credentials
- ⬜ Phase 2: Basic HS commands (no transitions)
- ⬜ Phase 5: Color persistence

**Target:** Functional device with unique identity that meets basic spec requirements.

### Full Specification Compliance
- ⬜ All MVP items
- ⬜ Phase 3: Smooth transitions
- ⬜ Phase 4: Enhanced hue support
- ⬜ Phase 6: StopMoveStep command

**Target:** Device fully compliant with claimed features (HS + EHUE).

### Production Ready
- ⬜ All compliance items
- ⬜ Manufacturing process documented
- ⬜ QR code generation automated
- ⬜ Error handling throughout
- ⬜ Comprehensive testing
- ⬜ User documentation

**Target:** Device ready for commercial deployment.

## Known Limitations & Trade-offs

1. **Hardware:** Single RGB LED limits color temperature accuracy
2. **Memory:** ESP32-C6 RAM constraints limit transition smoothness
3. **Dependencies:** Forked rs-matter libraries (not upstream)
4. **Certification:** No plans for official Matter certification (costs ~$25K)
5. **Features:** XY and CT support may be skipped due to hardware limitations

## Notes & Decisions

### 2024-10-02
- Created initial roadmap
- Prioritized credentials over features (security first)
- Decided to implement HS + Enhanced Hue features only
- May skip XY and CT features due to hardware limitations
- Will implement smooth transitions for better UX

## Questions & Open Issues

1. **Credential Generation:** Need to research rs-matter's exact API for custom credentials
2. **PAI Certificate:** Where to obtain PAI for development? Use self-signed?
3. **Vendor ID:** Continue using test VID 0xFFF1 or register real VID?
4. **Manufacturing:** How to provision credentials at scale?
5. **Transition Rate:** What update frequency for smooth transitions? (10Hz, 20Hz, 50Hz?)
6. **Color Space:** Should we implement XY conversion even if not claiming feature?

## Resources

- [Matter Specification 1.2](https://csa-iot.org/developer-resource/specifications-download-request/)
- [rs-matter Documentation](https://github.com/project-chip/rs-matter)
- [ESP32-C6 Technical Reference](https://www.espressif.com/sites/default/files/documentation/esp32-c6_technical_reference_manual_en.pdf)
- [CSA Certification Process](https://csa-iot.org/certification/)

---

**Last Updated:** 2024-10-02  
**Next Review:** After Phase 1 completion  
**Maintained By:** Project Team
