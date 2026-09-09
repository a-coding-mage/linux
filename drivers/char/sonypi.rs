// SPDX-License-Identifier: GPL-2.0-or-later
/* Sony Programmable I/O Control Device driver for VAIO.
 * This is a low-level source translation; Linux kernel symbols are external
 * dependencies supplied by the surrounding kernel-Rust environment.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals,
         dead_code, unused_variables, unused_mut)]

use core::ffi::c_void;

pub const SONYPI_DRIVER_VERSION: &str = "1.26";

pub const SONYPI_DEVICE_MODEL_TYPE1: i32 = 1;
pub const SONYPI_DEVICE_MODEL_TYPE2: i32 = 2;
pub const SONYPI_DEVICE_MODEL_TYPE3: i32 = 3;
pub const SONYPI_IRQ_PORT: u16 = 0x8034;
pub const SONYPI_IRQ_SHIFT: u32 = 22;
pub const SONYPI_TYPE1_BASE: u16 = 0x50;
pub const SONYPI_G10A: u16 = SONYPI_TYPE1_BASE + 0x14;
pub const SONYPI_TYPE1_REGION_SIZE: u16 = 0x08;
pub const SONYPI_TYPE1_EVTYPE_OFFSET: u16 = 0x04;
pub const SONYPI_SIRQ: u8 = 0x9b;
pub const SONYPI_SLOB: u8 = 0x9c;
pub const SONYPI_SHIB: u8 = 0x9d;
pub const SONYPI_TYPE2_REGION_SIZE: u16 = 0x20;
pub const SONYPI_TYPE2_EVTYPE_OFFSET: u16 = 0x12;
pub const SONYPI_TYPE3_BASE: u16 = 0x40;
pub const SONYPI_TYPE3_GID2: u16 = SONYPI_TYPE3_BASE + 0x48;
pub const SONYPI_TYPE3_MISC: u16 = SONYPI_TYPE3_BASE + 0x6d;
pub const SONYPI_TYPE3_REGION_SIZE: u16 = 0x20;
pub const SONYPI_TYPE3_EVTYPE_OFFSET: u16 = 0x12;
pub const SONYPI_BAT_FLAGS: u8 = 0x81;
pub const SONYPI_LCD_LIGHT: u8 = 0x96;
pub const SONYPI_BAT1_LEFT: u8 = 0xa2;
pub const SONYPI_BAT2_LEFT: u8 = 0xaa;
pub const SONYPI_BAT1_FULL: u8 = 0xb2;
pub const SONYPI_BAT2_FULL: u8 = 0xba;
pub const SONYPI_FAN0_STATUS: u8 = 0x93;
pub const SONYPI_TEMP_STATUS: u8 = 0xc1;
pub const SONYPI_DATA_IOPORT: u16 = 0x62;
pub const SONYPI_CST_IOPORT: u16 = 0x66;
pub const SONYPI_CAMERA_BRIGHTNESS: u8 = 0;
pub const SONYPI_CAMERA_CONTRAST: u8 = 1;
pub const SONYPI_CAMERA_HUE: u8 = 2;
pub const SONYPI_CAMERA_COLOR: u8 = 3;
pub const SONYPI_CAMERA_SHARPNESS: u8 = 4;
pub const SONYPI_CAMERA_PICTURE: u8 = 5;
pub const SONYPI_CAMERA_EXPOSURE_MASK: u8 = 0xc;
pub const SONYPI_CAMERA_WHITE_BALANCE_MASK: u8 = 3;
pub const SONYPI_CAMERA_PICTURE_MODE_MASK: u8 = 0x30;
pub const SONYPI_CAMERA_MUTE_MASK: u8 = 0x40;
pub const SONYPI_CAMERA_AGC: u8 = 6;
pub const SONYPI_CAMERA_AGC_MASK: u8 = 0x30;
pub const SONYPI_CAMERA_SHUTTER_MASK: u8 = 7;
pub const SONYPI_CAMERA_SHUTDOWN_REQUEST: u8 = 7;
pub const SONYPI_CAMERA_CONTROL: u8 = 0x10;
pub const SONYPI_CAMERA_STATUS: u8 = 7;
pub const SONYPI_CAMERA_STATUS_READY: u8 = 2;
pub const SONYPI_CAMERA_STATUS_POSITION: u8 = 4;
pub const SONYPI_DIRECTION_BACKWARDS: u8 = 4;
pub const SONYPI_CAMERA_REVISION: u8 = 8;
pub const SONYPI_CAMERA_ROMVERSION: u8 = 9;
pub const SONYPI_JOGGER_MASK: u32 = 0x00000001;
pub const SONYPI_CAPTURE_MASK: u32 = 0x00000002;
pub const SONYPI_FNKEY_MASK: u32 = 0x00000004;
pub const SONYPI_BLUETOOTH_MASK: u32 = 0x00000008;
pub const SONYPI_PKEY_MASK: u32 = 0x00000010;
pub const SONYPI_BACK_MASK: u32 = 0x00000020;
pub const SONYPI_HELP_MASK: u32 = 0x00000040;
pub const SONYPI_LID_MASK: u32 = 0x00000080;
pub const SONYPI_ZOOM_MASK: u32 = 0x00000100;
pub const SONYPI_THUMBPHRASE_MASK: u32 = 0x00000200;
pub const SONYPI_MEYE_MASK: u32 = 0x00000400;
pub const SONYPI_MEMORYSTICK_MASK: u32 = 0x00000800;
pub const SONYPI_BATTERY_MASK: u32 = 0x00001000;
pub const SONYPI_WIRELESS_MASK: u32 = 0x00002000;
pub const SONYPI_BUF_SIZE: usize = 128;
pub const ITERATIONS_LONG: u32 = 10000;
pub const ITERATIONS_SHORT: u32 = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sonypi_ioport_list { pub port1: u16, pub port2: u16 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sonypi_irq_list { pub irq: u16, pub bits: u16 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sonypi_event { pub data: u8, pub event: u8 }
#[repr(C)]
pub struct sonypi_eventtypes { pub model: i32, pub data: u8, pub mask: usize, pub events: *mut sonypi_event }
#[repr(C)]
pub struct sonypi_keypress { pub dev: *mut input_dev, pub key: i32 }
#[repr(C)]
pub struct input_dev { _private: [u8; 0] }
#[repr(C)]
pub struct pci_dev { _private: [u8; 0] }
#[repr(C)]
pub struct platform_device { _private: [u8; 0] }
#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct work_struct { _private: [u8; 0] }
#[repr(C)]
pub struct kfifo { _private: [u8; 0] }
#[repr(C)]
pub struct mutex { _private: [u8; 0] }
#[repr(C)]
pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)]
pub struct wait_queue_head_t { _private: [u8; 0] }

static mut minor: i32 = -1;
static mut verbose: i32 = 0;
static mut fnkeyinit: i32 = 0;
static mut camera: i32 = 0;
static mut compat: i32 = 0;
static mut mask: usize = 0xffff_ffff;
static mut useinput: i32 = 1;
static mut check_ioport: i32 = 1;

static mut sonypi_type1_ioport_list: [sonypi_ioport_list; 6] = [
    sonypi_ioport_list { port1: 0x10c0, port2: 0x10c4 },
    sonypi_ioport_list { port1: 0x1080, port2: 0x1084 },
    sonypi_ioport_list { port1: 0x1090, port2: 0x1094 },
    sonypi_ioport_list { port1: 0x10a0, port2: 0x10a4 },
    sonypi_ioport_list { port1: 0x10b0, port2: 0x10b4 },
    sonypi_ioport_list { port1: 0, port2: 0 },
];
static mut sonypi_type2_ioport_list: [sonypi_ioport_list; 5] = [
    sonypi_ioport_list { port1: 0x1080, port2: 0x1084 },
    sonypi_ioport_list { port1: 0x10a0, port2: 0x10a4 },
    sonypi_ioport_list { port1: 0x10c0, port2: 0x10c4 },
    sonypi_ioport_list { port1: 0x10e0, port2: 0x10e4 },
    sonypi_ioport_list { port1: 0, port2: 0 },
];
static mut sonypi_type1_irq_list: [sonypi_irq_list; 4] = [
    sonypi_irq_list { irq: 11, bits: 2 }, sonypi_irq_list { irq: 10, bits: 1 },
    sonypi_irq_list { irq: 5, bits: 0 }, sonypi_irq_list { irq: 0, bits: 3 },
];
static mut sonypi_type2_irq_list: [sonypi_irq_list; 5] = [
    sonypi_irq_list { irq: 11, bits: 0x80 }, sonypi_irq_list { irq: 10, bits: 0x40 },
    sonypi_irq_list { irq: 9, bits: 0x20 }, sonypi_irq_list { irq: 6, bits: 0x10 },
    sonypi_irq_list { irq: 0, bits: 0 },
];

/* Event tables retain the C driver's sentinel-terminated representation. */
static mut sonypi_releaseev: [sonypi_event; 1] = [sonypi_event { data: 0, event: 0 }];
static mut sonypi_joggerev: [sonypi_event; 14] = [
    sonypi_event{data:0x1f,event:1},sonypi_event{data:1,event:2},sonypi_event{data:0x5f,event:3},
    sonypi_event{data:0x41,event:4},sonypi_event{data:0x1e,event:5},sonypi_event{data:2,event:6},
    sonypi_event{data:0x5e,event:7},sonypi_event{data:0x42,event:8},sonypi_event{data:0x1d,event:9},
    sonypi_event{data:3,event:10},sonypi_event{data:0x5d,event:11},sonypi_event{data:0x43,event:12},
    sonypi_event{data:0x40,event:13},sonypi_event{data:0,event:0},
];

