// SPDX-License-Identifier: GPL-2.0+
/*
 * written by David Borowski
 *
 * Copyright (C) 2003 David Borowski.
 *
 * specifically written as a driver for the speakup screenreview
 * package it's not a general device driver.
 * This driver is for the Keynote Gold internal synthesizer.
 */
// Linux kernel headers and Speakup headers are supplied by the surrounding build.

const DRV_VERSION: *const u8 = b"2.10\0".as_ptr();
const SYNTH_IO_EXTENT: u32 = 0x04;
const PROCSPEECH: u8 = 0x1f;
const SYNTH_CLEAR: u8 = 0x03;

extern "C" {
    fn synth_request_region(port: u32, extent: u32) -> i32;
    fn synth_release_region(port: u32, extent: u32);
    fn spk_stop_serial_interrupt();
    fn spk_synth_is_alive_nop(synth: *mut spk_synth);
    fn spk_var_show();
    fn spk_var_store();
    fn spk_get_var(id: i32) -> *mut var_t;
    fn synth_buffer_skip_nonlatin1();
    fn synth_buffer_empty() -> bool;
    fn synth_buffer_getc() -> u8;
    fn kthread_should_stop() -> bool;
    fn set_current_state(state: i32);
    fn schedule_timeout(timeout: i64) -> i64;
    fn msecs_to_jiffies(value: i32) -> i64;
    fn inb(port: u32) -> u8;
    fn inb_p(port: u32) -> u8;
    fn outb_p(value: u8, port: u32);
    fn udelay(value: u32);
    fn pr_info(fmt: *const u8, ...);
    fn pr_warn(fmt: *const u8, ...);
    static mut speakup_info: speakup_info_t;
    static mut jiffies: u64;
}

#[allow(non_camel_case_types)]
type u_char = u8;

// Types, constants, and callbacks below are provided by spk_priv.h and speakup.h.
#[repr(C)]
pub struct var_t { pub u: var_union }
#[repr(C)]
pub union var_union { pub n: var_num, pub s: var_string }
#[repr(C)]
pub struct var_num { pub value: i32, pub default_val: i32 }
#[repr(C)]
pub struct var_string { pub value: *const u8 }
#[repr(C)]
pub struct speakup_info_t { pub spinlock: spinlock_t, pub flushing: i32 }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct spk_synth { pub name: *const u8, pub version: *const u8, pub long_name: *const u8, pub alive: i32, pub flush: Option<unsafe extern "C" fn(*mut spk_synth)> }

extern "C" {
    static mut synth_port: i32;
}

static mut port_forced: i32 = 0;
static synth_portlist: [u32; 2] = [0x2a8, 0];

#[repr(i32)]
enum default_vars_id { CAPS_START_ID = 0, CAPS_STOP_ID, RATE_ID, PITCH_ID, DIRECT_ID, V_LAST_VAR_ID, NB_ID }

static mut vars: [var_t; NB_ID as usize] = [
    var_t { u: var_union { s: var_string { value: b"[f130]\0".as_ptr() } } },
    var_t { u: var_union { s: var_string { value: b"[f90]\0".as_ptr() } } },
    var_t { u: var_union { n: var_num { value: 0, default_val: 8 } } },
    var_t { u: var_union { n: var_num { value: 0, default_val: 5 } } },
    var_t { u: var_union { n: var_num { value: 0, default_val: 0 } } },
];

const UART_RX: u32 = 2;
const SPACE: u8 = b' ';
const JIFFY: i32 = 0;
const DELAY: i32 = 0;
const FULL: i32 = 0;
const TASK_INTERRUPTIBLE: i32 = 1;
const TASK_RUNNING: i32 = 0;

unsafe fn synth_writable() -> bool { (inb_p(synth_port as u32 + UART_RX) & 0x10) != 0 }
unsafe fn synth_full() -> bool { (inb_p(synth_port as u32 + UART_RX) & 0x80) == 0 }

unsafe fn oops() -> *mut u8 {
    let s1 = inb_p(synth_port as u32); let s2 = inb_p(synth_port as u32 + 1);
    let s3 = inb_p(synth_port as u32 + 2); let s4 = inb_p(synth_port as u32 + 3);
    pr_warn(b"synth timeout %d %d %d %d\0".as_ptr(), s1 as i32, s2 as i32, s3 as i32, s4 as i32);
    core::ptr::null_mut()
}

unsafe fn synth_immediate(_synth: *mut spk_synth, mut buf: *const u8) -> *const u8 {
    while *buf != 0 { let mut ch = *buf; if ch == b'\n' { ch = PROCSPEECH; }
        if synth_full() { return buf; }
        let mut timeout = 1000; while synth_writable() { timeout -= 1; if timeout <= 0 { return oops(); } }
        outb_p(ch, synth_port as u32); udelay(70); buf = buf.add(1);
    } core::ptr::null()
}

