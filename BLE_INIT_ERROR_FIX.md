# BLE HCI Initialization Error Fix

## Problem
Intermittent `BleHost(Hci(Invalid HCI Command Parameters))` error on startup.

## Root Cause
BLE/WiFi coexistence initialization timing issue. The BLE controller may not be fully ready when the Matter stack attempts to send HCI commands during commissioning setup.

## Proposed Solutions

### Option 1: Add initialization delay (Quick fix)
- Add `Timer::after(Duration::from_millis(100))` after `esp_wifi::init()` at main.rs:79
- Allows BLE controller to stabilize before Matter stack starts
- **Pros**: Simple, minimal code change
- **Cons**: Adds fixed delay to every boot

### Option 2: Add retry logic (Robust fix)
- Wrap `run_coex()` call in retry loop with exponential backoff
- Catch BLE initialization errors and retry up to 3 times
- Log failures for debugging
- **Pros**: Handles errors gracefully without always delaying
- **Cons**: More complex implementation

### Option 3: Use non-coex mode (Alternative approach)
- Switch from `run_coex()` to sequential `run()`
- Requires increasing `BUMP_SIZE` from 20,000 to ~35,000 bytes
- May not work with Alexa (per comments at main.rs:10)
- **Pros**: Avoids coexistence timing issues entirely
- **Cons**: Higher memory usage, potential Alexa compatibility issues

### Option 4: Update esp-wifi version (Long-term fix)
- Check if newer esp-wifi releases (>0.15.0) fix this issue
- Review changelog for BLE initialization improvements
- **Pros**: May fix underlying issue
- **Cons**: Requires testing, may introduce other changes

## Implementation Order
1. Start with Option 1 (initialization delay)
2. If issue persists, implement Option 2 (retry logic)
3. Consider Option 4 as long-term solution
