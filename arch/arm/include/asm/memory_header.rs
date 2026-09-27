/* SPDX-License-Identifier: GPL-2.0-only */
/* Direct Rust translation of arch/arm/include/asm/memory.h. */

/* Dependencies supplied by the surrounding kernel translation. */

pub const PAGE_OFFSET: usize = CONFIG_PAGE_OFFSET as usize;
pub const KERNEL_OFFSET: usize = PAGE_OFFSET;

#[cfg(CONFIG_MMU)]
pub const TASK_SIZE: usize = {
    #[cfg(not(CONFIG_KASAN))]
    { CONFIG_PAGE_OFFSET as usize - SZ_16M as usize }
    #[cfg(CONFIG_KASAN)]
    { KASAN_SHADOW_START as usize }
};

#[cfg(CONFIG_MMU)]
pub const TASK_UNMAPPED_BASE: usize = align(TASK_SIZE / 3, SZ_16M as usize);
#[cfg(CONFIG_MMU)]
pub const TASK_SIZE_26: usize = 1usize << 26;
#[cfg(all(CONFIG_MMU, not(CONFIG_THUMB2_KERNEL)))]
pub const MODULES_VADDR: usize = PAGE_OFFSET - SZ_16M as usize;
#[cfg(all(CONFIG_MMU, CONFIG_THUMB2_KERNEL))]
pub const MODULES_VADDR: usize = PAGE_OFFSET - SZ_8M as usize;
#[cfg(all(CONFIG_MMU, CONFIG_HIGHMEM))]
pub const MODULES_END: usize = PAGE_OFFSET - PMD_SIZE as usize;
#[cfg(all(CONFIG_MMU, not(CONFIG_HIGHMEM)))]
pub const MODULES_END: usize = PAGE_OFFSET;
#[cfg(CONFIG_MMU)]
pub const FDT_FIXED_BASE: usize = 0xff800000;
#[cfg(CONFIG_MMU)]
pub const FDT_FIXED_SIZE: usize = 2 * SECTION_SIZE as usize;
#[cfg(CONFIG_MMU)]
pub const VECTORS_BASE: usize = 0xffff0000;

#[cfg(not(CONFIG_MMU))]
extern "C" {
    pub fn setup_vectors_base() -> kernel::ffi::c_ulong;
    pub static mut vectors_base: kernel::ffi::c_ulong;
}
#[cfg(not(CONFIG_MMU))]
pub const TASK_SIZE: usize = 0xffff_ffff;
#[cfg(all(not(CONFIG_MMU), not(feature = "TASK_UNMAPPED_BASE")))]
pub const TASK_UNMAPPED_BASE: usize = 0;
#[cfg(not(CONFIG_MMU))]
pub const MODULES_END: usize = END_MEM as usize;
#[cfg(not(CONFIG_MMU))]
pub const MODULES_VADDR: usize = PAGE_OFFSET;

#[cfg(CONFIG_XIP_KERNEL)]
extern "C" { pub static mut _sdata: u8; }
#[cfg(not(CONFIG_XIP_KERNEL))]
extern "C" { pub static mut _stext: u8; }
extern "C" { pub static mut _end: u8; }

#[cfg(CONFIG_HAVE_TCM)]
pub const ITCM_OFFSET: usize = 0xfffe0000;
#[cfg(CONFIG_HAVE_TCM)]
pub const DTCM_OFFSET: usize = 0xfffe8000;
pub const PLAT_PHYS_OFFSET: u64 = CONFIG_PHYS_OFFSET as u64;

extern "C" {
    pub static mut kernel_sec_start: u64;
    pub static mut kernel_sec_end: u64;
}

#[cfg(CONFIG_ARM_PATCH_PHYS_VIRT)]
pub const __PV_BITS_31_24: u32 = 0x81000000;
#[cfg(CONFIG_ARM_PATCH_PHYS_VIRT)]
pub const __PV_BITS_23_16: u32 = 0x810000;
#[cfg(CONFIG_ARM_PATCH_PHYS_VIRT)]
pub const __PV_BITS_7_0: u32 = 0x81;

#[cfg(CONFIG_ARM_PATCH_PHYS_VIRT)]
extern "C" {
    pub static mut __pv_phys_pfn_offset: kernel::ffi::c_ulong;
    pub static mut __pv_offset: u64;
    pub fn fixup_pv_table(table: *const kernel::ffi::c_void, size: kernel::ffi::c_ulong);
    pub static __pv_table_begin: *const kernel::ffi::c_void;
    pub static __pv_table_end: *const kernel::ffi::c_void;
}

#[cfg(not(CONFIG_ARM_PATCH_PHYS_VIRT))]
pub const PHYS_OFFSET: u64 = PLAT_PHYS_OFFSET;

#[cfg(CONFIG_ARM_PATCH_PHYS_VIRT)]
pub unsafe fn __virt_to_phys_nodebug(x: kernel::ffi::c_ulong) -> u64 {
    /* The original uses ARM-specific inline-assembly patchable stubs. */
    let _ = x;
    panic!("__pv_stub requires ARM inline assembly")
}
#[cfg(not(CONFIG_ARM_PATCH_PHYS_VIRT))]
pub unsafe fn __virt_to_phys_nodebug(x: kernel::ffi::c_ulong) -> u64 {
    x as u64 - PAGE_OFFSET as u64 + PHYS_OFFSET
}

