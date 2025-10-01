# Phase 2 Final: State Persistence Implementation

**Goal:** Persist light state (on/off, color settings) to NVS so it survives reboots.

**Date Started:** 2025-10-01
**Status:** In Progress

---

## Problem Statement

**Current Behavior:**
- Device boots with default state (off, hue=0, saturation=0, warm white temp)
- Any user-configured colors/settings lost on reboot
- Matter stack state persists (commissioning data) but light state does not
- Users must reconfigure their preferred color after every power cycle

**User Impact:**
- Frustrating UX - "Why does my light always reset to white?"
- Inconsistent with commercial smart lights (which remember state)
- Makes device feel unreliable or broken

---

## Design Decision 1: Serialization Format

**Question:** How should light state be serialized for NVS storage?

**Options Considered:**

###Option A: One NVS key per field (9 separate keys)
```rust
// Pros: Simple to implement, easy to read individual fields
// Cons: 9 flash writes per state change, wastes NVS space
Key 100: on_off (1 byte)
Key 101: hue (1 byte)
Key 102: saturation (1 byte)
... etc (9 keys total)
```

### Option B: Single binary blob (1 key, packed format)
```rust
// Pros: Single flash write, efficient, atomic updates
// Cons: More complex serialization
Key 100: [version|on_off|hue|sat|x|y|temp|enh|mode|opts|checksum]
```

### Option C: JSON/CBOR (1 key, structured format)
```rust
// Pros: Self-describing, extensible, debuggable
// Cons: Heavier dependencies, larger size, overkill for embedded
```

**DECISION: Option B - Single Binary Blob**

**Rationale:**
1. **Atomic updates**: Single flash write = state can't be partially corrupted
2. **Efficient**: ~15 bytes total (fits well within NVS block)
3. **Fast**: No parsing overhead, direct memory layout
4. **Appropriate for embedded**: No heap allocations, fixed size
5. **NVS-friendly**: Matches buffer size patterns (use 32-byte buffer)

---

## Design Decision 2: Binary Layout

**Chosen Format** (15 bytes total):

```
Offset | Field                      | Size | Type | Notes
-------|----------------------------|------|------|------------------
0      | VERSION                    | 1    | u8   | Format version (0x01)
1      | on_off                     | 1    | bool | 0=off, 1=on
2      | hue                        | 1    | u8   | 0-254
3      | saturation                 | 1    | u8   | 0-254
4-5    | current_x                  | 2    | u16  | Little-endian
6-7    | current_y                  | 2    | u16  | Little-endian
8-9    | color_temperature_mireds   | 2    | u16  | Little-endian
10-11  | enhanced_hue               | 2    | u16  | Little-endian
12     | color_mode_state           | 1    | u8   | Enum as u8
13     | options                    | 1    | u8   | Bitmap
14     | checksum                   | 1    | u8   | XOR of bytes 0-13

TOTAL: 15 bytes
```

**Version Byte Purpose:**
- Allows format evolution
- Version 0x01 = current format
- Future versions can add fields or change layout
- Restore code can handle multiple versions

**Checksum Purpose:**
- Simple XOR checksum for basic integrity
- Detects single-bit errors or corruption
- Fast to compute (no crypto dependencies)
- On mismatch: log error, use defaults

**Why Little-Endian:**
- ESP32-C6 is little-endian (RISC-V)
- Native byte order = no swapping needed
- Standard for embedded systems

---

## Design Decision 3: NVS Key Allocation

**Question:** Which NVS key number should light state use?

**Analysis:**
- Matter stack uses NVS for commissioning data (unknown key range)
- Need to avoid conflicts
- rs-matter KvBlobStore trait uses u16 keys (0-65535 range)

**Investigation:**
Looking at Matter stack code, commissioning data likely uses low keys (0-99).

**DECISION: Use key 100 for light state**

**Rationale:**
1. High enough to avoid Matter stack keys (likely 0-99)
2. Low enough to be easy to remember/document
3. Leaves room for future expansion (keys 101-199)
4. Round number (easy to spot in logs)

**Future Key Allocation Plan:**
```
Key Range | Purpose
----------|------------------
0-99      | Matter stack (commissioning, fabrics, etc.)
100       | Light state (this implementation)
101-109   | Reserved for future light features (scenes, presets)
110-199   | Application-specific data
200+      | User-defined
```

---

## Design Decision 4: When to Save State

**Question:** When should state be written to NVS?

**Options:**

### Option A: Immediate save after every change
```rust
// Pros: Simple, state always current
// Cons: Flash wear - could write 1000s of times per session
fn set_hue(&self, hue: u8) {
    *self.hue.borrow_mut() = hue;
    self.save_state();  // ⚠️ Flash write every time
}
```

### Option B: Debounced save (delay N seconds)
```rust
// Pros: Reduces flash writes dramatically
// Cons: More complex, requires timer/task
fn set_hue(&self, hue: u8) {
    *self.hue.borrow_mut() = hue;
    self.schedule_save();  // Saves after 2s of inactivity
}
```

### Option C: Save on specific triggers only
```rust
// Pros: Minimal flash writes
// Cons: State lost if user doesn't trigger save event
// Save only on: device sleep, explicit save command, etc.
```

**DECISION: Option A for Phase 2, Plan for Option B in Phase 3**

