# Testing Strategy for ESP32-C6 Matter Light

**Challenge:** This is a `no_std` embedded project with hardware dependencies, making traditional Rust unit tests difficult.

**Status:** Phase 3 - Testing infrastructure in progress

---

## Testing Challenges

### Why Standard `cargo test` Doesn't Work

1. **no_std Environment**
   - Project uses `#![no_std]`
   - Standard test framework requires std library
   - Cannot use assert macros that allocate

2. **Hardware Dependencies**
   - `esp-hal` crates require actual ESP32-C6 hardware
   - Embassy runtime needs embedded executor
   - Flash storage (NVS) requires real flash partition

3. **Async Dependencies**
   - Matter stack is async (embassy-executor)
   - Test harness doesn't support async without std

4. **Cargo.toml Configuration**
   ```toml
   [lib]
   test = false    # Tests disabled for embedded target
   ```

---

## Testing Approach: Multi-Layer Strategy

### Layer 1: Manual Hardware Testing ⭐ (Current Focus)
**Tool:** `HARDWARE_VALIDATION_CHECKLIST.md`

**What:**
- Flash device and test all functionality
- Validate color handlers
- Test persistence
- Verify error handling

**Coverage:**
- Integration testing: 100%
- End-to-end workflows
- Hardware-specific behavior

**Status:** ✅ Checklist created, ready for use

---

### Layer 2: Logic Verification (Manual Code Review)
**Tool:** Code inspection + mathematical verification

#### Testable Components (Pure Logic)

**2.1: LightState Serialization**
```rust
// What to verify manually:
let state = LightState {
    on_off: true,
    hue: 85,           // Green
    saturation: 254,   // Fully saturated
    current_x: 0x6000,
    current_y: 0x6000,
    color_temperature_mireds: 370,
    enhanced_hue: 21845,  // 85 * 65535 / 254
    color_mode_state: 0,  // HSV mode
    options: 0,
};

let bytes = state.serialize();
// Expected: [0x01, 0x01, 0x55, 0xFE, 0x00,0x60, 0x00,0x60, 0x72,0x01, 0x55,0x55, 0x00, 0x00, checksum]

let restored = LightState::deserialize(&bytes)?;
assert_eq!(state, restored);  // Would verify if we had tests
```

**Manual Verification:**
- [ ] Serialize a known state
- [ ] Inspect bytes match expected layout
- [ ] Deserialize and compare
- [ ] Flip a bit, verify checksum catches it

**2.2: Checksum Calculation**
```rust
// XOR checksum verification
let data = [0x01, 0x01, 0x55, 0xFE, ...];  // 14 bytes
let checksum = LightState::compute_checksum(&data);
// Expected: XOR of all bytes

// Verify: checksum ^ all_bytes == 0
let verify = data.iter().fold(checksum, |acc, &b| acc ^ b);
assert_eq!(verify, 0);  // Should be 0 if checksum correct
```

**Manual Verification:**
- [ ] Compute checksum of known data
- [ ] Verify XOR property: sum ^ checksum == 0
- [ ] Test with corrupted data

**2.3: Color Conversion Math**
```rust
// HSV to RGB (h=0, s=254, v=255) should be RED
// h=0 → 0 degrees → Red region
// s=254 → Fully saturated
// Expected: RGB(255, 0, 0) or close

// HSV to RGB (h=85, s=254, v=255) should be GREEN
// h=85 → ~120 degrees → Green region
// Expected: RGB(0, 255, 0) or close

// Color Temp: 370 mireds ≈ 2700K → Warm white
// Expected: RGB(255, ~200, ~140) yellowish white
```

**Manual Verification:**
- [ ] Test known hue values (0=red, 85=green, 170=blue)
- [ ] Verify saturation 0 = white
- [ ] Test color temperature extremes
- [ ] Visual inspection on hardware LED

---

### Layer 3: Future Unit Tests (Phase 4)

**Option A: Extract Pure Logic** (Recommended for Phase 4)
Refactor to separate pure functions from hardware:

```rust
// New file: src/color/math.rs (pure, no hardware)
pub fn hsv_to_rgb(h: u8, s: u8, v: u8) -> (u8, u8, u8) {
    // Pure math, no dependencies
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hsv_red() {
        let (r, g, b) = hsv_to_rgb(0, 254, 255);
        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 0);
    }
}
```