#[cfg(CONFIG_ARM_PATCH_PHYS_VIRT)]
pub unsafe fn __phys_to_virt(x: u64) -> kernel::ffi::c_ulong {
    let _ = x;
    panic!("__pv_stub requires ARM inline assembly")
}
#[cfg(not(CONFIG_ARM_PATCH_PHYS_VIRT))]
pub unsafe fn __phys_to_virt(x: u64) -> kernel::ffi::c_ulong {
    (x - PHYS_OFFSET + PAGE_OFFSET as u64) as kernel::ffi::c_ulong
}

pub unsafe fn virt_to_pfn(p: *const kernel::ffi::c_void) -> kernel::ffi::c_ulong {
    ((p as kernel::ffi::c_ulong - PAGE_OFFSET as kernel::ffi::c_ulong) >> PAGE_SHIFT) + PHYS_PFN_OFFSET()
}

#[cfg(CONFIG_DEBUG_VIRTUAL)]
extern "C" {
    pub fn __virt_to_phys(x: kernel::ffi::c_ulong) -> u64;
    pub fn __phys_addr_symbol(x: kernel::ffi::c_ulong) -> u64;
}
#[cfg(not(CONFIG_DEBUG_VIRTUAL))]
pub unsafe fn __virt_to_phys(x: kernel::ffi::c_ulong) -> u64 { __virt_to_phys_nodebug(x) }
#[cfg(not(CONFIG_DEBUG_VIRTUAL))]
pub unsafe fn __phys_addr_symbol(x: kernel::ffi::c_ulong) -> u64 { __virt_to_phys_nodebug(x) }

pub unsafe fn virt_to_phys(x: *const kernel::ffi::c_void) -> u64 { __virt_to_phys(x as kernel::ffi::c_ulong) }
pub unsafe fn phys_to_virt(x: u64) -> *mut kernel::ffi::c_void { __phys_to_virt(x) as *mut _ }
pub unsafe fn __pa<T>(x: *const T) -> u64 { __virt_to_phys(x as kernel::ffi::c_ulong) }
pub unsafe fn __va(x: u64) -> *mut kernel::ffi::c_void { __phys_to_virt(x) as *mut _ }
pub unsafe fn pfn_to_kaddr(pfn: kernel::ffi::c_ulong) -> *mut kernel::ffi::c_void { __va((pfn as u64) << PAGE_SHIFT) }

extern "C" { pub static mut arch_phys_to_idmap_offset: i64; }
pub const IDMAP_INVALID_ADDR: u32 = u32::MAX;
pub unsafe fn arm_has_idmap_alias() -> bool { cfg!(CONFIG_MMU) && arch_phys_to_idmap_offset != 0 }
pub unsafe fn phys_to_idmap(mut addr: u64) -> kernel::ffi::c_ulong {
    if cfg!(CONFIG_MMU) && arch_phys_to_idmap_offset != 0 {
        addr = addr.wrapping_add(arch_phys_to_idmap_offset as u64);
        if addr > u32::MAX as u64 { return IDMAP_INVALID_ADDR as kernel::ffi::c_ulong; }
    }
    addr as kernel::ffi::c_ulong
}
pub unsafe fn idmap_to_phys(idmap: kernel::ffi::c_ulong) -> u64 {
    if cfg!(CONFIG_MMU) && arch_phys_to_idmap_offset != 0 {
        (idmap as u64).wrapping_sub(arch_phys_to_idmap_offset as u64)
    } else { idmap as u64 }
}
pub unsafe fn __virt_to_idmap(x: kernel::ffi::c_ulong) -> kernel::ffi::c_ulong { phys_to_idmap(__virt_to_phys(x)) }

pub const fn PHYS_PFN_OFFSET() -> kernel::ffi::c_ulong {
    #[cfg(CONFIG_ARM_PATCH_PHYS_VIRT)]
    { 0 }
    #[cfg(not(CONFIG_ARM_PATCH_PHYS_VIRT))]
    { (PHYS_OFFSET >> PAGE_SHIFT) as kernel::ffi::c_ulong }
}

extern "C" { pub fn pfn_to_page(pfn: kernel::ffi::c_ulong) -> *mut kernel::ffi::c_void; pub fn pfn_valid(pfn: kernel::ffi::c_ulong) -> bool; pub static mut high_memory: *mut kernel::ffi::c_void; }
pub unsafe fn virt_to_page(kaddr: *const kernel::ffi::c_void) -> *mut kernel::ffi::c_void { pfn_to_page(virt_to_pfn(kaddr)) }
pub unsafe fn virt_addr_valid(kaddr: *const kernel::ffi::c_void) -> bool {
    (kaddr as kernel::ffi::c_ulong >= PAGE_OFFSET as kernel::ffi::c_ulong && (kaddr as kernel::ffi::c_ulong) < high_memory as kernel::ffi::c_ulong) && pfn_valid(virt_to_pfn(kaddr))
}

/* External constants/macros referenced by the original header: CONFIG_PAGE_OFFSET,
 * SZ_16M, SZ_8M, PMD_SIZE, SECTION_SIZE, PAGE_SHIFT, CONFIG_PHYS_OFFSET,
 * CONFIG_DRAM_BASE, CONFIG_DRAM_SIZE, END_MEM, KASAN_SHADOW_START, and ALIGN. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
