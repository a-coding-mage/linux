/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/linkage.h, linux/mmzone.h, asm/addrspace.h, asm/io.h, asm/pgtable.h

pub const KASAN_SHADOW_SCALE_SHIFT: u32 = 3;
pub const KASAN_SHADOW_OFFSET: usize = CONFIG_KASAN_SHADOW_OFFSET as usize;

pub const XRANGE_SHIFT: u32 = 48;

/* Valid address length */
pub const XRANGE_SHADOW_SHIFT: usize = if cpu_vabits < VA_BITS { cpu_vabits } else { VA_BITS };
/* Used for taking out the valid address */
pub const XRANGE_SHADOW_MASK: usize = (1usize << XRANGE_SHADOW_SHIFT) - 1;
/* One segment whole address space size */
pub const XRANGE_SIZE: usize = XRANGE_SHADOW_MASK + 1;

/* 64-bit segment value. */
pub const XKPRANGE_UC_SEG: usize = 0x8000;
pub const XKPRANGE_CC_SEG: usize = 0x9000;
pub const XKPRANGE_WC_SEG: usize = 0xa000;
pub const XKVRANGE_VC_SEG: usize = 0xffff;

/* Cached */
pub const XKPRANGE_CC_START: usize = CACHE_BASE;
pub const XKPRANGE_CC_SIZE: usize = XRANGE_SIZE;
pub const XKPRANGE_CC_KASAN_OFFSET: usize = 0;
pub const XKPRANGE_CC_SHADOW_SIZE: usize = XKPRANGE_CC_SIZE >> KASAN_SHADOW_SCALE_SHIFT;
pub const XKPRANGE_CC_SHADOW_END: usize = XKPRANGE_CC_KASAN_OFFSET + XKPRANGE_CC_SHADOW_SIZE;

/* UnCached */
pub const XKPRANGE_UC_START: usize = UNCACHE_BASE;
pub const XKPRANGE_UC_SIZE: usize = XRANGE_SIZE;
pub const XKPRANGE_UC_KASAN_OFFSET: usize = XKPRANGE_CC_SHADOW_END;
pub const XKPRANGE_UC_SHADOW_SIZE: usize = XKPRANGE_UC_SIZE >> KASAN_SHADOW_SCALE_SHIFT;
pub const XKPRANGE_UC_SHADOW_END: usize = XKPRANGE_UC_KASAN_OFFSET + XKPRANGE_UC_SHADOW_SIZE;

/* WriteCombine */
pub const XKPRANGE_WC_START: usize = WRITECOMBINE_BASE;
pub const XKPRANGE_WC_SIZE: usize = XRANGE_SIZE;
pub const XKPRANGE_WC_KASAN_OFFSET: usize = XKPRANGE_UC_SHADOW_END;
pub const XKPRANGE_WC_SHADOW_SIZE: usize = XKPRANGE_WC_SIZE >> KASAN_SHADOW_SCALE_SHIFT;
pub const XKPRANGE_WC_SHADOW_END: usize = XKPRANGE_WC_KASAN_OFFSET + XKPRANGE_WC_SHADOW_SIZE;

/* VMALLOC (Cached or UnCached) */
pub const XKVRANGE_VC_START: usize = MODULES_VADDR;
pub const XKVRANGE_VC_SIZE: usize = round_up(KFENCE_AREA_END - MODULES_VADDR + 1, PGDIR_SIZE);
pub const XKVRANGE_VC_KASAN_OFFSET: usize = XKPRANGE_WC_SHADOW_END;
pub const XKVRANGE_VC_SHADOW_SIZE: usize = XKVRANGE_VC_SIZE >> KASAN_SHADOW_SCALE_SHIFT;
pub const XKVRANGE_VC_SHADOW_END: usize = XKVRANGE_VC_KASAN_OFFSET + XKVRANGE_VC_SHADOW_SIZE;

/* KAsan shadow memory start right after vmalloc. */
pub const KASAN_SHADOW_START: usize = round_up(KFENCE_AREA_END, PGDIR_SIZE);
pub const KASAN_SHADOW_SIZE: usize = XKVRANGE_VC_SHADOW_END - XKPRANGE_CC_KASAN_OFFSET;
pub const KASAN_SHADOW_END: usize = round_up(KASAN_SHADOW_START + KASAN_SHADOW_SIZE, PGDIR_SIZE) - 1;

pub const XKPRANGE_CC_SHADOW_OFFSET: usize = KASAN_SHADOW_START + XKPRANGE_CC_KASAN_OFFSET;
pub const XKPRANGE_UC_SHADOW_OFFSET: usize = KASAN_SHADOW_START + XKPRANGE_UC_KASAN_OFFSET;
pub const XKPRANGE_WC_SHADOW_OFFSET: usize = KASAN_SHADOW_START + XKPRANGE_WC_KASAN_OFFSET;
pub const XKVRANGE_VC_SHADOW_OFFSET: usize = KASAN_SHADOW_START + XKVRANGE_VC_KASAN_OFFSET;

unsafe extern "C" {
    pub static mut kasan_early_shadow_page: [u8; PAGE_SIZE];
    pub fn kasan_mem_to_shadow(addr: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    pub fn kasan_shadow_to_mem(shadow_addr: *const core::ffi::c_void) -> *const core::ffi::c_void;
    pub fn kasan_init();
    pub fn kasan_early_init();
}

#[inline(always)]
pub unsafe fn addr_has_metadata(addr: *const core::ffi::c_void) -> bool {
    kasan_mem_to_shadow(addr as *mut core::ffi::c_void) != core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