**Effort:** 4-6 hours to refactor + test
**Value:** Regression prevention, confidence in math
**When:** Phase 4 (after code organization)

**Option B: Embedded Test Framework**
Use `defmt-test` or similar embedded test harness:

```toml
[dev-dependencies]
defmt-test = "0.3"

[[test]]
name = "serial tests"
harness = false
```

**Effort:** 6-8 hours setup + learning
**Value:** Real unit tests on target hardware
**When:** Phase 4 if needed

**Option C: Host-Side Test Binary**
Create a separate std binary that tests just the logic:

```
tests/
└── host_tests/
    ├── Cargo.toml  # with std, separate project
    ├── lightstate_tests.rs
    └── color_math_tests.rs
```

**Effort:** 3-4 hours
**Value:** Standard Rust tests for pure logic
**When:** Phase 4 if refactoring happens

---

## Current Testing Status (Phase 3)

### ✅ Available Testing Tools
1. **Hardware Validation Checklist** - Comprehensive manual testing guide
2. **Logging & Observability** - Extensive debug logs for monitoring
3. **Error Handling** - Graceful fallbacks prevent crashes
4. **Checksums** - Data integrity validation built-in

### ❌ Not Available Yet
1. Automated unit tests
2. Regression test suite
3. CI/CD integration
4. Code coverage metrics

### 🔄 In Progress
1. Manual verification of serialization logic
2. Hardware validation preparations
3. Test infrastructure design

---

## Recommended Testing Workflow (Phase 3)

### Step 1: Manual Serialization Verification
```rust
// Add temporary test function to main.rs
fn verify_serialization() {
    let state = LightState {
        on_off: true,
        hue: 85,
        saturation: 254,
        // ... fill in all fields
    };

    let bytes = state.serialize();
    info!("Serialized: {:?}", bytes);

    match LightState::deserialize(&bytes) {
        Ok(restored) => {
            info!("Deserialization OK");
            info!("on_off: {} == {}", state.on_off, restored.on_off);
            info!("hue: {} == {}", state.hue, restored.hue);
            // ... check all fields
        }
        Err(e) => info!("Deserialization FAILED: {}", e),
    }
}
```

**How to Test:**
1. Add function to main.rs
2. Call once at boot
3. Flash device
4. Check serial output
5. Verify all fields match

### Step 2: Corruption Test
```rust
fn test_checksum() {
    let state = LightState { /* ... */ };
    let mut bytes = state.serialize();

    info!("Valid checksum: {:?}", LightState::deserialize(&bytes));

    // Corrupt a byte
    bytes[5] ^= 0xFF;  // Flip all bits

    info!("Corrupted: {:?}", LightState::deserialize(&bytes));
    // Should get Err("Checksum mismatch")
}
```

### Step 3: Hardware Validation
Use `HARDWARE_VALIDATION_CHECKLIST.md`:
- Test all color handlers
- Test persistence
- Test error recovery
- Document bugs found

---

## Test Coverage Goals

### Phase 3 (Current)
- Hardware validation: 100% (manual)
- Serialization logic: Verified (manual)
- Color conversions: Visually verified (hardware)
- Error handling: Verified (manual testing)

### Phase 4 (Future)
- Unit tests: >70% coverage (automated)
- Integration tests: >50% coverage
- Regression tests: Critical paths
- CI/CD: Automated testing on commits

---

## Known Gaps (To Address in Phase 4)

1. **No Automated Tests**
   - Risk: Regressions not caught automatically
   - Mitigation: Comprehensive hardware checklist

2. **No Color Math Tests**
   - Risk: HSV/RGB conversions might be wrong
   - Mitigation: Visual verification on hardware LED

3. **No Persistence Tests**
   - Risk: Serialization bugs
   - Mitigation: Manual verification + checksums

4. **No CI/CD**
   - Risk: Compilation breaks not caught early
   - Mitigation: Manual `cargo check` before commits

---

## Summary

**Current Strategy:** Manual hardware validation + visual inspection

**Why:** no_std embedded constraints make traditional testing difficult

**When to Add Unit Tests:** Phase 4, after refactoring to separate pure logic

**Confidence Level:** Moderate (checksums + extensive logging + manual testing)

**Next Steps:**
1. ✅ Use hardware validation checklist
2. ✅ Manual serialization verification
3. ⏳ Document bugs found
4. ⏳ Phase 4: Automated testing infrastructure

---

**Testing is critical, but must be pragmatic for embedded no_std projects.**
