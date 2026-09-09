// SPDX-License-Identifier: GPL-2.0-or-later

// The declarations below are supplied by the Linux kernel environment.
use core::ffi::c_void;

extern "C" {
    fn pgprot_val(prot: pgprot_t) -> usize;
    fn slab_is_available() -> bool;
    fn generic_ioremap_prot(addr: phys_addr_t, size: c_ulong, prot: pgprot_t) -> *mut c_void;
    fn early_ioremap_range(
        addr: c_ulong,
        phys_addr: phys_addr_t,
        size: c_ulong,
        prot: pgprot_t,
    ) -> i32;
    fn generic_iounmap(token: *const c_void);
    fn pr_warn(format: *const u8, ...);
}

extern "C" {
    static mut ioremap_bot: c_ulong;
}

// Kernel-provided types and constants.
type phys_addr_t = usize;
type c_ulong = usize;
type pgprot_t = usize;

const PAGE_MASK: usize = 0xffff_ffff_ffff_f000;
const PAGE_SIZE: usize = 4096;
const H_PAGE_4K_PFN: usize = 0;

#[inline]
unsafe fn page_align(value: usize) -> usize {
    (value + PAGE_SIZE - 1) & !(PAGE_SIZE - 1)
}

pub unsafe fn __ioremap_caller(
    addr: phys_addr_t,
    mut size: c_ulong,
    prot: pgprot_t,
    caller: *mut c_void,
) -> *mut c_void {
    let paligned: phys_addr_t;
    let offset: phys_addr_t;
    let ret: *mut c_void;
    let err: i32;

    /* We don't support the 4K PFN hack with ioremap */
    if pgprot_val(prot) & H_PAGE_4K_PFN != 0 {
        return core::ptr::null_mut();
    }

    /*
     * Choose an address to map it to. Once the vmalloc system is running,
     * we use it. Before that, we map using addresses going up from
     * ioremap_bot.  vmalloc will use the addresses from IOREMAP_BASE
     * through ioremap_bot.
     */
    paligned = addr & PAGE_MASK;
    offset = addr & !PAGE_MASK;
    size = page_align(addr.wrapping_add(size)) - paligned;

    if size == 0 || paligned == 0 {
        return core::ptr::null_mut();
    }

    if slab_is_available() {
        return generic_ioremap_prot(addr, size, prot);
    }

    // pr_warn("ioremap() called early from %pS. Use early_ioremap() instead\n", caller);
    pr_warn(
        b"ioremap() called early from %pS. Use early_ioremap() instead\n\0".as_ptr(),
        caller,
    );

    err = early_ioremap_range(ioremap_bot, paligned, size, prot);
    if err != 0 {
        return core::ptr::null_mut();
    }

    ret = (ioremap_bot as *mut u8).add(offset) as *mut c_void;
    ioremap_bot = ioremap_bot.wrapping_add(size).wrapping_add(PAGE_SIZE);

    ret
}

/*
 * Unmap an IO region and remove it from vmalloc'd list.
 * Access to IO memory should be serialized by driver.
 */
pub unsafe fn iounmap(token: *const c_void) {
    if !slab_is_available() {
        return;
    }

    generic_iounmap(token);
}

// EXPORT_SYMBOL(iounmap);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
