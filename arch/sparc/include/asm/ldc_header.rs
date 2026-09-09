/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_void};

// Dependency supplied by asm/hypervisor.h in the C header.
extern "C" {
    pub static mut ldom_domaining_enabled: i32;
    pub fn ldom_set_var(var: *const c_char, value: *const c_char);
    pub fn ldom_reboot(boot_command: *const c_char);
    pub fn ldom_power_off();
}

#[repr(C)]
pub struct ldc_channel_config {
    pub event: Option<unsafe extern "C" fn(arg: *mut c_void, event: i32)>,
    pub mtu: u32,
    pub rx_irq: u32,
    pub tx_irq: u32,
    pub mode: u8,
    pub debug: u8,
}

pub const LDC_MODE_RAW: u8 = 0x00;
pub const LDC_MODE_UNRELIABLE: u8 = 0x01;
pub const LDC_MODE_RESERVED: u8 = 0x02;
pub const LDC_MODE_STREAM: u8 = 0x03;

pub const LDC_DEBUG_HS: u8 = 0x01;
pub const LDC_DEBUG_STATE: u8 = 0x02;
pub const LDC_DEBUG_RX: u8 = 0x04;
pub const LDC_DEBUG_TX: u8 = 0x08;
pub const LDC_DEBUG_DATA: u8 = 0x10;

pub const LDC_EVENT_RESET: i32 = 0x01;
pub const LDC_EVENT_UP: i32 = 0x02;
pub const LDC_EVENT_DATA_READY: i32 = 0x04;

pub const LDC_STATE_INVALID: u8 = 0x00;
pub const LDC_STATE_INIT: u8 = 0x01;
pub const LDC_STATE_BOUND: u8 = 0x02;
pub const LDC_STATE_READY: u8 = 0x03;
pub const LDC_STATE_CONNECTED: u8 = 0x04;

pub const LDC_PACKET_SIZE: usize = 64;

#[repr(C)]
pub struct ldc_channel {
    _private: [u8; 0],
}

extern "C" {
    pub fn ldc_alloc(
        id: libc::c_ulong,
        cfgp: *const ldc_channel_config,
        event_arg: *mut c_void,
        name: *const c_char,
    ) -> *mut ldc_channel;
    pub fn ldc_free(lp: *mut ldc_channel);
    pub fn ldc_bind(lp: *mut ldc_channel) -> i32;
    pub fn ldc_unbind(lp: *mut ldc_channel);
    pub fn ldc_connect(lp: *mut ldc_channel) -> i32;
    pub fn ldc_disconnect(lp: *mut ldc_channel) -> i32;
    pub fn ldc_state(lp: *mut ldc_channel) -> i32;
    pub fn ldc_set_state(lp: *mut ldc_channel, state: u8);
    pub fn ldc_mode(lp: *mut ldc_channel) -> i32;
    pub fn __ldc_print(lp: *mut ldc_channel, caller: *const c_char);
    pub fn ldc_rx_reset(lp: *mut ldc_channel) -> i32;
    pub fn ldc_write(lp: *mut ldc_channel, buf: *const c_void, size: u32) -> i32;
    pub fn ldc_read(lp: *mut ldc_channel, buf: *mut c_void, size: u32) -> i32;
}

// C macro: ldc_print(chan) expands to __ldc_print(chan, __func__).
// Callers should provide the function name as a C string when invoking __ldc_print.

pub const LDC_MAP_SHADOW: u32 = 0x01;
pub const LDC_MAP_DIRECT: u32 = 0x02;
pub const LDC_MAP_IO: u32 = 0x04;
pub const LDC_MAP_R: u32 = 0x08;
pub const LDC_MAP_W: u32 = 0x10;
pub const LDC_MAP_X: u32 = 0x20;
pub const LDC_MAP_RW: u32 = LDC_MAP_R | LDC_MAP_W;
pub const LDC_MAP_RWX: u32 = LDC_MAP_R | LDC_MAP_W | LDC_MAP_X;
pub const LDC_MAP_ALL: u32 = 0x03f;

#[repr(C)]
pub struct ldc_trans_cookie {
    pub cookie_addr: u64,
    pub cookie_size: u64,
}

#[repr(C)]
pub struct scatterlist {
    _private: [u8; 0],
}

extern "C" {
    pub fn ldc_map_sg(
        lp: *mut ldc_channel, sg: *mut scatterlist, num_sg: i32,
        cookies: *mut ldc_trans_cookie, ncookies: i32, map_perm: u32,
    ) -> i32;
    pub fn ldc_map_single(
        lp: *mut ldc_channel, buf: *mut c_void, len: u32,
        cookies: *mut ldc_trans_cookie, ncookies: i32, map_perm: u32,
    ) -> i32;
    pub fn ldc_unmap(lp: *mut ldc_channel, cookies: *mut ldc_trans_cookie, ncookies: i32);
    pub fn ldc_copy(
        lp: *mut ldc_channel, copy_dir: i32, buf: *mut c_void, len: u32,
        offset: libc::c_ulong, cookies: *mut ldc_trans_cookie, ncookies: i32,
    ) -> i32;
    pub fn ldc_alloc_exp_dring(
        lp: *mut ldc_channel, len: u32, cookies: *mut ldc_trans_cookie,
        ncookies: *mut i32, map_perm: u32,
    ) -> *mut c_void;
    pub fn ldc_free_exp_dring(
        lp: *mut ldc_channel, buf: *mut c_void, len: u32,
        cookies: *mut ldc_trans_cookie, ncookies: i32,
    );
}

pub unsafe fn ldc_get_dring_entry(
    lp: *mut ldc_channel, buf: *mut c_void, len: u32, offset: libc::c_ulong,
    cookies: *mut ldc_trans_cookie, ncookies: i32,
) -> i32 {
    ldc_copy(lp, LDC_COPY_IN, buf, len, offset, cookies, ncookies)
}

pub unsafe fn ldc_put_dring_entry(
    lp: *mut ldc_channel, buf: *mut c_void, len: u32, offset: libc::c_ulong,
    cookies: *mut ldc_trans_cookie, ncookies: i32,
) -> i32 {
    ldc_copy(lp, LDC_COPY_OUT, buf, len, offset, cookies, ncookies)
}

// Dependency supplied by asm/hypervisor.h in the C header.
extern "C" {
    pub static LDC_COPY_IN: i32;
    pub static LDC_COPY_OUT: i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
