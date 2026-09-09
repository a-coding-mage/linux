/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * (C) Copyright 1995 1996 Linus Torvalds
 * (C) Copyright 2001, 2002 Ralf Baechle
 */

/* Dependencies are supplied by the corresponding kernel headers. */

#[inline]
fn is_low512(addr: phys_addr_t) -> bool {
    !(addr & !(0x1fffffff_u64 as phys_addr_t) != 0)
}

#[inline]
fn is_kseg1(addr: usize) -> bool {
    (addr & !0x1fffffff_usize) == CKSEG1
}

unsafe extern "C" {
    fn pfn_valid(pfn: c_ulong) -> c_int;
    fn pfn_to_page(pfn: c_ulong) -> *mut page;
    fn PageReserved(page: *mut page) -> c_int;
    fn plat_ioremap(phys_addr: phys_addr_t, size: c_ulong, flags: c_ulong) -> *mut core::ffi::c_void;
    fn fixup_bigphys_addr(phys_addr: phys_addr_t, size: c_ulong) -> phys_addr_t;
    fn slab_is_available() -> c_int;
    fn walk_system_ram_range(
        start_pfn: c_ulong,
        nr_pages: c_ulong,
        arg: *mut core::ffi::c_void,
        func: unsafe extern "C" fn(c_ulong, c_ulong, *mut core::ffi::c_void) -> c_int,
    ) -> c_int;
    fn get_vm_area(size: c_ulong, flags: c_ulong) -> *mut vm_struct;
    fn free_vm_area(area: *mut vm_struct);
    fn ioremap_page_range(
        addr: c_ulong,
        end: c_ulong,
        phys_addr: phys_addr_t,
        prot: pgprot_t,
    ) -> c_int;
    fn vunmap(addr: *mut core::ffi::c_void);
    fn plat_iounmap(addr: *const core::ffi::c_void) -> c_int;
}

unsafe extern "C" fn __ioremap_check_ram(
    start_pfn: c_ulong,
    nr_pages: c_ulong,
    _arg: *mut core::ffi::c_void,
) -> c_int {
    let mut i: c_ulong = 0;

    while i < nr_pages {
        if pfn_valid(start_pfn.wrapping_add(i)) != 0
            && PageReserved(pfn_to_page(start_pfn.wrapping_add(i))) == 0
        {
            return 1;
        }
        i = i.wrapping_add(1);
    }

    0
}

/*
 * ioremap_prot     -   map bus memory into CPU space
 * @phys_addr:    bus address of the memory
 * @size:      size of the resource to map
 *
 * ioremap_prot gives the caller control over cache coherency attributes (CCA)
 */
#[no_mangle]
pub unsafe extern "C" fn ioremap_prot(
    mut phys_addr: phys_addr_t,
    mut size: c_ulong,
    prot: pgprot_t,
) -> *mut core::ffi::c_void {
    let mut flags: c_ulong = pgprot_val(prot) & _CACHE_MASK;
    let mut offset: c_ulong;
    let mut pfn: c_ulong;
    let mut last_pfn: c_ulong;
    let area: *mut vm_struct;
    let last_addr: phys_addr_t;
    let vaddr: c_ulong;
    let cpu_addr: *mut core::ffi::c_void;

    cpu_addr = plat_ioremap(phys_addr, size, flags);
    if !cpu_addr.is_null() {
        return cpu_addr;
    }

    phys_addr = fixup_bigphys_addr(phys_addr, size);

    /* Don't allow wraparound or zero size */
    last_addr = phys_addr.wrapping_add(size as phys_addr_t).wrapping_sub(1);
    if size == 0 || last_addr < phys_addr {
        return core::ptr::null_mut();
    }

    /*
     * Map uncached objects in the low 512mb of address space using KSEG1,
     * otherwise map using page tables.
     */
    if is_low512(phys_addr) && is_low512(last_addr) && flags == _CACHE_UNCACHED {
        return CKSEG1ADDR(phys_addr) as usize as *mut core::ffi::c_void;
    }

    /* Early remaps should use the unmapped regions til' VM is available */
    if slab_is_available() == 0 {
        return core::ptr::null_mut();
    }

    /*
     * Don't allow anybody to remap RAM that may be allocated by the page
     * allocator, since that could lead to races & data clobbering.
     */
    pfn = PFN_DOWN(phys_addr);
    last_pfn = PFN_DOWN(last_addr);
    if walk_system_ram_range(
        pfn,
        last_pfn.wrapping_sub(pfn).wrapping_add(1),
        core::ptr::null_mut(),
        __ioremap_check_ram,
    ) == 1 {
        return core::ptr::null_mut();
    }

    /* Mappings have to be page-aligned */
    offset = phys_addr as c_ulong & !PAGE_MASK;
    phys_addr &= PAGE_MASK as phys_addr_t;
    size = PAGE_ALIGN(last_addr.wrapping_add(1)) - phys_addr as c_ulong;

    /* Ok, go for it.. */
    area = get_vm_area(size, VM_IOREMAP);
    if area.is_null() {
        return core::ptr::null_mut();
    }
    vaddr = (*area).addr as c_ulong;

    flags |= _PAGE_GLOBAL | _PAGE_PRESENT | __READABLE | __WRITEABLE;
    if ioremap_page_range(vaddr, vaddr.wrapping_add(size), phys_addr, __pgprot(flags)) != 0 {
        free_vm_area(area);
        return core::ptr::null_mut();
    }

    vaddr.wrapping_add(offset) as *mut core::ffi::c_void
}

#[no_mangle]
pub unsafe extern "C" fn iounmap(addr: *const core::ffi::c_void) {
    if plat_iounmap(addr) == 0 && !is_kseg1(addr as usize) {
        vunmap((addr as usize & PAGE_MASK) as *mut core::ffi::c_void);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
