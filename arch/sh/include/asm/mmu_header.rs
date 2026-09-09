/* SPDX-License-Identifier: GPL-2.0 */

/* Privileged Space Mapping Buffer (PMB) definitions */
pub const PMB_PASCR: u32 = 0xff000070;
pub const PMB_IRMCR: u32 = 0xff000078;

pub const PASCR_SE: u32 = 0x80000000;

pub const PMB_ADDR: u32 = 0xf6100000;
pub const PMB_DATA: u32 = 0xf7100000;

pub const NR_PMB_ENTRIES: usize = 16;

pub const PMB_E_MASK: u32 = 0x0000000f;
pub const PMB_E_SHIFT: u32 = 8;

pub const PMB_PFN_MASK: u32 = 0xff000000;

pub const PMB_SZ_16M: u32 = 0x00000000;
pub const PMB_SZ_64M: u32 = 0x00000010;
pub const PMB_SZ_128M: u32 = 0x00000080;
pub const PMB_SZ_512M: u32 = 0x00000090;
pub const PMB_SZ_MASK: u32 = PMB_SZ_512M;
pub const PMB_C: u32 = 0x00000008;
pub const PMB_WT: u32 = 0x00000001;
pub const PMB_UB: u32 = 0x00000200;
pub const PMB_CACHE_MASK: u32 = PMB_C | PMB_WT | PMB_UB;
pub const PMB_V: u32 = 0x00000100;

pub const PMB_NO_ENTRY: i32 = -1;

/* Default "unsigned long" context. NR_CPUS is supplied by the target. */
pub type MmContextIdT = [::core::ffi::c_ulong; NR_CPUS];

#[repr(C)]
pub struct MmContextT {
    #[cfg(feature = "CONFIG_MMU")]
    pub id: MmContextIdT,
    #[cfg(feature = "CONFIG_MMU")]
    pub vdso: *mut core::ffi::c_void,
    #[cfg(not(feature = "CONFIG_MMU"))]
    pub end_brk: ::core::ffi::c_ulong,
    #[cfg(feature = "CONFIG_BINFMT_ELF_FDPIC")]
    pub exec_fdpic_loadmap: ::core::ffi::c_ulong,
    #[cfg(feature = "CONFIG_BINFMT_ELF_FDPIC")]
    pub interp_fdpic_loadmap: ::core::ffi::c_ulong,
}

#[cfg(feature = "CONFIG_PMB")]
extern "C" {
    pub fn __in_29bit_mode() -> bool;
    pub fn pmb_init();
    pub fn pmb_bolt_mapping(
        virt: ::core::ffi::c_ulong,
        phys: phys_addr_t,
        size: ::core::ffi::c_ulong,
        prot: pgprot_t,
    ) -> i32;
    pub fn pmb_remap_caller(
        phys: phys_addr_t,
        size: ::core::ffi::c_ulong,
        prot: pgprot_t,
        caller: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    pub fn pmb_unmap(addr: *mut core::ffi::c_void) -> i32;
}

#[cfg(not(feature = "CONFIG_PMB"))]
pub unsafe fn pmb_bolt_mapping(
    _virt: ::core::ffi::c_ulong,
    _phys: phys_addr_t,
    _size: ::core::ffi::c_ulong,
    _prot: pgprot_t,
) -> i32 {
    -22
}

#[cfg(not(feature = "CONFIG_PMB"))]
pub unsafe fn pmb_remap_caller(
    _phys: phys_addr_t,
    _size: ::core::ffi::c_ulong,
    _prot: pgprot_t,
    _caller: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_PMB"))]
pub unsafe fn pmb_unmap(_addr: *mut core::ffi::c_void) -> i32 {
    -22
}

#[cfg(not(feature = "CONFIG_PMB"))]
pub unsafe fn pmb_init<T>(_addr: T) {}

#[cfg(all(not(feature = "CONFIG_PMB"), feature = "CONFIG_29BIT"))]
pub const fn __in_29bit_mode() -> i32 { 1 }

#[cfg(all(not(feature = "CONFIG_PMB"), not(feature = "CONFIG_29BIT")))]
pub const fn __in_29bit_mode() -> i32 { 0 }

pub unsafe fn pmb_remap(
    phys: phys_addr_t,
    size: ::core::ffi::c_ulong,
    prot: pgprot_t,
) -> *mut core::ffi::c_void {
    pmb_remap_caller(phys, size, prot, __builtin_return_address(0))
}

extern "C" {
    fn __builtin_return_address(level: usize) -> *mut core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
