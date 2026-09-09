/* SPDX-License-Identifier: GPL-2.0 */
/*
 * /proc/kcore definitions
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum kcore_type {
    KCORE_TEXT,
    KCORE_VMALLOC,
    KCORE_RAM,
    KCORE_VMEMMAP,
    KCORE_USER,
}

#[repr(C)]
pub struct kcore_list {
    pub list: crate::list_head,
    pub addr: usize,
    pub size: usize,
    pub type_: i32,
}

/* CONFIG_PROC_KCORE */
#[cfg(feature = "CONFIG_PROC_KCORE")]
extern "C" {
    pub fn kclist_add(
        new: *mut kcore_list,
        addr: *mut core::ffi::c_void,
        size: usize,
        type_: i32,
    );

    pub fn register_mem_pfn_is_ram(
        fn_: Option<unsafe extern "C" fn(pfn: usize) -> i32>,
    ) -> i32;
}

#[cfg(not(feature = "CONFIG_PROC_KCORE"))]
pub unsafe fn kclist_add(
    _new: *mut kcore_list,
    _addr: *mut core::ffi::c_void,
    _size: usize,
    _type: i32,
) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