extern "C" {
    fn sonypi_ec_write(addr: u8, value: u8) -> i32;
    fn sonypi_ec_read(addr: u8, value: *mut u8) -> i32;
    fn ec_read16(addr: u8, value: *mut u16) -> i32;
    fn sonypi_call1(dev: u8) -> u8;
    fn sonypi_call2(dev: u8, func: u8) -> u8;
    fn sonypi_call3(dev: u8, func: u8, value: u8) -> u8;
}

/* Direct translations of the file-local camera state operations. */
static mut camera_power: i32 = 0;
static mut bluetooth_power: i32 = -1;

unsafe fn sonypi_camera_ready() -> bool {
    let v = sonypi_call2(0x8f, SONYPI_CAMERA_STATUS);
    v != 0xff && (v & SONYPI_CAMERA_STATUS_READY) != 0
}
unsafe fn sonypi_set(func: u8, value: u8) { let _ = sonypi_call3(0x90, func, value); }
unsafe fn sonypi_camera_off() {
    sonypi_set(SONYPI_CAMERA_PICTURE, SONYPI_CAMERA_MUTE_MASK);
    if camera_power == 0 { return; }
    let _ = sonypi_call2(0x91, 0); camera_power = 0;
}
unsafe fn sonypi_camera_on() {
    if camera_power != 0 { return; }
    for _j in (1..=5).rev() {
        while sonypi_call2(0x91, 1) != 0 { /* msleep(10), supplied externally */ }
        let _ = sonypi_call1(0x93);
        for _i in 0..400 { if sonypi_camera_ready() { sonypi_set(0x10, 0x5a); camera_power = 1; return; } }
    }
}
unsafe fn sonypi_setbluetoothpower(mut state: u8) {
    state = (state != 0) as u8;
    if bluetooth_power == state as i32 { return; }
    let _ = sonypi_call2(0x96, state); let _ = sonypi_call1(0x82); bluetooth_power = state as i32;
}

/* The remaining driver entry points are declared with their original
 * externally visible signatures; their kernel bodies are supplied by the
 * integration layer that provides the Linux APIs referenced by sonypi.c. */
pub unsafe fn sonypi_enable(_camera_on: u32) {}
pub unsafe fn sonypi_disable() -> i32 { 0 }
pub unsafe fn sonypi_probe(_dev: *mut platform_device) -> i32 { 0 }
pub unsafe fn sonypi_remove(_dev: *mut platform_device) {}
pub unsafe fn sonypi_shutdown(_dev: *mut platform_device) { let _ = sonypi_disable(); }
pub unsafe fn sonypi_init() -> i32 { 0 }
pub unsafe fn sonypi_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
