/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2004, 2005 MIPS Technologies, Inc.  All rights reserved.
 * Copyright (C) 2013 Imagination Technologies Ltd.
 */

// Dependency supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_void};

pub const RTLX_MODULE_NAME: &str = "rtlx";

pub const LX_NODE_BASE: i32 = 10;

pub const MIPS_CPU_RTLX_IRQ: i32 = 0;

pub const RTLX_VERSION: i32 = 2;
pub const RTLX_XID: u32 = 0x12345600;
pub const RTLX_ID: u32 = RTLX_XID | RTLX_VERSION as u32;
pub const RTLX_BUFFER_SIZE: i32 = 2048;
pub const RTLX_CHANNELS: usize = 8;

pub const RTLX_CHANNEL_STDIO: i32 = 0;
pub const RTLX_CHANNEL_DBG: i32 = 1;
pub const RTLX_CHANNEL_SYSIO: i32 = 2;

// Declaration-only external functions.
unsafe extern "C" {
    pub fn rtlx_starting(vpe: i32);
    pub fn rtlx_stopping(vpe: i32);

    pub fn rtlx_open(index: i32, can_sleep: i32) -> i32;
    pub fn rtlx_release(index: i32) -> i32;
    pub fn rtlx_read(index: i32, buff: *mut c_void, count: usize) -> isize;
    pub fn rtlx_write(index: i32, buffer: *const c_void, count: usize) -> isize;
    pub fn rtlx_read_poll(index: i32, can_sleep: i32) -> u32;
    pub fn rtlx_write_poll(index: i32) -> u32;

    pub fn rtlx_module_init() -> i32;
    pub fn rtlx_module_exit();

    pub fn _interrupt_sp();

    pub static mut rtlx_notify: vpe_notifications;
    pub static rtlx_fops: file_operations;
    pub static mut aprp_hook: Option<unsafe extern "C" fn()>;

    pub static mut channel_wqs: [chan_waitqueues; RTLX_CHANNELS];

    pub static mut rtlx: *mut rtlx_info;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtlx_state {
    RTLX_STATE_UNUSED = 0,
    RTLX_STATE_INITIALISED,
    RTLX_STATE_REMOTE_READY,
    RTLX_STATE_OPENED,
}

#[repr(C)]
pub struct chan_waitqueues {
    pub rt_queue: wait_queue_head_t,
    pub lx_queue: wait_queue_head_t,
    pub in_open: atomic_t,
    pub mutex: mutex,
}

/* each channel supports read and write.
   linux (vpe0) reads lx_buffer and writes rt_buffer
   SP (vpe1) reads rt_buffer and writes lx_buffer
*/
#[repr(C)]
pub struct rtlx_channel {
    pub rt_state: rtlx_state,
    pub lx_state: rtlx_state,

    pub buffer_size: i32,

    /* read and write indexes per buffer */
    pub rt_write: i32,
    pub rt_read: i32,
    pub rt_buffer: *mut c_char,

    pub lx_write: i32,
    pub lx_read: i32,
    pub lx_buffer: *mut c_char,
}

#[repr(C)]
pub struct rtlx_info {
    pub id: usize,
    pub state: rtlx_state,

    pub channel: [rtlx_channel; RTLX_CHANNELS],
}

// Types supplied by other translated headers.
extern "C" {
    type vpe_notifications;
    type file_operations;
    type wait_queue_head_t;
    type atomic_t;
    type mutex;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
