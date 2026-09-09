/* SPDX-License-Identifier: GPL-2.0 */

// Dependency provided by <asm-generic/module.h>.
// Dependency provided by <asm/orc_types.h>.

#[repr(C)]
pub struct its_array {
    #[cfg(CONFIG_MITIGATION_ITS)]
    pub pages: *mut *mut core::ffi::c_void,
    #[cfg(CONFIG_MITIGATION_ITS)]
    pub num: core::ffi::c_int,
}

#[repr(C)]
pub struct mod_arch_specific {
    #[cfg(CONFIG_UNWINDER_ORC)]
    pub num_orcs: core::ffi::c_uint,
    #[cfg(CONFIG_UNWINDER_ORC)]
    pub orc_unwind_ip: *mut core::ffi::c_int,
    #[cfg(CONFIG_UNWINDER_ORC)]
    pub orc_unwind: *mut orc_entry,
    pub its_pages: its_array,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
