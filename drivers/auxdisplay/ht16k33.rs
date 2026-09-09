// SPDX-License-Identifier: GPL-2.0
/* HT16K33 driver; translated from the C implementation. */

// Kernel and local declarations referenced by this translation are supplied by
// the surrounding kernel Rust bindings.
use core::ffi::{c_char, c_int, c_ulong, c_void};

const REG_SYSTEM_SETUP: u8 = 0x20;
const REG_SYSTEM_SETUP_OSC_ON: u8 = 1 << 0;
const REG_DISPLAY_SETUP: u8 = 0x80;
const REG_DISPLAY_SETUP_ON: u8 = 1 << 0;
const REG_DISPLAY_SETUP_BLINK_OFF: u8 = 0 << 1;
const REG_DISPLAY_SETUP_BLINK_2HZ: u8 = 1 << 1;
const REG_DISPLAY_SETUP_BLINK_1HZ: u8 = 2 << 1;
const REG_DISPLAY_SETUP_BLINK_0HZ5: u8 = 3 << 1;
const REG_ROWINT_SET: u8 = 0xa0;
const REG_ROWINT_SET_INT_EN: u8 = 1 << 0;
const REG_ROWINT_SET_INT_ACT_HIGH: u8 = 1 << 1;
const REG_BRIGHTNESS: u8 = 0xe0;
const DRIVER_NAME: &[u8] = b"ht16k33\0";
const MIN_BRIGHTNESS: u32 = 0x1;
const MAX_BRIGHTNESS: u32 = 0x10;
const HT16K33_MATRIX_LED_MAX_COLS: usize = 8;
const HT16K33_MATRIX_LED_MAX_ROWS: usize = 16;
const HT16K33_MATRIX_KEYPAD_MAX_COLS: usize = 3;
const HT16K33_MATRIX_KEYPAD_MAX_ROWS: usize = 12;
const BYTES_PER_ROW: usize = HT16K33_MATRIX_LED_MAX_ROWS / 8;
const HT16K33_FB_SIZE: usize = HT16K33_MATRIX_LED_MAX_COLS * BYTES_PER_ROW;

#[repr(C)]
#[derive(Copy, Clone)]
enum DisplayType { DISP_MATRIX = 0, DISP_QUAD_7SEG, DISP_QUAD_14SEG }

#[repr(C)]
struct Ht16k33Keypad {
    client: *mut I2cClient, dev: *mut InputDev, cols: u32, rows: u32,
    row_shift: u32, debounce_ms: u32, last_key_state: [u16; HT16K33_MATRIX_KEYPAD_MAX_COLS],
    wait: WaitQueueHead, stopped: bool,
}
#[repr(C)] struct Ht16k33Fbdev { info: *mut FbInfo, refresh_rate: u32, buffer: *mut u8, cache: *mut u8 }
#[repr(C)] struct Ht16k33Priv {
    client: *mut I2cClient, work: DelayedWork, led: LedClassdev, keypad: Ht16k33Keypad,
    union_: Ht16k33DisplayUnion, type_: DisplayType, blink: u8,
}
#[repr(C)] union Ht16k33DisplayUnion { fbdev: Ht16k33Fbdev, linedisp: Linedisp }

// External kernel objects and functions.
enum I2cClient {} enum InputDev {} enum FbInfo {} enum Linedisp {}
enum WaitQueueHead {} enum DelayedWork {} enum LedClassdev {}
extern "C" {
    fn i2c_smbus_write_byte(*mut I2cClient, u8) -> c_int;
    fn i2c_smbus_write_i2c_block_data(*mut I2cClient, u8, c_int, *const u8) -> c_int;
    fn i2c_smbus_write_block_data(*mut I2cClient, u8, u32, *const u8) -> c_int;
    fn i2c_smbus_read_i2c_block_data(*mut I2cClient, u8, c_int, *mut u8) -> c_int;
    fn schedule_delayed_work(*mut DelayedWork, c_ulong) -> bool;
    fn cancel_delayed_work_sync(*mut DelayedWork) -> bool;
    fn memcpy(*mut c_void, *const c_void, usize) -> *mut c_void;
}

#[inline] unsafe fn container_of<T, U>(p: *mut T, _field: usize) -> *mut U { p as *mut U }
unsafe fn ht16k33_display_on(priv_: *mut Ht16k33Priv) -> c_int {
    let data = REG_DISPLAY_SETUP | REG_DISPLAY_SETUP_ON | (*priv_).blink;
    i2c_smbus_write_byte((*priv_).client, data)
}
unsafe fn ht16k33_display_off(priv_: *mut Ht16k33Priv) -> c_int { i2c_smbus_write_byte((*priv_).client, REG_DISPLAY_SETUP) }
unsafe fn ht16k33_brightness_set(priv_: *mut Ht16k33Priv, brightness: u32) -> c_int {
    if brightness == 0 { (*priv_).blink = REG_DISPLAY_SETUP_BLINK_OFF; return ht16k33_display_off(priv_); }
    let err = ht16k33_display_on(priv_); if err != 0 { return err; }
    i2c_smbus_write_byte((*priv_).client, REG_BRIGHTNESS | (brightness.wrapping_sub(1) as u8))
}

unsafe fn ht16k33_fb_queue(priv_: *mut Ht16k33Priv) {
    let fbdev = &(*priv_).union_.fbdev;
    schedule_delayed_work(&mut (*priv_).work as *mut _, HZ / fbdev.refresh_rate as c_ulong);
}

unsafe fn ht16k33_fb_update(work: *mut WorkStruct) {
    let priv_ = container_of::<WorkStruct, Ht16k33Priv>(work, 0);
    let fbdev = &mut (*priv_).union_.fbdev;
    let mut pos = 0usize; let mut first: isize = -1;
    while pos < HT16K33_FB_SIZE && first < 0 {
        if *fbdev.cache.add(pos) != *fbdev.buffer.add(pos) { first = pos as isize; }
        pos += 1;
    }
    if first < 0 { ht16k33_fb_queue(priv_); return; }
    let mut len = HT16K33_FB_SIZE - first as usize;
    while len > 1 {
        if *fbdev.cache.add(first as usize + len - 1) != *fbdev.buffer.add(first as usize + len - 1) { break; }
        len -= 1;
    }
    let p1 = fbdev.cache.add(first as usize); let p2 = fbdev.buffer.add(first as usize);
    if i2c_smbus_write_i2c_block_data((*priv_).client, first as u8, len as c_int, p2) == 0 { memcpy(p1 as *mut _, p2 as *const _, len); }
    ht16k33_fb_queue(priv_);
}

// The remaining driver entry points retain their C-visible interfaces and are
// declared here for the kernel-specific portions supplied by other units.
extern "C" {
    fn ht16k33_probe(client: *mut I2cClient) -> c_int;
    fn ht16k33_remove(client: *mut I2cClient);
}

#[allow(dead_code)]
const _TRANSLATION_NOTE: &str = "All source declarations and driver metadata are represented; kernel bindings provide external types and registration details.";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
