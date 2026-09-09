/* SPDX-License-Identifier: GPL-2.0-only */
/* Direct Rust translation of arch/arm/include/asm/memory.h. */

/* Dependencies supplied by the surrounding kernel translation. */

pub const PAGE_OFFSET: usize = CONFIG_PAGE_OFFSET as usize;
pub const KERNEL_OFFSET: usize = PAGE_OFFSET;

#[cfg(feature = "CONFIG_MMU")]
pub const TASK_SIZE: usize = {
    #[cfg(not(feature = "CONFIG_KASAN"))]
    { CONFIG_PAGE_OFFSET as usize - SZ_16M as usize }
    #[cfg(feature = "CONFIG_KASAN")]
    { KASAN_SHADOW_START as usize }
};

#[cfg(feature = "CONFIG_MMU")]
pub const TASK_UNMAPPED_BASE: usize = align(TASK_SIZE / 3, SZ_16M as usize);
#[cfg(feature = "CONFIG_MMU")]
pub const TASK_SIZE_26: usize = 1usize << 26;
#[cfg(all(feature = "CONFIG_MMU", not(feature = "CONFIG_THUMB2_KERNEL")))]
pub const MODULES_VADDR: usize = PAGE_OFFSET - SZ_16M as usize;
#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_THUMB2_KERNEL"))]
pub const MODULES_VADDR: usize = PAGE_OFFSET - SZ_8M as usize;
#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_HIGHMEM"))]
pub const MODULES_END: usize = PAGE_OFFSET - PMD_SIZE as usize;
#[cfg(all(feature = "CONFIG_MMU", not(feature = "CONFIG_HIGHMEM")))]
pub const MODULES_END: usize = PAGE_OFFSET;
#[cfg(feature = "CONFIG_MMU")]
pub const FDT_FIXED_BASE: usize = 0xff800000;
#[cfg(feature = "CONFIG_MMU")]
pub const FDT_FIXED_SIZE: usize = 2 * SECTION_SIZE as usize;
#[cfg(feature = "CONFIG_MMU")]
pub const VECTORS_BASE: usize = 0xffff0000;

#[cfg(not(feature = "CONFIG_MMU"))]
extern "C" {
    pub fn setup_vectors_base() -> libc::c_ulong;
    pub static mut vectors_base: libc::c_ulong;
}
#[cfg(not(feature = "CONFIG_MMU"))]
pub const TASK_SIZE: usize = 0xffff_ffff;
#[cfg(all(not(feature = "CONFIG_MMU"), not(feature = "TASK_UNMAPPED_BASE")))]
pub const TASK_UNMAPPED_BASE: usize = 0;
#[cfg(not(feature = "CONFIG_MMU"))]
pub const MODULES_END: usize = END_MEM as usize;
#[cfg(not(feature = "CONFIG_MMU"))]
pub const MODULES_VADDR: usize = PAGE_OFFSET;

#[cfg(feature = "CONFIG_XIP_KERNEL")]
extern "C" { pub static mut _sdata: u8; }
#[cfg(not(feature = "CONFIG_XIP_KERNEL"))]
extern "C" { pub static mut _stext: u8; }
extern "C" { pub static mut _end: u8; }

#[cfg(feature = "CONFIG_HAVE_TCM")]
pub const ITCM_OFFSET: usize = 0xfffe0000;
#[cfg(feature = "CONFIG_HAVE_TCM")]
pub const DTCM_OFFSET: usize = 0xfffe8000;
pub const PLAT_PHYS_OFFSET: u64 = CONFIG_PHYS_OFFSET as u64;

extern "C" {
    pub static mut kernel_sec_start: u64;
    pub static mut kernel_sec_end: u64;
}

#[cfg(feature = "CONFIG_ARM_PATCH_PHYS_VIRT")]
pub const __PV_BITS_31_24: u32 = 0x81000000;
#[cfg(feature = "CONFIG_ARM_PATCH_PHYS_VIRT")]
pub const __PV_BITS_23_16: u32 = 0x810000;
#[cfg(feature = "CONFIG_ARM_PATCH_PHYS_VIRT")]
pub const __PV_BITS_7_0: u32 = 0x81;

#[cfg(feature = "CONFIG_ARM_PATCH_PHYS_VIRT")]
extern "C" {
    pub static mut __pv_phys_pfn_offset: libc::c_ulong;
    pub static mut __pv_offset: u64;
    pub fn fixup_pv_table(table: *const core::ffi::c_void, size: libc::c_ulong);
    pub static __pv_table_begin: *const core::ffi::c_void;
    pub static __pv_table_end: *const core::ffi::c_void;
}

#[cfg(not(feature = "CONFIG_ARM_PATCH_PHYS_VIRT"))]
pub const PHYS_OFFSET: u64 = PLAT_PHYS_OFFSET;

#[cfg(feature = "CONFIG_ARM_PATCH_PHYS_VIRT")]
pub unsafe fn __virt_to_phys_nodebug(x: libc::c_ulong) -> u64 {
    /* The original uses ARM-specific inline-assembly patchable stubs. */
    let _ = x;
    panic!("__pv_stub requires ARM inline assembly")
}
#[cfg(not(feature = "CONFIG_ARM_PATCH_PHYS_VIRT"))]
pub unsafe fn __virt_to_phys_nodebug(x: libc::c_ulong) -> u64 {
    x as u64 - PAGE_OFFSET as u64 + PHYS_OFFSET
}

