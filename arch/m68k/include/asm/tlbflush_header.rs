/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of the m68k TLB flush header. */

#[cfg(feature = "CONFIG_MMU")]
#[cfg(not(feature = "CONFIG_SUN3"))]
pub unsafe fn flush_tlb_kernel_page(addr: *mut core::ffi::c_void) {
    if CPU_IS_COLDFIRE {
        mmu_write(MMUOR, MMUOR_CNL);
    } else if CPU_IS_040_OR_060 {
        set_fc(SUPER_DATA);
        core::arch::asm!(".chip 68040", "pflush ({0})", ".chip 68k", in("a") addr);
        set_fc(USER_DATA);
    } else if CPU_IS_020_OR_030 {
        core::arch::asm!("pflush #4,#4,({0})", in("a") addr);
    }
}

/* flush all user-space atc entries. */
#[cfg(feature = "CONFIG_MMU")]
#[cfg(not(feature = "CONFIG_SUN3"))]
pub unsafe fn __flush_tlb() {
    if CPU_IS_COLDFIRE {
        mmu_write(MMUOR, MMUOR_CNL);
    } else if CPU_IS_040_OR_060 {
        core::arch::asm!(".chip 68040", "pflushan", ".chip 68k");
    } else if CPU_IS_020_OR_030 {
        core::arch::asm!("pflush #0,#4");
    }
}

#[cfg(feature = "CONFIG_MMU")]
#[cfg(not(feature = "CONFIG_SUN3"))]
pub unsafe fn __flush_tlb040_one(addr: usize) {
    core::arch::asm!(".chip 68040", "pflush ({0})", ".chip 68k", in("a") addr);
}

#[cfg(feature = "CONFIG_MMU")]
#[cfg(not(feature = "CONFIG_SUN3"))]
pub unsafe fn __flush_tlb_one(addr: usize) {
    if CPU_IS_COLDFIRE {
        mmu_write(MMUOR, MMUOR_CNL);
    } else if CPU_IS_040_OR_060 {
        __flush_tlb040_one(addr);
    } else if CPU_IS_020_OR_030 {
        core::arch::asm!("pflush #0,#4,({0})", in("a") addr);
    }
}

#[cfg(feature = "CONFIG_MMU")]
#[cfg(not(feature = "CONFIG_SUN3"))]
pub unsafe fn flush_tlb() { __flush_tlb(); }

/* flush all atc entries (both kernel and user-space entries). */
#[cfg(feature = "CONFIG_MMU")]
#[cfg(not(feature = "CONFIG_SUN3"))]
pub unsafe fn flush_tlb_all() {
    if CPU_IS_COLDFIRE {
        mmu_write(MMUOR, MMUOR_CNL);
    } else if CPU_IS_040_OR_060 {
        core::arch::asm!(".chip 68040", "pflusha", ".chip 68k");
    } else if CPU_IS_020_OR_030 {
        core::arch::asm!("pflusha");
    }
}

#[cfg(feature = "CONFIG_MMU")]
#[cfg(not(feature = "CONFIG_SUN3"))]
pub unsafe fn flush_tlb_mm(mm: *mut mm_struct) {
    if (*mm).active_mm == current.active_mm { __flush_tlb(); }
}

#[cfg(feature = "CONFIG_MMU")]
#[cfg(not(feature = "CONFIG_SUN3"))]
pub unsafe fn flush_tlb_page(vma: *mut vm_area_struct, addr: usize) {
    if (*vma).vm_mm == current.active_mm { __flush_tlb_one(addr); }
}

#[cfg(feature = "CONFIG_MMU")]
#[cfg(not(feature = "CONFIG_SUN3"))]
pub unsafe fn flush_tlb_range(vma: *mut vm_area_struct, _start: usize, _end: usize) {
    if (*vma).vm_mm == current.active_mm { __flush_tlb(); }
}

#[cfg(feature = "CONFIG_MMU")]
#[cfg(not(feature = "CONFIG_SUN3"))]
pub unsafe fn flush_tlb_kernel_range(_start: usize, _end: usize) { flush_tlb_all(); }

#[cfg(feature = "CONFIG_MMU")]
#[cfg(feature = "CONFIG_SUN3")]
extern "C" {
    static mut sun3_reserved_pmeg: [core::ffi::c_char; SUN3_PMEGS_NUM];
    static mut pmeg_vaddr: [usize; SUN3_PMEGS_NUM];
    static mut pmeg_alloc: [u8; SUN3_PMEGS_NUM];
    static mut pmeg_ctx: [u8; SUN3_PMEGS_NUM];
}

