# Hardware Validation Checklist - Phase 2 Complete

**Purpose:** Validate ~450 LOC of Phase 2 work on actual ESP32-C6 hardware before proceeding to Phase 3.

**Date:** 2025-10-01 (after Phase 2 completion)
**Hardware:** ESP32-C6 with WS2812 LED
**Firmware:** Phase 2 Complete (persistence + 16/20 color handlers)

---

## Pre-Flash Checklist

- [ ] Code compiles successfully (`cargo check --release`)
- [ ] No compilation errors or warnings
- [ ] Review recent changes (git diff)
- [ ] Backup current working firmware (if any)

---

## Flash & Boot Testing

### Basic Boot
- [ ] Flash device: `cargo run --release`
- [ ] Device boots without panic
- [ ] Serial output shows initialization logs
- [ ] "NVS partition size = X" logged
- [ ] No error messages during boot
- [ ] LED initializes (off by default)

### Expected Boot Log Pattern:
```
Starting...
NVS partition size = 20480
Matter stack initialized
WiFi/BLE starting...
Commissioning ready
```

---

## Phase 2a: Basic Color Control Testing

### On/Off Commands (Cluster 0x0006)
- [ ] **Turn On** - LED lights up
- [ ] **Turn Off** - LED turns off
- [ ] Verify on/off state persists in memory

### Hue Control (8-bit)
- [ ] **Set Hue = 0** - Red color
- [ ] **Set Hue = 85** - Green color (Matter hue: 0-254)
- [ ] **Set Hue = 170** - Blue color
- [ ] **Set Hue = 254** - Red (wraps around)
- [ ] Verify smooth hue changes

### Saturation Control
- [ ] **Set Saturation = 0** - White (no color)
- [ ] **Set Saturation = 127** - Pastel color
- [ ] **Set Saturation = 254** - Fully saturated
- [ ] Combine with hue changes

### Color Temperature Mode
- [ ] **Set Color Temperature = 153 mireds** - Cool white (~6500K)
- [ ] **Set Color Temperature = 370 mireds** - Warm white (~2700K)
- [ ] **Set Color Temperature = 500 mireds** - Very warm (~2000K)
- [ ] Verify color mode switches to ColorTemperature

### Move-To Commands
- [ ] **MoveToHue** - Hue changes instantly
- [ ] **MoveToSaturation** - Saturation changes instantly
- [ ] **MoveToColorTemperature** - Color temp changes instantly
- [ ] **MoveToHueAndSaturation** - Both change together

### Step Commands
- [ ] **StepHue (Up)** - Hue increments
- [ ] **StepHue (Down)** - Hue decrements
- [ ] **StepSaturation (Up)** - Saturation increases
- [ ] **StepSaturation (Down)** - Saturation decreases
- [ ] **StepColorTemperature (Up/Down)** - Temp changes

---

## Phase 2b: Enhanced Hue & Attribute Readers

### Enhanced Hue (16-bit)
- [ ] **EnhancedMoveToHue(32767)** - Mid-range 16-bit hue
- [ ] **EnhancedStepHue(1000, Up)** - Fine increment
- [ ] **EnhancedStepHue(1000, Down)** - Fine decrement
- [ ] **EnhancedMoveToHueAndSaturation** - Combined command
- [ ] Verify 16-bit → 8-bit hue synchronization

### Attribute Readers (via Matter Controller)
If using a Matter controller (Google Home/Alexa/Apple Home):
- [ ] Current hue displays correctly
- [ ] Current saturation displays correctly
- [ ] Color temperature displays correctly
- [ ] Enhanced current hue displays correctly
- [ ] Attributes update when changed
- [ ] Color mode attribute correct (0=HSV, 2=Temp)

### Stubbed Commands (Should Not Crash)
- [ ] **MoveHue** - Logs warning, doesn't crash
- [ ] **MoveSaturation** - Logs warning, doesn't crash
- [ ] **MoveToColor (XY)** - Logs warning, doesn't crash
- [ ] **ColorLoopSet** - Logs warning, doesn't crash
- [ ] **StopMoveStep** - Logs warning, doesn't crash
- [ ] Verify all stubs return Ok (no panics)

---

## Phase 2 Final: State Persistence Testing

