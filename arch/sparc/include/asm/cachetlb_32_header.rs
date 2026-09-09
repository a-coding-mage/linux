/* SPDX-License-Identifier: GPL-2.0 */

// Forward declarations from the surrounding kernel translation unit.
#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sparc32_cachetlb_ops {
    pub cache_all: Option<unsafe extern "C" fn()>,
    pub cache_mm: Option<unsafe extern "C" fn(*mut mm_struct)>,
    pub cache_range:
        Option<unsafe extern "C" fn(*mut vm_area_struct, core::ffi::c_ulong, core::ffi::c_ulong)>,
    pub cache_page: Option<unsafe extern "C" fn(*mut vm_area_struct, core::ffi::c_ulong)>,

    pub tlb_all: Option<unsafe extern "C" fn()>,
    pub tlb_mm: Option<unsafe extern "C" fn(*mut mm_struct)>,
    pub tlb_range:
        Option<unsafe extern "C" fn(*mut vm_area_struct, core::ffi::c_ulong, core::ffi::c_ulong)>,
    pub tlb_page: Option<unsafe extern "C" fn(*mut vm_area_struct, core::ffi::c_ulong)>,

    pub page_to_ram: Option<unsafe extern "C" fn(core::ffi::c_ulong)>,
    pub sig_insns: Option<unsafe extern "C" fn(*mut mm_struct, core::ffi::c_ulong)>,
    pub page_for_dma: Option<unsafe extern "C" fn(core::ffi::c_ulong)>,
}

unsafe extern "C" {
    pub static sparc32_cachetlb_ops: *const sparc32_cachetlb_ops;

    // Preserved build-time condition: declared only when CONFIG_SMP is enabled.
    #[cfg(feature = "CONFIG_SMP")]
    pub static local_ops: *const sparc32_cachetlb_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
