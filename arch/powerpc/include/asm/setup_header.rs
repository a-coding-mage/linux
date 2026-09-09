/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by <uapi/asm/setup.h> is intentionally not expanded here.

#[cfg(not(feature = "assembler"))]
extern "C" {
    pub fn ppc_printk_progress(s: *mut core::ffi::c_char, hex: u16);

    pub static mut memory_limit: u64;

    // Used in very early kernel initialization.
    pub fn reloc_offset() -> usize;
    pub fn add_reloc_offset(x: usize) -> usize;
    pub fn reloc_got2(x: usize);

    pub fn check_for_initrd();
    pub fn mem_topology_setup();
    pub fn setup_panic();

    pub fn rfi_flush_enable(enable: bool);

    pub fn setup_rfi_flush(kind: l1d_flush_type, enable: bool);
    pub fn setup_entry_flush(enable: bool);
    pub fn setup_uaccess_flush(enable: bool);
    pub fn do_rfi_flush_fixups(types: l1d_flush_type);
    pub fn do_uaccess_flush_fixups(types: l1d_flush_type);
    pub fn do_entry_flush_fixups(types: l1d_flush_type);
    pub fn do_barrier_nospec_fixups(enable: bool);

    pub static mut barrier_nospec_enabled: bool;

    pub fn do_btb_flush_fixups();

    pub fn early_setup(dt_ptr: usize);
    pub fn early_setup_secondary();

    pub static mut ppc_hw_desc: seq_buf;
}

// Forward declaration of the externally defined device-tree node type.
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[macro_export]
macro_rules! PTRRELOC {
    ($x:expr) => {{
        unsafe { $crate::add_reloc_offset(($x) as usize) as _ }
    }};
}

pub const ARCH_PANIC_TIMEOUT: i32 = 180;

#[cfg(feature = "numa")]
extern "C" {
    pub fn initmem_init();
}
#[cfg(not(feature = "numa"))]
#[inline]
pub fn initmem_init() {}

#[cfg(feature = "ppc_pseries")]
extern "C" {
    pub fn pseries_reloc_on_exception() -> bool;
    pub fn pseries_enable_reloc_on_exc() -> bool;
    pub fn pseries_disable_reloc_on_exc();
    pub fn pseries_big_endian_exceptions();
    pub fn pseries_little_endian_exceptions();
}
#[cfg(not(feature = "ppc_pseries"))]
#[inline]
pub fn pseries_reloc_on_exception() -> bool { false }
#[cfg(not(feature = "ppc_pseries"))]
#[inline]
pub fn pseries_enable_reloc_on_exc() -> bool { false }
#[cfg(not(feature = "ppc_pseries"))]
#[inline]
pub fn pseries_disable_reloc_on_exc() {}
#[cfg(not(feature = "ppc_pseries"))]
#[inline]
pub fn pseries_big_endian_exceptions() {}
#[cfg(not(feature = "ppc_pseries"))]
#[inline]
pub fn pseries_little_endian_exceptions() {}

// These are bit flags.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum l1d_flush_type {
    L1D_FLUSH_NONE = 0x1,
    L1D_FLUSH_FALLBACK = 0x2,
    L1D_FLUSH_ORI = 0x4,
    L1D_FLUSH_MTTRIG = 0x8,
}

#[cfg(feature = "ppc_barrier_nospec")]
extern "C" {
    pub fn setup_barrier_nospec();
    pub fn do_barrier_nospec_fixups_range(enable: bool, start: *mut core::ffi::c_void, end: *mut core::ffi::c_void);
}
#[cfg(not(feature = "ppc_barrier_nospec"))]
#[inline]
pub fn setup_barrier_nospec() {}
#[cfg(not(feature = "ppc_barrier_nospec"))]
#[inline]
pub fn do_barrier_nospec_fixups_range(_enable: bool, _start: *mut core::ffi::c_void, _end: *mut core::ffi::c_void) {}

#[cfg(feature = "ppc_e500")]
extern "C" {
    pub fn setup_spectre_v2();
}
#[cfg(not(feature = "ppc_e500"))]
#[inline]
pub fn setup_spectre_v2() {}

#[cfg(feature = "ppc32")]
extern "C" {
    pub fn early_init(dt_ptr: usize) -> usize;
    pub fn machine_init(dt_ptr: u64);
}

// Externally defined sequence-buffer type.
#[repr(C)]
pub struct seq_buf {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
