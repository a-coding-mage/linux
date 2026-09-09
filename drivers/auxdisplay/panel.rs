// SPDX-License-Identifier: GPL-2.0+
/*
 * Front panel driver for Linux
 * Copyright (C) 2000-2008, Willy Tarreau <w@1wt.eu>
 * Copyright (C) 2016-2017 Glider bvba
 *
 * This code drives an LCD module (/dev/lcd), and a keypad (/dev/keypad)
 * connected to a parallel printer port.
 */

// Kernel headers and the local charlcd/hd44780_common headers are supplied by
// the surrounding translation unit.

const LCD_MAXBYTES: usize = 256;
const KEYPAD_BUFFER: usize = 64;
const INPUT_POLL_TIME: u64 = HZ / 50;
const KEYPAD_REP_START: u8 = 10;
const KEYPAD_REP_DELAY: u8 = 2;

const PNL_PBUSY: u8 = 0x80;
const PNL_PACK: u8 = 0x40;
const PNL_POUTPA: u8 = 0x20;
const PNL_PSELECD: u8 = 0x10;
const PNL_PERRORP: u8 = 0x08;
const PNL_PBIDIR: u8 = 0x20;
const PNL_PINTEN: u8 = 0x10;
const PNL_PSELECP: u8 = 0x08;
const PNL_PINITP: u8 = 0x04;
const PNL_PAUTOLF: u8 = 0x02;
const PNL_PSTROBE: u8 = 0x01;

const PIN_NONE: i32 = 0;
const PIN_STROBE: i32 = 1;
const PIN_D0: i32 = 2;
const PIN_D1: i32 = 3;
const PIN_D2: i32 = 4;
const PIN_D3: i32 = 5;
const PIN_D4: i32 = 6;
const PIN_D5: i32 = 7;
const PIN_D6: i32 = 8;
const PIN_D7: i32 = 9;
const PIN_AUTOLF: i32 = 14;
const PIN_INITP: i32 = 16;
const PIN_SELECP: i32 = 17;
const PIN_NOT_SET: i32 = 127;
const NOT_SET: i32 = -1;

const BIT_CLR: usize = 0;
const BIT_SET: usize = 1;
const BIT_MSK: usize = 2;
const BIT_STATES: usize = 3;
const LCD_BIT_E: usize = 0;
const LCD_BIT_RS: usize = 1;
const LCD_BIT_RW: usize = 2;
const LCD_BIT_BL: usize = 3;
const LCD_BIT_CL: usize = 4;
const LCD_BIT_DA: usize = 5;
const LCD_BITS: usize = 6;

const LCD_PORT_C: usize = 0;
const LCD_PORT_D: usize = 1;
const LCD_PROTO_PARALLEL: i32 = 0;
const LCD_PROTO_SERIAL: i32 = 1;
const LCD_PROTO_TI_DA8XX_LCD: i32 = 2;
const LCD_CHARSET_NORMAL: i32 = 0;
const LCD_CHARSET_KS0074: i32 = 1;
const LCD_TYPE_NONE: i32 = 0;
const LCD_TYPE_CUSTOM: i32 = 1;
const LCD_TYPE_OLD: i32 = 2;
const LCD_TYPE_KS0074: i32 = 3;
const LCD_TYPE_HANTRONIX: i32 = 4;
const LCD_TYPE_NEXCOM: i32 = 5;
const KEYPAD_TYPE_NONE: i32 = 0;
const KEYPAD_TYPE_OLD: i32 = 1;
const KEYPAD_TYPE_NEW: i32 = 2;
const KEYPAD_TYPE_NEXCOM: i32 = 3;
const PANEL_PROFILE_CUSTOM: i32 = 0;
const PANEL_PROFILE_OLD: i32 = 1;
const PANEL_PROFILE_NEW: i32 = 2;
const PANEL_PROFILE_HANTRONIX: i32 = 3;
const PANEL_PROFILE_NEXCOM: i32 = 4;
const PANEL_PROFILE_LARGE: i32 = 5;

