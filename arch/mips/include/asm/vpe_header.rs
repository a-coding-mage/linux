/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2005 MIPS Technologies, Inc.  All rights reserved.
 * Copyright (C) 2013 Imagination Technologies Ltd.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than redefined.

pub const VPE_MODULE_NAME: &str = "vpe";
pub const VPE_MODULE_MINOR: i32 = 1;

/* Grab the likely amount of memory we will need. */
// CONFIG_MIPS_VPE_LOADER_TOM selects the 2 MiB build-time configuration.
#[cfg(CONFIG_MIPS_VPE_LOADER_TOM)]
pub const P_SIZE: usize = 2 * 1024 * 1024;
#[cfg(not(CONFIG_MIPS_VPE_LOADER_TOM))]
/* Add an overhead to the max kmalloc size for non-striped symbols/etc. */
pub const P_SIZE: usize = 256 * 1024;

pub const MAX_VPES: i32 = 16;

#[inline]
pub unsafe fn aprp_cpu_index() -> i32 {
    unsafe extern "C" {
        static mut tclimit: i32;
    }
    unsafe { tclimit }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpe_state {
    VPE_STATE_UNUSED = 0,
    VPE_STATE_INUSE,
    VPE_STATE_RUNNING,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tc_state {
    TC_STATE_UNUSED = 0,
    TC_STATE_INUSE,
    TC_STATE_RUNNING,
    TC_STATE_DYNAMIC,
}

#[repr(C)]
pub struct vpe {
    pub state: vpe_state,
    pub minor: i32,
    pub load_addr: *mut core::ffi::c_void,
    pub len: u64,
    pub pbuffer: *mut i8,
    pub plen: u64,
    pub __start: u64,
    pub tc: list_head,
    pub list: list_head,
    pub shared_ptr: *mut core::ffi::c_void,
    pub notify: list_head,
    pub ntcs: u32,
}

#[repr(C)]
pub struct tc {
    pub state: tc_state,
    pub index: i32,
    pub pvpe: *mut vpe,
    pub tc: list_head,
    pub list: list_head,
}

#[repr(C)]
pub struct vpe_notifications {
    pub start: Option<unsafe extern "C" fn(vpe: i32)>,
    pub stop: Option<unsafe extern "C" fn(vpe: i32)>,
    pub list: list_head,
}

#[repr(C)]
pub struct vpe_control {
    pub vpe_list_lock: spinlock_t,
    pub vpe_list: list_head,
    pub tc_list_lock: spinlock_t,
    pub tc_list: list_head,
}

unsafe extern "C" {
    pub static mut vpecontrol: vpe_control;
    pub static vpe_fops: file_operations;

    pub fn vpe_notify(index: i32, notify: *mut vpe_notifications) -> i32;
    pub fn vpe_get_shared(index: i32) -> *mut core::ffi::c_void;
    pub fn get_vpe(minor: i32) -> *mut vpe;
    pub fn get_tc(index: i32) -> *mut tc;
    pub fn alloc_vpe(minor: i32) -> *mut vpe;
    pub fn alloc_tc(index: i32) -> *mut tc;
    pub fn release_vpe(v: *mut vpe);
    pub fn alloc_progmem(len: u64) -> *mut core::ffi::c_void;
    pub fn release_progmem(ptr: *mut core::ffi::c_void);
    pub fn vpe_run(v: *mut vpe) -> i32;
    pub fn cleanup_tc(tc: *mut tc);
    pub fn vpe_module_init() -> i32;
    pub fn vpe_module_exit();

    // CONFIG_MIPS_VPE_LOADER_MT declarations.
    #[cfg(CONFIG_MIPS_VPE_LOADER_MT)]
    pub fn vpe_alloc() -> *mut core::ffi::c_void;
    #[cfg(CONFIG_MIPS_VPE_LOADER_MT)]
    pub fn vpe_start(vpe: *mut core::ffi::c_void, start: u64) -> i32;
    #[cfg(CONFIG_MIPS_VPE_LOADER_MT)]
    pub fn vpe_stop(vpe: *mut core::ffi::c_void) -> i32;
    #[cfg(CONFIG_MIPS_VPE_LOADER_MT)]
    pub fn vpe_free(vpe: *mut core::ffi::c_void) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
