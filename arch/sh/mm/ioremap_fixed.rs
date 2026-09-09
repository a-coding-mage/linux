// SPDX-License-Identifier: GPL-2.0
/*
 * Re-map IO memory to kernel address space so that we can access it.
 *
 * These functions should only be used when it is necessary to map a
 * physical address space into the kernel address space before ioremap()
 * can be used, e.g. early in boot before paging_init().
 *
 * Copyright (C) 2009  Matt Fleming
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/vmalloc.h, linux/ioport.h, linux/module.h, linux/mm.h, linux/io.h,
// linux/memblock.h, linux/proc_fs.h, asm/fixmap.h, asm/page.h,
// asm/addrspace.h, asm/cacheflush.h, asm/tlbflush.h, asm/mmu.h,
// asm/mmu_context.h, and "ioremap.h".

#[repr(C)]
struct ioremap_map {
    addr: *mut core::ffi::c_void,
    size: usize,
    fixmap_addr: usize,
}

static mut ioremap_maps: [ioremap_map; FIX_N_IOREMAPS] = [
    ioremap_map {
        addr: core::ptr::null_mut(),
        size: 0,
        fixmap_addr: 0,
    };
    FIX_N_IOREMAPS
];

pub unsafe extern "C" fn ioremap_fixed_init() {
    let mut map: *mut ioremap_map;
    let mut i: i32;

    i = 0;
    while i < FIX_N_IOREMAPS as i32 {
        map = &mut ioremap_maps[i as usize];
        (*map).fixmap_addr = __fix_to_virt(FIX_IOREMAP_BEGIN + i as usize);
        i += 1;
    }
}

pub unsafe extern "C" fn ioremap_fixed(
    mut phys_addr: phys_addr_t,
    mut size: usize,
    mut prot: pgprot_t,
) -> *mut core::ffi::c_void {
    let mut idx0: fixed_addresses;
    let mut idx: fixed_addresses;
    let mut map: *mut ioremap_map;
    let mut nrpages: u32;
    let mut offset: usize;
    let mut i: i32;
    let mut slot: i32;

    /*
     * Mappings have to be page-aligned
     */
    offset = phys_addr as usize & !PAGE_MASK;
    phys_addr &= PAGE_MASK as phys_addr_t;
    size = PAGE_ALIGN(phys_addr as usize + size) - phys_addr as usize;

    slot = -1;
    i = 0;
    while i < FIX_N_IOREMAPS as i32 {
        map = &mut ioremap_maps[i as usize];
        if (*map).addr.is_null() {
            (*map).size = size;
            slot = i;
            break;
        }
        i += 1;
    }

    if slot < 0 {
        return core::ptr::null_mut();
    }

    /*
     * Mappings have to fit in the FIX_IOREMAP area.
     */
    nrpages = (size >> PAGE_SHIFT) as u32;
    if nrpages > FIX_N_IOREMAPS as u32 {
        return core::ptr::null_mut();
    }

    /*
     * Ok, go for it..
     */
    idx0 = FIX_IOREMAP_BEGIN + slot as usize;
    idx = idx0;
    while nrpages > 0 {
        pgprot_val(&mut prot) |= _PAGE_WIRED;
        __set_fixmap(idx, phys_addr, prot);
        phys_addr += PAGE_SIZE as phys_addr_t;
        idx += 1;
        nrpages -= 1;
    }

    map = &mut ioremap_maps[slot as usize];
    (*map).addr = (offset + (*map).fixmap_addr) as *mut core::ffi::c_void;
    (*map).addr
}

pub unsafe extern "C" fn iounmap_fixed(addr: *mut core::ffi::c_void) -> i32 {
    let mut idx: fixed_addresses;
    let mut map: *mut ioremap_map;
    let mut nrpages: u32;
    let mut i: i32;
    let mut slot: i32;

    slot = -1;
    i = 0;
    while i < FIX_N_IOREMAPS as i32 {
        map = &mut ioremap_maps[i as usize];
        if (*map).addr == addr {
            slot = i;
            break;
        }
        i += 1;
    }

    /*
     * If we don't match, it's not for us.
     */
    if slot < 0 {
        return -EINVAL;
    }

    map = &mut ioremap_maps[slot as usize];
    nrpages = ((*map).size >> PAGE_SHIFT) as u32;

    idx = FIX_IOREMAP_BEGIN + slot as usize + nrpages as usize - 1;
    while nrpages > 0 {
        __clear_fixmap(idx, __pgprot(_PAGE_WIRED));
        idx -= 1;
        nrpages -= 1;
    }

    (*map).size = 0;
    (*map).addr = core::ptr::null_mut();

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
