/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Copyright IBM Corp. 2003, 2012
 *  Virtual CPU timer
 *
 *  Author(s): Jan Glauber <jan.glauber@de.ibm.com>
 */

// The C header guard `_ASM_S390_TIMER_H` is omitted; Rust item identity
// provides the corresponding protection.

pub const VTIMER_MAX_SLICE: u64 = 0x7fffffffffffffff_u64;

#[repr(C)]
pub struct vtimer_list {
    pub entry: list_head,
    pub expires: u64,
    pub interval: u64,
    pub function: Option<unsafe extern "C" fn(::core::ffi::c_ulong)>,
    pub data: ::core::ffi::c_ulong,
}

unsafe extern "C" {
    pub fn init_virt_timer(timer: *mut vtimer_list);
    pub fn add_virt_timer(timer: *mut vtimer_list);
    pub fn add_virt_timer_periodic(timer: *mut vtimer_list);
    pub fn mod_virt_timer(timer: *mut vtimer_list, expires: u64) -> ::core::ffi::c_int;
    pub fn mod_virt_timer_periodic(
        timer: *mut vtimer_list,
        expires: u64,
    ) -> ::core::ffi::c_int;
    pub fn del_virt_timer(timer: *mut vtimer_list) -> ::core::ffi::c_int;
    pub fn vtime_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