unsafe fn do_catch_up(synth: *mut spk_synth) {
    let jiffy_delta = spk_get_var(JIFFY); let delay_time = spk_get_var(DELAY); let full_time = spk_get_var(FULL);
    let jiffy_max = jiffies.wrapping_add((*jiffy_delta).u.n.value as u64);
    while !kthread_should_stop() {
        synth_buffer_skip_nonlatin1(); if synth_buffer_empty() { break; }
        set_current_state(TASK_INTERRUPTIBLE); let full = (*full_time).u.n.value; set_current_state(TASK_RUNNING);
        if synth_full() { schedule_timeout(msecs_to_jiffies(full)); continue; }
        let mut timeout = 1000; while synth_writable() { timeout -= 1; if timeout <= 0 { break; } }
        if timeout <= 0 { oops(); break; }
        let mut ch = synth_buffer_getc(); if ch == b'\n' { ch = PROCSPEECH; } outb_p(ch, synth_port as u32); udelay(70);
        if jiffies >= jiffy_max && ch == SPACE { outb_p(PROCSPEECH, synth_port as u32); schedule_timeout(msecs_to_jiffies((*delay_time).u.n.value)); }
    }
    let mut timeout = 1000; while synth_writable() { timeout -= 1; if timeout <= 0 { break; } }
    if timeout <= 0 { oops(); } else { outb_p(PROCSPEECH, synth_port as u32); }
}

unsafe fn synth_flush(_synth: *mut spk_synth) { outb_p(SYNTH_CLEAR, synth_port as u32); }

unsafe fn synth_probe(synth: *mut spk_synth) -> i32 {
    pr_info(b"Probing for %s.\n\0".as_ptr(), (*synth).long_name);
    if port_forced != 0 { synth_port = port_forced; if synth_request_region((synth_port - 1) as u32, SYNTH_IO_EXTENT) != 0 { return -16; } }
    else { for &port in &synth_portlist[..1] { if synth_request_region(port, SYNTH_IO_EXTENT) == 0 && inb(port) == 0x80 { synth_port = port as i32; break; } } }
    if synth_port == 0 { return -19; } (*synth).alive = 1; 0
}

unsafe fn keynote_release(_synth: *mut spk_synth) { spk_stop_serial_interrupt(); if synth_port != 0 { synth_release_region(synth_port as u32, SYNTH_IO_EXTENT); } synth_port = 0; }

// The following kernel objects correspond to the C kobj_attribute declarations and
// attribute group; their concrete layouts are supplied by the kernel bindings.
#[repr(C)] pub struct kobj_attribute { _private: [u8; 0] }
static mut caps_start_attribute: kobj_attribute = kobj_attribute { _private: [] };
static mut caps_stop_attribute: kobj_attribute = kobj_attribute { _private: [] };
static mut pitch_attribute: kobj_attribute = kobj_attribute { _private: [] };
static mut rate_attribute: kobj_attribute = kobj_attribute { _private: [] };
static mut delay_time_attribute: kobj_attribute = kobj_attribute { _private: [] };
static mut direct_attribute: kobj_attribute = kobj_attribute { _private: [] };
static mut full_time_attribute: kobj_attribute = kobj_attribute { _private: [] };
static mut jiffy_delta_attribute: kobj_attribute = kobj_attribute { _private: [] };
static mut trigger_time_attribute: kobj_attribute = kobj_attribute { _private: [] };

// Equivalent to the C synth_keypc registration object and module declarations.
#[no_mangle] pub static mut synth_keypc: spk_synth = spk_synth {
    name: b"keypc\0".as_ptr(), version: DRV_VERSION, long_name: b"Keynote PC\0".as_ptr(),
    alive: 0, flush: Some(synth_flush),
};
// module_param*(port, start, rate, pitch, direct)
// MODULE_PARM_DESC: port="Set the port for the synthesizer (override probing)."
// MODULE_PARM_DESC: start="Start the synthesizer once it is loaded."
// MODULE_PARM_DESC: rate="Set the rate variable on load."
// MODULE_PARM_DESC: pitch="Set the pitch variable on load."
// MODULE_PARM_DESC: direct="Set the direct variable on load."
// module_spk_synth(synth_keypc)
// MODULE_AUTHOR("David Borowski"); MODULE_DESCRIPTION("Speakup support for Keynote Gold PC synthesizers");
// MODULE_LICENSE("GPL"); MODULE_VERSION(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