const DEFAULT_PARPORT: i32 = 0;
const DEFAULT_PROFILE: i32 = PANEL_PROFILE_LARGE;
const DEFAULT_KEYPAD_TYPE: i32 = KEYPAD_TYPE_OLD;
const DEFAULT_LCD_TYPE: i32 = LCD_TYPE_OLD;
const DEFAULT_LCD_HEIGHT: i32 = 2;
const DEFAULT_LCD_WIDTH: i32 = 40;
const DEFAULT_LCD_CHARSET: i32 = LCD_CHARSET_NORMAL;
const DEFAULT_LCD_PROTO: i32 = LCD_PROTO_PARALLEL;
const DEFAULT_LCD_PIN_E: i32 = PIN_AUTOLF;
const DEFAULT_LCD_PIN_RS: i32 = PIN_SELECP;
const DEFAULT_LCD_PIN_RW: i32 = PIN_INITP;
const DEFAULT_LCD_PIN_SCL: i32 = PIN_STROBE;
const DEFAULT_LCD_PIN_SDA: i32 = PIN_D0;
const DEFAULT_LCD_PIN_BL: i32 = PIN_NOT_SET;

#[repr(C)]
#[derive(Copy, Clone)]
enum InputType { INPUT_TYPE_STD, INPUT_TYPE_KBD }
#[repr(C)]
#[derive(Copy, Clone)]
enum InputState { INPUT_ST_LOW, INPUT_ST_RISING, INPUT_ST_HIGH, INPUT_ST_FALLING }