### First Boot (No Saved State)
- [ ] Flash fresh firmware
- [ ] Device boots with defaults
- [ ] LED is off
- [ ] Hue = 0, Saturation = 0
- [ ] Log shows "No saved state" or similar

### Manual Save Test
**Note:** Current Phase 2 requires manual save API calls. This test requires adding temporary code to main.rs or using a test command.

**Test Plan:**
1. Set color to: Red (hue=0, sat=254)
2. Call `light_controller.save_to_nvs(&mut nvs).await`
3. Verify log: "Saving light state to NVS (key 100)"
4. Verify log: "Light state saved successfully"

### Persistence Validation
- [ ] Set LED to specific color (e.g., Green: hue=85, sat=254)
- [ ] Manually trigger save (via code modification or command)
- [ ] **Power cycle device** (unplug/replug or reset button)
- [ ] Device boots and LED is GREEN
- [ ] Log shows "Restored light state from NVS"
- [ ] Log shows correct hue and saturation values

### Corruption Recovery Test
**Advanced:** Flash corruption simulation
- [ ] Save valid state
- [ ] Manually corrupt NVS (flip bits in flash)
- [ ] Reboot device
- [ ] Device boots with defaults (doesn't crash)
- [ ] Log shows "Failed to deserialize state: Checksum mismatch"
- [ ] LED works normally with defaults

---

## Matter Controller Integration (If Available)

### Commissioning
- [ ] Device advertises over BLE
- [ ] Commissions successfully to Matter network
- [ ] Appears in controller app (Google Home/Alexa/etc.)
- [ ] Device name/type shows as "Color Light"

### Control via App
- [ ] Turn on/off via app
- [ ] Change color via color picker
- [ ] Change brightness (if supported)
- [ ] Change color temperature via slider
- [ ] All changes reflected on LED

### Persistence with Matter
- [ ] Set color via Matter controller
- [ ] Reboot device
- [ ] **IF auto-save implemented:** Color persists
- [ ] **IF manual save:** Color resets to default (expected)

---

## Error Handling & Edge Cases

### NVS Error Scenarios
- [ ] Full NVS partition (write many keys) - graceful error
- [ ] Read non-existent key - returns None
- [ ] Write to read-only flash - error logged

### Invalid Commands
- [ ] Hue = 255 (invalid per spec) - clamped to 254?
- [ ] Saturation = 255 - clamped to 254?
- [ ] Color temp out of range - clamped?

### Serial Log Monitoring
- [ ] No unexpected panics
- [ ] No error loops
- [ ] No stack overflows
- [ ] Memory usage stable

---

## Performance & Stability

### Long-Running Test
- [ ] Leave device powered for 30+ minutes
- [ ] No crashes or reboots
- [ ] LED remains responsive
- [ ] Memory not leaking (check logs)

### Rapid Changes
- [ ] Send 100 rapid color changes
- [ ] Device handles all commands
- [ ] LED updates smoothly
- [ ] No buffer overflows or crashes

### Flash Wear (Long-term)
- [ ] **Optional:** Script 1000 save cycles
- [ ] All saves succeed
- [ ] No flash errors
- [ ] Device still works after 1000 writes

---

## Bugs Found (Track Here)

### Bug Template
**Bug #:** [Number]
**Severity:** [Critical/High/Medium/Low]
**Component:** [Which part of code]
**Description:** [What went wrong]
**Steps to Reproduce:**
1.
2.
3.
**Expected:**
**Actual:**
**Fix Required:**

---

## Sign-Off

### Validation Complete
- [ ] All critical tests passed
- [ ] All blocking bugs fixed
- [ ] Device stable for production testing
- [ ] Ready for Phase 3 development

**Date Validated:** __________
**Tested By:** __________
**Hardware Revision:** __________
**Firmware Version/Commit:** __________

**Notes:**

---

## Next Steps After Validation

### If All Tests Pass:
1. Document any minor issues found
2. Commit Phase 2 complete work
3. Begin Phase 3: Production Readiness
   - Auto-save integration
   - Testing infrastructure
   - Code organization
   - Continuous commands implementation

### If Critical Bugs Found:
1. Document bugs in detail
2. Prioritize fixes
3. Fix critical bugs
4. Re-validate after fixes
5. Then proceed to Phase 3

---

**Remember:** This is the first hardware test of ~450 LOC from Phase 2. Expect to find issues - that's the point of validation!