#[cfg(feature = "CONFIG_ARM_PATCH_PHYS_VIRT")]
pub unsafe fn __phys_to_virt(x: u64) -> libc::c_ulong {
    let _ = x;
    panic!("__pv_stub requires ARM inline assembly")
}
#[cfg(not(feature = "CONFIG_ARM_PATCH_PHYS_VIRT"))]
pub unsafe fn __phys_to_virt(x: u64) -> libc::c_ulong {
    (x - PHYS_OFFSET + PAGE_OFFSET as u64) as libc::c_ulong
}

pub unsafe fn virt_to_pfn(p: *const core::ffi::c_void) -> libc::c_ulong {
    ((p as libc::c_ulong - PAGE_OFFSET as libc::c_ulong) >> PAGE_SHIFT) + PHYS_PFN_OFFSET()
}

#[cfg(feature = "CONFIG_DEBUG_VIRTUAL")]
extern "C" {
    pub fn __virt_to_phys(x: libc::c_ulong) -> u64;
    pub fn __phys_addr_symbol(x: libc::c_ulong) -> u64;
}
#[cfg(not(feature = "CONFIG_DEBUG_VIRTUAL"))]
pub unsafe fn __virt_to_phys(x: libc::c_ulong) -> u64 { __virt_to_phys_nodebug(x) }
#[cfg(not(feature = "CONFIG_DEBUG_VIRTUAL"))]
pub unsafe fn __phys_addr_symbol(x: libc::c_ulong) -> u64 { __virt_to_phys_nodebug(x) }

pub unsafe fn virt_to_phys(x: *const core::ffi::c_void) -> u64 { __virt_to_phys(x as libc::c_ulong) }
pub unsafe fn phys_to_virt(x: u64) -> *mut core::ffi::c_void { __phys_to_virt(x) as *mut _ }
pub unsafe fn __pa<T>(x: *const T) -> u64 { __virt_to_phys(x as libc::c_ulong) }
pub unsafe fn __va(x: u64) -> *mut core::ffi::c_void { __phys_to_virt(x) as *mut _ }
pub unsafe fn pfn_to_kaddr(pfn: libc::c_ulong) -> *mut core::ffi::c_void { __va((pfn as u64) << PAGE_SHIFT) }

extern "C" { pub static mut arch_phys_to_idmap_offset: i64; }
pub const IDMAP_INVALID_ADDR: u32 = u32::MAX;
pub unsafe fn arm_has_idmap_alias() -> bool { cfg!(feature = "CONFIG_MMU") && arch_phys_to_idmap_offset != 0 }
pub unsafe fn phys_to_idmap(mut addr: u64) -> libc::c_ulong {
    if cfg!(feature = "CONFIG_MMU") && arch_phys_to_idmap_offset != 0 {
        addr = addr.wrapping_add(arch_phys_to_idmap_offset as u64);
        if addr > u32::MAX as u64 { return IDMAP_INVALID_ADDR as libc::c_ulong; }
    }
    addr as libc::c_ulong
}
pub unsafe fn idmap_to_phys(idmap: libc::c_ulong) -> u64 {
    if cfg!(feature = "CONFIG_MMU") && arch_phys_to_idmap_offset != 0 {
        (idmap as u64).wrapping_sub(arch_phys_to_idmap_offset as u64)
    } else { idmap as u64 }
}
pub unsafe fn __virt_to_idmap(x: libc::c_ulong) -> libc::c_ulong { phys_to_idmap(__virt_to_phys(x)) }

pub const fn PHYS_PFN_OFFSET() -> libc::c_ulong {
    #[cfg(feature = "CONFIG_ARM_PATCH_PHYS_VIRT")]
    { 0 }
    #[cfg(not(feature = "CONFIG_ARM_PATCH_PHYS_VIRT"))]
    { (PHYS_OFFSET >> PAGE_SHIFT) as libc::c_ulong }
}

extern "C" { pub fn pfn_to_page(pfn: libc::c_ulong) -> *mut core::ffi::c_void; pub fn pfn_valid(pfn: libc::c_ulong) -> bool; pub static mut high_memory: *mut core::ffi::c_void; }
pub unsafe fn virt_to_page(kaddr: *const core::ffi::c_void) -> *mut core::ffi::c_void { pfn_to_page(virt_to_pfn(kaddr)) }
pub unsafe fn virt_addr_valid(kaddr: *const core::ffi::c_void) -> bool {
    (kaddr as libc::c_ulong >= PAGE_OFFSET as libc::c_ulong && kaddr as libc::c_ulong < high_memory as libc::c_ulong) && pfn_valid(virt_to_pfn(kaddr))
}

/* External constants/macros referenced by the original header: CONFIG_PAGE_OFFSET,
 * SZ_16M, SZ_8M, PMD_SIZE, SECTION_SIZE, PAGE_SHIFT, CONFIG_PHYS_OFFSET,
 * CONFIG_DRAM_BASE, CONFIG_DRAM_SIZE, END_MEM, KASAN_SHADOW_START, and ALIGN. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
