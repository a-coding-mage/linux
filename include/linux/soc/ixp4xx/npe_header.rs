/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_void};

/* linux/kernel.h and linux/regmap.h dependencies are supplied externally. */
pub struct regmap;
pub struct device;

extern "C" {
    pub static mut npe_names: *const *const c_char;
}

#[repr(C)]
pub struct npe_regs {
    pub exec_addr: u32,
    pub exec_data: u32,
    pub exec_status_cmd: u32,
    pub exec_count: u32,
    pub action_points: [u32; 4],
    pub watchpoint_fifo: u32,
    pub watch_count: u32,
    pub profile_count: u32,
    pub messaging_status: u32,
    pub messaging_control: u32,
    pub mailbox_status: u32,
    pub in_out_fifo: u32,
}

#[repr(C)]
pub struct npe {
    /* __iomem */
    pub regs: *mut npe_regs,
    pub rmap: *mut regmap,
    pub id: i32,
    pub valid: i32,
}

#[inline]
pub unsafe fn npe_name(npe: *mut npe) -> *const c_char {
    *npe_names.add((*npe).id as usize)
}

extern "C" {
    pub fn npe_running(npe: *mut npe) -> i32;
    pub fn npe_send_message(npe: *mut npe, msg: *const c_void, what: *const c_char) -> i32;
    pub fn npe_recv_message(npe: *mut npe, msg: *mut c_void, what: *const c_char) -> i32;
    pub fn npe_send_recv_message(npe: *mut npe, msg: *mut c_void, what: *const c_char) -> i32;
    pub fn npe_load_firmware(
        npe: *mut npe,
        name: *const c_char,
        dev: *mut device,
    ) -> i32;
    pub fn npe_request(id: u32) -> *mut npe;
    pub fn npe_release(npe: *mut npe);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