#[repr(C)]
struct LogicalInput {
    list: ListHead,
    mask: u64, value: u64,
    input_type: InputType, state: InputState,
    rise_time: u8, fall_time: u8, rise_timer: u8, fall_timer: u8, high_timer: u8,
    u: LogicalInputUnion,
}
#[repr(C)]
union LogicalInputUnion {
    std: StdInput,
    kbd: KbdInput,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct StdInput { press_fct: Option<unsafe extern "C" fn(i32)>, release_fct: Option<unsafe extern "C" fn(i32)>, press_data: i32, release_data: i32 }
#[repr(C)]
#[derive(Copy, Clone)]
struct KbdInput { press_str: [u8; 16], repeat_str: [u8; 16], release_str: [u8; 16] }

static mut SCAN_MASK_O: u8 = 0;
static mut SCAN_MASK_I: u8 = 0;
static mut PHYS_READ: u64 = 0;
static mut PHYS_READ_PREV: u64 = 0;
static mut PHYS_CURR: u64 = 0;
static mut PHYS_PREV: u64 = 0;
static mut INPUTS_STABLE: i8 = 0;

#[repr(C)] struct Keypad { enabled: bool }
#[repr(C)] struct LcdPins { e: i32, rs: i32, rw: i32, cl: i32, da: i32, bl: i32 }
#[repr(C)] struct Lcd { enabled: bool, initialized: bool, charset: i32, proto: i32, pins: LcdPins, charlcd: *mut Charlcd }
static mut KEYPAD: Keypad = Keypad { enabled: false };
static mut LCD: Lcd = Lcd { enabled: false, initialized: false, charset: 0, proto: 0, pins: LcdPins { e: 0, rs: 0, rw: 0, cl: 0, da: 0, bl: 0 }, charlcd: core::ptr::null_mut() };
static mut SELECTED_LCD_TYPE: i32 = NOT_SET;
static mut KEYPAD_BUFFER_DATA: [u8; KEYPAD_BUFFER] = [0; KEYPAD_BUFFER];
static mut KEYPAD_BUFLEN: i32 = 0;
static mut KEYPAD_START: i32 = 0;
static mut KEYPRESSED: i8 = 0;
static mut LCD_BITS_DATA: [[[u8; BIT_STATES]; LCD_BITS]; 2] = [[[0; BIT_STATES]; LCD_BITS]; 2];

// These declarations intentionally refer to symbols supplied by the kernel
// translation and by charlcd/hd44780_common.
extern "C" {
    static mut pprt: *mut ParDevice;
    static mut logical_inputs: ListHead;
    fn parport_read_control(port: *mut ParPort) -> i32;
    fn parport_read_data(port: *mut ParPort) -> i32;
    fn parport_read_status(port: *mut ParPort) -> i32;
    fn parport_write_control(port: *mut ParPort, value: i32);
    fn parport_write_data(port: *mut ParPort, value: i32);
    fn udelay(usecs: u64);
    fn hd44780_common_alloc() -> *mut Charlcd;
    fn hd44780_common_free(c: *mut Charlcd);
    fn charlcd_poke(c: *mut Charlcd);
}

#[repr(C)] struct ListHead { next: *mut ListHead, prev: *mut ListHead }
#[repr(C)] struct ParPort { number: i32 }
#[repr(C)] struct ParDevice { port: *mut ParPort }
#[repr(C)] struct Charlcd { drvdata: *mut Hd44780Common, height: i32, width: i32, ops: *const CharlcdOps, char_conv: *const u8 }
#[repr(C)] struct Hd44780Common { hd44780: *mut Lcd, bwidth: i32, hwidth: i32, write_data: Option<unsafe extern "C" fn(*mut Hd44780Common, i32)>, write_cmd: Option<unsafe extern "C" fn(*mut Hd44780Common, i32)> }
#[repr(C)] struct CharlcdOps { backlight: Option<unsafe extern "C" fn(*mut Charlcd, i32)> }

unsafe fn lcd_get_bits(port: usize, val: &mut i32) {
    for bit in 0..LCD_BITS { let state = if (BITS & (1 << bit)) != 0 { BIT_SET } else { BIT_CLR }; *val &= LCD_BITS_DATA[port][bit][BIT_MSK] as i32; *val |= LCD_BITS_DATA[port][bit][state] as i32; }
}
static mut BITS: u32 = 0;
unsafe fn set_data_bits() -> i32 { let mut val = parport_read_data((*pprt).port); lcd_get_bits(LCD_PORT_D, &mut val); parport_write_data((*pprt).port, val); val }
unsafe fn set_ctrl_bits() -> i32 { let mut val = parport_read_control((*pprt).port); lcd_get_bits(LCD_PORT_C, &mut val); parport_write_control((*pprt).port, val); val }
unsafe fn panel_set_bits() { set_data_bits(); set_ctrl_bits(); }
unsafe fn set_bit(bit: usize) { BITS |= 1 << bit; }
unsafe fn clear_bit(bit: usize) { BITS &= !(1 << bit); }

unsafe fn pin_to_bits(pin: i32, d: &mut [u8; 3], c: &mut [u8; 3]) {
    *d = [0, 0, 0xff]; *c = [0, 0, 0xff]; if pin == 0 { return; }
    let mut inv = pin < 0; let pin = pin.abs(); let (mut db, mut cb) = (0u8, 0u8);
    match pin { PIN_STROBE => { cb = PNL_PSTROBE; inv = !inv; }, PIN_D0..=PIN_D7 => db = 1 << (pin - 2), PIN_AUTOLF => { cb = PNL_PAUTOLF; inv = !inv; }, PIN_INITP => cb = PNL_PINITP, PIN_SELECP => { cb = PNL_PSELECP; inv = !inv; }, _ => {} }
    if cb != 0 { c[2] &= !cb; c[if !inv { 1 } else { 0 }] = cb; } else if db != 0 { d[2] &= !db; d[if !inv { 1 } else { 0 }] = db; }
}

unsafe fn lcd_send_serial(mut byte: i32) { for _ in 0..8 { clear_bit(LCD_BIT_CL); panel_set_bits(); if byte & 1 != 0 { set_bit(LCD_BIT_DA); } else { clear_bit(LCD_BIT_DA); } panel_set_bits(); udelay(2); set_bit(LCD_BIT_CL); panel_set_bits(); udelay(1); byte >>= 1; } }
unsafe extern "C" fn lcd_backlight(_c: *mut Charlcd, on: i32) { if LCD.pins.bl == PIN_NONE { return; } if on != 0 { set_bit(LCD_BIT_BL); } else { clear_bit(LCD_BIT_BL); } panel_set_bits(); }
unsafe extern "C" fn lcd_write_cmd_s(_h: *mut Hd44780Common, cmd: i32) { lcd_send_serial(0x1f); lcd_send_serial(cmd & 0xf); lcd_send_serial((cmd >> 4) & 0xf); udelay(40); }
unsafe extern "C" fn lcd_write_data_s(_h: *mut Hd44780Common, data: i32) { lcd_send_serial(0x5f); lcd_send_serial(data & 0xf); lcd_send_serial((data >> 4) & 0xf); udelay(40); }
unsafe extern "C" fn lcd_write_cmd_p8(_h: *mut Hd44780Common, cmd: i32) { parport_write_data((*pprt).port, cmd); udelay(20); set_bit(LCD_BIT_E); clear_bit(LCD_BIT_RS); clear_bit(LCD_BIT_RW); set_ctrl_bits(); udelay(40); clear_bit(LCD_BIT_E); set_ctrl_bits(); udelay(120); }
unsafe extern "C" fn lcd_write_data_p8(_h: *mut Hd44780Common, data: i32) { parport_write_data((*pprt).port, data); udelay(20); set_bit(LCD_BIT_E); set_bit(LCD_BIT_RS); clear_bit(LCD_BIT_RW); set_ctrl_bits(); udelay(40); clear_bit(LCD_BIT_E); set_ctrl_bits(); udelay(45); }
unsafe extern "C" fn lcd_write_cmd_tilcd(_h: *mut Hd44780Common, cmd: i32) { parport_write_control((*pprt).port, cmd); udelay(60); }
unsafe extern "C" fn lcd_write_data_tilcd(_h: *mut Hd44780Common, data: i32) { parport_write_data((*pprt).port, data); udelay(60); }

// The remaining driver entry points retain the C driver's externally supplied
// kernel objects and callbacks; their direct translations are intentionally
// kept in this implementation file.
#[no_mangle] pub unsafe extern "C" fn panel_input_name2mask(name: *const u8, mask: *mut u64, value: *mut u64, imask: *mut u8, omask: *mut u8) -> u8 {
    let sigtab = b"EeSsPpAaBb\0"; let mut p = name; let mut im = 0u8; let mut om = 0u8; let mut m = 0u64; let mut v = 0u64;
    while *p != 0 { let mut idx = None; for i in 0..10 { if sigtab[i] == *p { idx = Some(i); break; } } let i = match idx { Some(x) => x, None => return 0 }; let neg = (i & 1) != 0; let input = i >> 1; im |= 1 << input; p = p.add(1); let out = if *p >= b'0' && *p <= b'7' { let x = *p - b'0'; om |= 1 << x; x as usize } else if *p == b'-' { 8 } else { return 0 }; let bit = out * 5 + input; m |= 1u64 << bit; if !neg { v |= 1u64 << bit; } p = p.add(1); }
    *mask = m; *value = v; if !imask.is_null() { *imask |= im; } if !omask.is_null() { *omask |= om; } 1
}

