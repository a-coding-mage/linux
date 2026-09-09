/*
 * linux/arch/m68k/mm/sun3kmap.c
 *
 * Copyright (C) 2002 Sam Creasey <sammy@sammy.net>
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

// C dependencies supplied by the surrounding kernel translation unit.

extern "C" {
    fn mmu_emu_map_pmeg(context: i32, vaddr: i32);
    fn pfn_pte(pfn: usize, prot: usize) -> Pte;
    fn pte_val(pte: Pte) -> usize;
    fn sun3_put_pte(virt: usize, pte: usize);
    fn sun3_get_segmap(addr: usize) -> i32;
    fn sun3_get_context() -> i32;
    fn get_vm_area(size: usize, flags: usize) -> *mut VmStruct;
    fn vfree(addr: *mut core::ffi::c_void);
}

#[repr(C)]
pub struct Pte {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct VmStruct {
    pub addr: *mut core::ffi::c_void,
    pub size: usize,
}

extern "C" {
    static SUN3_INVALID_PMEG: i32;
    static SUN3_PMEG_MASK: usize;
    static SUN3_PMEG_SIZE: usize;
    static PAGE_SHIFT: usize;
    static PAGE_SIZE: usize;
    static PAGE_MASK: usize;
    static PAGE_KERNEL: usize;
    static SUN3_PAGE_TYPE_IO: usize;
    static VM_IOREMAP: usize;
}

#[inline]
unsafe fn do_page_mapin(mut phys: usize, virt: usize, map_type: usize) {
    let ptep = pfn_pte(phys >> PAGE_SHIFT, PAGE_KERNEL);
    let mut pte = pte_val(ptep);
    pte |= map_type;

    sun3_put_pte(virt, pte);

    /* SUN3_KMAP_DEBUG: pr_info("mapin:"); print_pte_vaddr(virt); */
    let _ = &mut phys;
}

#[inline]
unsafe fn do_pmeg_mapin(mut phys: usize, mut virt: usize, map_type: usize, mut pages: i32) {
    if sun3_get_segmap(virt & !SUN3_PMEG_MASK) == SUN3_INVALID_PMEG {
        mmu_emu_map_pmeg(sun3_get_context(), virt as i32);
    }

    while pages != 0 {
        do_page_mapin(phys, virt, map_type);
        phys = phys.wrapping_add(PAGE_SIZE);
        virt = virt.wrapping_add(PAGE_SIZE);
        pages -= 1;
    }
}

pub unsafe fn sun3_ioremap(mut phys: usize, size: usize, map_type: usize) -> *mut core::ffi::c_void {
    if size == 0 {
        return core::ptr::null_mut();
    }

    let offset = phys & (PAGE_SIZE - 1);
    phys &= !(PAGE_SIZE - 1);

    let size = (size.wrapping_add(offset) + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
    let area = get_vm_area(size, VM_IOREMAP);
    if area.is_null() {
        return core::ptr::null_mut();
    }

    /* SUN3_KMAP_DEBUG: pr_info("ioremap: got virt %p size %lx(%lx)\n", ...); */
    let mut pages = (size / PAGE_SIZE) as i32;
    let mut virt = (*area).addr as usize;
    let ret = virt.wrapping_add(offset);

    while pages != 0 {
        let mut seg_pages = ((SUN3_PMEG_SIZE - (virt & SUN3_PMEG_MASK)) / PAGE_SIZE) as i32;
        if seg_pages > pages {
            seg_pages = pages;
        }

        do_pmeg_mapin(phys, virt, map_type, seg_pages);
        pages -= seg_pages;
        phys = phys.wrapping_add((seg_pages as usize).wrapping_mul(PAGE_SIZE));
        virt = virt.wrapping_add((seg_pages as usize).wrapping_mul(PAGE_SIZE));
    }

    ret as *mut core::ffi::c_void
}

pub unsafe fn __ioremap(phys: usize, size: usize, _cache: i32) -> *mut core::ffi::c_void {
    sun3_ioremap(phys, size, SUN3_PAGE_TYPE_IO)
}

pub unsafe fn iounmap(addr: *mut core::ffi::c_void) {
    vfree((addr as usize & PAGE_MASK) as *mut core::ffi::c_void);
}

/* sun3_map_test(addr, val) -- Reads a byte from addr, storing to val,
 * trapping the potential read fault.  Returns 0 if the access faulted,
 * 1 on success.
 *
 * This function is primarily used to check addresses on the VME bus.
 *
 * Mucking with the page fault handler seems a little hackish to me, but
 * SunOS, NetBSD, and Mach all implemented this check in such a manner,
 * so I figure we're allowed.
 */
pub unsafe fn sun3_map_test(addr: usize, val: *mut u8) -> i32 {
    // The m68k exception-table inline assembly is retained as an external
    // ABI hook; its implementation is architecture-specific.
    extern "C" {
        fn sun3_map_test_asm(addr: usize, val: *mut u8) -> i32;
    }
    sun3_map_test_asm(addr, val)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