**Rationale:**
1. **Phase 2 Goal**: Feature completeness, not optimization
2. **Simplicity**: Option A is trivial to implement correctly
3. **Flash Lifetime**: Modern flash is rated for 100K+ cycles
   - Even 10 changes/day = 27 years of life
   - Commercial lights do immediate saves
4. **Debouncing Complexity**: Requires async task management
   - Better suited for Phase 3 (production readiness)
   - Needs proper timer infrastructure
5. **User Expectation**: State saved immediately feels more reliable

**Phase 3 Optimization:**
- Add debouncing when implementing transition timing system
- Both features share timer/task infrastructure
- Measure actual flash write frequency first

---

## Design Decision 5: Error Handling Strategy

**Question:** What should happen if restore fails?

**Scenarios:**
1. No saved state (first boot)
2. Corrupted data (checksum mismatch)
3. Wrong version
4. NVS read error

**DECISION: Graceful Degradation with Logging**

**Behavior:**
```rust
match restore_state(&mut nvs) {
    Ok(Some(state)) => {
        log::info!("Restored light state from NVS");
        state  // Use restored state
    }
    Ok(None) => {
        log::info!("No saved state, using defaults (first boot)");
        LightState::default()
    }
    Err(e) => {
        log::error!("Failed to restore state: {:?}, using defaults", e);
        LightState::default()
    }
}
```

**Rationale:**
1. **Fail-safe**: Device always boots (never crashes from bad NVS)
2. **Observable**: Logging helps debug NVS issues
3. **User-friendly**: Light works even if persistence broken
4. **Matches commercial behavior**: Lights default to on/white if state lost

**No Retry Logic:**
- Read errors unlikely to be transient
- Retrying could delay boot significantly
- Better to boot with defaults than hang

---

## Design Decision 6: State Structure

**Question:** Should we extract state into a separate struct?

**Option A: Inline in LightController** (current)
```rust
impl LightController {
    fn save_state(&self, nvs: &mut Nvs) { ... }
}
```

**Option B: Separate LightState struct**
```rust
struct LightState {
    on_off: bool,
    hue: u8,
    // ... etc
}

impl LightState {
    fn serialize(&self) -> [u8; 15] { ... }
    fn deserialize(bytes: &[u8]) -> Result<Self> { ... }
}
```

**DECISION: Option B - Separate Struct**

**Rationale:**
1. **Testability**: Can unit test serialization without LightController
2. **Clarity**: Clear separation of data vs behavior
3. **Future-proof**: Easy to add versioning/migration logic
4. **Standard pattern**: Matches Rust best practices
5. **No cost**: Zero-copy in practice (stack allocated)

---

## Implementation Plan

### Step 1: Add LightState struct (src/lib.rs)
```rust
/// Serializable light state for NVS persistence
#[derive(Debug, Clone, Copy)]
struct LightState {
    on_off: bool,
    hue: u8,
    saturation: u8,
    current_x: u16,
    current_y: u16,
    color_temperature_mireds: u16,
    enhanced_hue: u16,
    color_mode_state: u8,  // ColorMode as u8
    options: u8,
}
```

### Step 2: Add serialization methods
```rust
impl LightState {
    const VERSION: u8 = 0x01;
    const NVS_KEY: u16 = 100;
    const SERIALIZED_SIZE: usize = 15;

    fn serialize(&self) -> [u8; 15] { ... }
    fn deserialize(bytes: &[u8]) -> Result<Self, &'static str> { ... }
    fn checksum(bytes: &[u8; 14]) -> u8 { ... }
}
```

### Step 3: Add save/restore to LightController
```rust
impl LightController {
    fn get_state(&self) -> LightState { ... }
    fn apply_state(&self, state: LightState) { ... }
    fn save_state(&self, nvs: &mut Nvs) -> Result<()> { ... }
    fn restore_state(nvs: &mut Nvs) -> Option<LightState> { ... }
}
```

### Step 4: Update LightController::new()
- Accept optional NVS reference
- Attempt to restore state
- Fall back to defaults if restore fails

### Step 5: Call save_state() after mutations
- After every on/off change
- After every color change
- On any state modification

### Step 6: Testing
- Unit test serialize/deserialize round-trip
- Test checksum validation
- Test version handling
- Manual hardware test (set color, reboot, verify)

---

## Testing Strategy

### Unit Tests
```rust
#[test]
fn test_serialize_deserialize_roundtrip() { ... }

#[test]
fn test_checksum_validation() { ... }

#[test]
fn test_version_mismatch() { ... }

#[test]
fn test_corrupted_data() { ... }
```

### Integration Test
- Set color to red (hue=0, sat=254)
- Reboot device
- Verify LED is red on boot
- Check logs for "Restored light state from NVS"

### Edge Cases
- First boot (no saved state)
- Corrupted NVS (flip random bits)
- Full NVS partition
- Power loss during write

---

## Success Criteria

✅ Phase 2 Final Complete When:
- [ ] LightState struct implemented
- [ ] Serialize/deserialize with checksums working
- [ ] save_state() called after all state changes
- [ ] restore_state() works in LightController::new()
- [ ] Unit tests pass
- [ ] Hardware test: color persists across reboot
- [ ] Documentation updated (IMPLEMENTATION_LOG.md, IMPROVEMENTS.md)
- [ ] Code compiles with no warnings

✅ User-Visible Outcome:
- Light remembers color/state after power cycle
- No need to reconfigure after reboot
- Consistent with commercial smart light behavior