#[cfg(feature = "CONFIG_MMU")]
#[cfg(feature = "CONFIG_SUN3")]
pub unsafe fn flush_tlb_all() {
    let oldctx = sun3_get_context();
    let mut addr: usize = 0;
    while addr < TASK_SIZE {
        let mut ctx: u8 = 0;
        while ctx < 8 {
            sun3_put_context(ctx);
            sun3_put_segmap(addr, SUN3_INVALID_PMEG);
            ctx += 1;
        }
        addr += SUN3_PMEG_SIZE;
    }
    sun3_put_context(oldctx);
    addr = 0;
    while addr < SUN3_INVALID_PMEG {
        if pmeg_alloc[addr] == 1 {
            pmeg_alloc[addr] = 0;
            pmeg_ctx[addr] = 0;
            pmeg_vaddr[addr] = 0;
        }
        addr += 1;
    }
}

#[cfg(feature = "CONFIG_MMU")]
#[cfg(feature = "CONFIG_SUN3")]
pub unsafe fn flush_tlb_mm(mm: *mut mm_struct) {
    let oldctx = sun3_get_context();
    sun3_put_context((*mm).context);
    let mut i = 0;
    while i < TASK_SIZE {
        let seg = sun3_get_segmap(i);
        if seg != SUN3_INVALID_PMEG {
            sun3_put_segmap(i, SUN3_INVALID_PMEG);
            pmeg_alloc[seg] = 0; pmeg_ctx[seg] = 0; pmeg_vaddr[seg] = 0;
        }
        i += SUN3_PMEG_SIZE;
    }
    sun3_put_context(oldctx);
}

#[cfg(feature = "CONFIG_MMU")]
#[cfg(feature = "CONFIG_SUN3")]
pub unsafe fn flush_tlb_page(vma: *mut vm_area_struct, mut addr: usize) {
    let oldctx = sun3_get_context();
    sun3_put_context((*(*vma).vm_mm).context);
    addr &= !SUN3_PMEG_MASK;
    let i = sun3_get_segmap(addr);
    if i != SUN3_INVALID_PMEG {
        pmeg_alloc[i] = 0; pmeg_ctx[i] = 0; pmeg_vaddr[i] = 0;
        sun3_put_segmap(addr, SUN3_INVALID_PMEG);
    }
    sun3_put_context(oldctx);
}

#[cfg(feature = "CONFIG_MMU")]
#[cfg(feature = "CONFIG_SUN3")]
pub unsafe fn flush_tlb_range(vma: *mut vm_area_struct, mut start: usize, end: usize) {
    let mm = (*vma).vm_mm;
    start &= !SUN3_PMEG_MASK;
    let oldctx = sun3_get_context(); sun3_put_context((*mm).context);
    while start < end {
        let seg = sun3_get_segmap(start);
        if seg != SUN3_INVALID_PMEG {
            if pmeg_ctx[seg] == (*mm).context { pmeg_alloc[seg] = 0; pmeg_ctx[seg] = 0; pmeg_vaddr[seg] = 0; }
            sun3_put_segmap(start, SUN3_INVALID_PMEG);
        }
        start += SUN3_PMEG_SIZE;
    }
    sun3_put_context(oldctx);
}

#[cfg(feature = "CONFIG_MMU")]
#[cfg(feature = "CONFIG_SUN3")]
pub unsafe fn flush_tlb_kernel_range(_start: usize, _end: usize) { flush_tlb_all(); }

#[cfg(feature = "CONFIG_MMU")]
#[cfg(feature = "CONFIG_SUN3")]
pub unsafe fn flush_tlb_kernel_page(addr: usize) { sun3_put_segmap(addr & !(SUN3_PMEG_SIZE - 1), SUN3_INVALID_PMEG); }

#[cfg(not(feature = "CONFIG_MMU"))]
pub unsafe fn __flush_tlb() { BUG(); }
#[cfg(not(feature = "CONFIG_MMU"))]
pub unsafe fn __flush_tlb_one(_addr: usize) { BUG(); }
#[cfg(not(feature = "CONFIG_MMU"))]
pub unsafe fn flush_tlb() { __flush_tlb(); }
#[cfg(not(feature = "CONFIG_MMU"))]
pub unsafe fn flush_tlb_all() { BUG(); }
#[cfg(not(feature = "CONFIG_MMU"))]
pub unsafe fn flush_tlb_mm(_mm: *mut mm_struct) { BUG(); }
#[cfg(not(feature = "CONFIG_MMU"))]
pub unsafe fn flush_tlb_page(_vma: *mut vm_area_struct, _addr: usize) { BUG(); }
#[cfg(not(feature = "CONFIG_MMU"))]
pub unsafe fn flush_tlb_range(_vma: *mut vm_area_struct, _start: usize, _end: usize) { BUG(); }
#[cfg(not(feature = "CONFIG_MMU"))]
pub unsafe fn flush_tlb_kernel_page(_addr: usize) { BUG(); }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
