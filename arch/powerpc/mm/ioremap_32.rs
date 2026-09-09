// SPDX-License-Identifier: GPL-2.0-or-later

// C dependencies: <linux/io.h>, <linux/slab.h>, <linux/vmalloc.h>,
// <mm/mmu_decl.h>

use core::ffi::c_void;

// These types, constants, globals, and functions are supplied by the surrounding
// kernel translation units.
extern "C" {
    static mut ioremap_bot: c_ulong;

    fn pgprot_cached_wthru(prot: pgprot_t) -> pgprot_t;
    fn __ioremap_caller(
        addr: phys_addr_t,
        size: c_ulong,
        prot: pgprot_t,
        caller: *mut c_void,
    ) -> *mut c_void;
    fn slab_is_available() -> bool;
    fn virt_to_phys(addr: *const c_void) -> phys_addr_t;
    fn page_is_ram(pfn: c_ulong) -> bool;
    fn __phys_to_pfn(addr: phys_addr_t) -> c_ulong;
    fn p_block_mapped(addr: phys_addr_t) -> c_ulong;
    fn generic_ioremap_prot(addr: phys_addr_t, size: c_ulong, prot: pgprot_t) -> *mut c_void;
    fn early_ioremap_range(
        addr: c_ulong,
        p: phys_addr_t,
        size: c_ulong,
        prot: pgprot_t,
    ) -> c_int;
    fn v_block_mapped(addr: c_ulong) -> bool;
    fn generic_iounmap(addr: *mut c_void);
}

// Build-time kernel definitions supplied externally.
type c_ulong = core::ffi::c_ulong;
type c_int = core::ffi::c_int;
type phys_addr_t = u64;
type pgprot_t = usize;

const SZ_16M: phys_addr_t = 16 * 1024 * 1024;
const PAGE_MASK: phys_addr_t = !(4096 - 1);
const PAGE_SIZE: c_ulong = 4096;
extern "C" {
    static _ISA_MEM_BASE: phys_addr_t;
    static PAGE_KERNEL: pgprot_t;
}

pub unsafe fn ioremap_wt(addr: phys_addr_t, size: c_ulong) -> *mut c_void {
    let prot = pgprot_cached_wthru(PAGE_KERNEL);

    __ioremap_caller(addr, size, prot, core::ptr::null_mut())
}

pub unsafe fn __ioremap_caller(
    mut addr: phys_addr_t,
    mut size: c_ulong,
    prot: pgprot_t,
    caller: *mut c_void,
) -> *mut c_void {
    let mut v: c_ulong;
    let p: phys_addr_t;
    let offset: phys_addr_t;
    let err: c_int;

    /*
     * If the address lies within the first 16 MB, assume it's in ISA
     * memory space
     */
    if addr < SZ_16M {
        addr = addr.wrapping_add(_ISA_MEM_BASE);
    }

    /*
     * Choose an address to map it to.
     * Once the vmalloc system is running, we use it.
     * Before then, we use space going down from IOREMAP_TOP
     * (ioremap_bot records where we're up to).
     */
    p = addr & PAGE_MASK;
    offset = addr & !PAGE_MASK;
    size = (addr.wrapping_add(size as phys_addr_t).wrapping_add(4095) & PAGE_MASK)
        .wrapping_sub(p) as c_ulong;

    /* CONFIG_CRASH_DUMP condition is preserved from the C source. */
    #[cfg(not(CONFIG_CRASH_DUMP))]
    {
        /*
         * Don't allow anybody to remap normal RAM that we're using.
         * mem_init() sets high_memory so only do the check after that.
         */
        if slab_is_available()
            && p <= virt_to_phys(unsafe { (high_memory as *mut u8).sub(1) } as *const c_void)
            && page_is_ram(__phys_to_pfn(p))
        {
            pr_warn("%s(): phys addr 0x%llx is RAM lr %ps\n", "__ioremap_caller", p, caller);
            return core::ptr::null_mut();
        }
    }

    if size == 0 {
        return core::ptr::null_mut();
    }

    /*
     * Is it already mapped?  Perhaps overlapped by a previous
     * mapping.
     */
    v = p_block_mapped(p);
    if v != 0 {
        return (v as *mut u8).add(offset as usize) as *mut c_void;
    }

    if slab_is_available() {
        return generic_ioremap_prot(addr, size, prot);
    }

    /*
     * Should check if it is a candidate for a BAT mapping
     */
    pr_warn(
        "ioremap() called early from %pS. Use early_ioremap() instead\n",
        caller,
    );

    err = early_ioremap_range(ioremap_bot - size - PAGE_SIZE, p, size, prot);
    if err != 0 {
        return core::ptr::null_mut();
    }
    ioremap_bot -= size + PAGE_SIZE;

    (ioremap_bot as *mut u8).add(offset as usize) as *mut c_void
}

pub unsafe fn iounmap(addr: *mut c_void) {
    /*
     * If mapped by BATs then there is nothing to do.
     * Calling vfree() generates a benign warning.
     */
    if v_block_mapped(addr as c_ulong) {
        return;
    }

    generic_iounmap(addr);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