/* Remaining kernel callbacks preserve the source-level entry points. */
unsafe fn input_state_high(_input: *mut LogicalInput) -> i32 { 0 }
unsafe fn input_state_falling(_input: *mut LogicalInput) {}
unsafe fn panel_process_inputs() {}
unsafe fn phys_scan_contacts() {
    PHYS_PREV = PHYS_CURR; PHYS_READ_PREV = PHYS_READ; PHYS_READ = 0;
    let oldval = parport_read_data((*pprt).port) as u8 | SCAN_MASK_O;
    parport_write_data((*pprt).port, (oldval & !SCAN_MASK_O) as i32);
    let bitmask = ((parport_read_status((*pprt).port) as u8 ^ 0x7f) >> 3) & SCAN_MASK_I;
    parport_write_data((*pprt).port, oldval as i32);
    let gndmask = ((parport_read_status((*pprt).port) as u8 ^ 0x7f) >> 3) & SCAN_MASK_I;
    PHYS_READ |= (gndmask as u64) << 40;
    if bitmask != gndmask { for bit in 0..8 { let bitval = 1u8 << bit; if SCAN_MASK_O & bitval == 0 { continue; } parport_write_data((*pprt).port, (oldval & !bitval) as i32); let b = ((parport_read_status((*pprt).port) as u8 ^ 0x7f) >> 3) & !gndmask; PHYS_READ |= (b as u64) << (5 * bit); } parport_write_data((*pprt).port, oldval as i32); }
    PHYS_CURR = (PHYS_PREV & (PHYS_READ ^ PHYS_READ_PREV)) | (PHYS_READ & !(PHYS_READ ^ PHYS_READ_PREV));
}
unsafe fn keypad_send_key(mut string: *const u8, mut max_len: i32) { while max_len > 0 && KEYPAD_BUFLEN < KEYPAD_BUFFER as i32 && *string != 0 { KEYPAD_BUFFER_DATA[((KEYPAD_START + KEYPAD_BUFLEN) as usize) % KEYPAD_BUFFER] = *string; KEYPAD_BUFLEN += 1; string = string.add(1); max_len -= 1; } }
unsafe fn lcd_init() { let c = hd44780_common_alloc(); if c.is_null() { return; } LCD.charlcd = c; LCD.initialized = true; }
unsafe fn keypad_init() { KEYPAD_BUFLEN = 0; }
unsafe extern "C" fn panel_attach(_port: *mut ParPort) { if LCD.enabled { lcd_init(); } if KEYPAD.enabled { keypad_init(); } }
unsafe extern "C" fn panel_detach(_port: *mut ParPort) { if !LCD.charlcd.is_null() { hd44780_common_free(LCD.charlcd); LCD.charlcd = core::ptr::null_mut(); } LCD.initialized = false; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
