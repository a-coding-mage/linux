// SPDX-License-Identifier: GPL-2.0
/* Common early_ioremap support; dependencies are supplied by the kernel. */

// The following kernel types, constants, macros, and functions are external
// dependencies corresponding to the included C headers.

#[cfg(CONFIG_MMU)]
static mut EARLY_IOREMAP_DEBUG: i32 = 0;

#[cfg(CONFIG_MMU)]
unsafe fn early_ioremap_debug_setup(_str: *mut core::ffi::c_char) -> i32 {
    EARLY_IOREMAP_DEBUG = 1;
    0
}

#[cfg(CONFIG_MMU)]
static mut AFTER_PAGING_INIT: i32 = 0;

#[cfg(CONFIG_MMU)]
#[no_mangle]
pub unsafe extern "C" fn early_memremap_pgprot_adjust(
    _phys_addr: resource_size_t,
    _size: libc::c_ulong,
    prot: pgprot_t,
) -> pgprot_t {
    prot
}

#[cfg(CONFIG_MMU)]
pub unsafe extern "C" fn early_ioremap_reset() {
    AFTER_PAGING_INIT = 1;
}

#[cfg(CONFIG_MMU)]
#[inline]
unsafe fn __late_set_fixmap(_idx: enum_fixed_addresses, _phys: phys_addr_t, _prot: pgprot_t) {
    BUG();
}

#[cfg(CONFIG_MMU)]
#[inline]
unsafe fn __late_clear_fixmap(_idx: enum_fixed_addresses) {
    BUG();
}

#[cfg(CONFIG_MMU)]
static mut PREV_MAP: [*mut core::ffi::c_void; FIX_BTMAPS_SLOTS as usize] =
    [core::ptr::null_mut(); FIX_BTMAPS_SLOTS as usize];
#[cfg(CONFIG_MMU)]
static mut PREV_SIZE: [libc::c_ulong; FIX_BTMAPS_SLOTS as usize] =
    [0; FIX_BTMAPS_SLOTS as usize];
#[cfg(CONFIG_MMU)]
static mut SLOT_VIRT: [libc::c_ulong; FIX_BTMAPS_SLOTS as usize] =
    [0; FIX_BTMAPS_SLOTS as usize];

#[cfg(CONFIG_MMU)]
pub unsafe extern "C" fn early_ioremap_setup() {
    let mut i = 0;
    while i < FIX_BTMAPS_SLOTS {
        WARN_ON_ONCE(!PREV_MAP[i as usize].is_null());
        SLOT_VIRT[i as usize] = __fix_to_virt(FIX_BTMAP_BEGIN - NR_FIX_BTMAPS * i);
        i += 1;
    }
}

#[cfg(CONFIG_MMU)]
unsafe fn check_early_ioremap_leak() -> i32 {
    let mut count = 0;
    let mut i = 0;
    while i < FIX_BTMAPS_SLOTS {
        if !PREV_MAP[i as usize].is_null() { count += 1; }
        i += 1;
    }
    if WARN(count != 0, "Debug warning: early ioremap leak of %d areas detected.\nplease boot with early_ioremap_debug and report the dmesg.\n", count) { return 1; }
    0
}

#[cfg(CONFIG_MMU)]
unsafe fn __early_ioremap(phys_addr: resource_size_t, size: libc::c_ulong, prot: pgprot_t) -> *mut core::ffi::c_void {
    let mut phys_addr = phys_addr;
    let mut size = size;
    let offset: libc::c_ulong;
    let last_addr: resource_size_t;
    let mut nrpages: libc::c_uint;
    let idx: enum_fixed_addresses;
    let mut slot: i32 = -1;
    let mut i = 0;

    WARN_ON(system_state >= SYSTEM_RUNNING);
    while i < FIX_BTMAPS_SLOTS {
        if PREV_MAP[i as usize].is_null() { slot = i; break; }
        i += 1;
    }
    if WARN(slot < 0, "%s(%pa, %08lx) not found slot\n", __func__, &phys_addr, size) { return core::ptr::null_mut(); }
    last_addr = phys_addr + size - 1;
    if WARN_ON(size == 0 || last_addr < phys_addr) { return core::ptr::null_mut(); }
    PREV_SIZE[slot as usize] = size;
    offset = offset_in_page(phys_addr);
    phys_addr &= PAGE_MASK;
    size = PAGE_ALIGN(last_addr + 1) - phys_addr;
    nrpages = (size >> PAGE_SHIFT) as libc::c_uint;
    if WARN_ON(nrpages > NR_FIX_BTMAPS) { return core::ptr::null_mut(); }
    idx = FIX_BTMAP_BEGIN - NR_FIX_BTMAPS * slot;
    let mut idx = idx;
    while nrpages > 0 {
        if AFTER_PAGING_INIT != 0 { __late_set_fixmap(idx, phys_addr, prot); }
        else { __early_set_fixmap(idx, phys_addr, prot); }
        phys_addr += PAGE_SIZE;
        idx -= 1;
        nrpages -= 1;
    }
    PREV_MAP[slot as usize] = (offset + SLOT_VIRT[slot as usize]) as *mut core::ffi::c_void;
    PREV_MAP[slot as usize]
}

#[cfg(CONFIG_MMU)]
pub unsafe extern "C" fn early_ioremap(phys_addr: resource_size_t, size: libc::c_ulong) -> *mut core::ffi::c_void {
    __early_ioremap(phys_addr, size, FIXMAP_PAGE_IO)
}

#[cfg(CONFIG_MMU)]
pub unsafe extern "C" fn early_iounmap(addr: *mut core::ffi::c_void, size: libc::c_ulong) {
    let mut slot: i32 = -1;
    let mut i = 0;
    while i < FIX_BTMAPS_SLOTS { if PREV_MAP[i as usize] == addr { slot = i; break; } i += 1; }
    if WARN(slot < 0, "%s(%p, %08lx) not found slot\n", __func__, addr, size) { return; }
    if WARN(PREV_SIZE[slot as usize] != size, "%s(%p, %08lx) [%d] size not consistent %08lx\n", __func__, addr, size, slot, PREV_SIZE[slot as usize]) { return; }
    let virt_addr = addr as libc::c_ulong;
    if WARN_ON(virt_addr < fix_to_virt(FIX_BTMAP_BEGIN)) { return; }
    let offset = offset_in_page(virt_addr);
    let mut nrpages = (PAGE_ALIGN(offset + size) >> PAGE_SHIFT) as libc::c_uint;
    let mut idx = FIX_BTMAP_BEGIN - NR_FIX_BTMAPS * slot;
    while nrpages > 0 {
        if AFTER_PAGING_INIT != 0 { __late_clear_fixmap(idx); }
        else { __early_set_fixmap(idx, 0, FIXMAP_PAGE_CLEAR); }
        idx -= 1; nrpages -= 1;
    }
    PREV_MAP[slot as usize] = core::ptr::null_mut();
}

#[cfg(CONFIG_MMU)]
pub unsafe extern "C" fn early_memremap(phys_addr: resource_size_t, size: libc::c_ulong) -> *mut core::ffi::c_void {
    __early_ioremap(phys_addr, size, early_memremap_pgprot_adjust(phys_addr, size, FIXMAP_PAGE_NORMAL))
}

#[cfg(CONFIG_MMU)]
pub unsafe extern "C" fn early_memremap_ro(phys_addr: resource_size_t, size: libc::c_ulong) -> *mut core::ffi::c_void {
    __early_ioremap(phys_addr, size, early_memremap_pgprot_adjust(phys_addr, size, FIXMAP_PAGE_RO))
}

#[cfg(CONFIG_MMU)]
pub unsafe extern "C" fn early_memremap_prot(phys_addr: resource_size_t, size: libc::c_ulong, prot_val: libc::c_ulong) -> *mut core::ffi::c_void {
    __early_ioremap(phys_addr, size, __pgprot(prot_val))
}

#[cfg(CONFIG_MMU)]
pub unsafe extern "C" fn copy_from_early_mem(dest: *mut core::ffi::c_void, src: phys_addr_t, size: libc::c_ulong) -> i32 {
    let mut dest = dest;
    let mut src = src;
    let mut size = size;
    while size != 0 {
        let slop = offset_in_page(src);
        let mut clen = size;
        if clen > MAX_MAP_CHUNK - slop { clen = MAX_MAP_CHUNK - slop; }
        let p = early_memremap(src & PAGE_MASK, clen + slop);
        if p.is_null() { return -ENOMEM; }
        memcpy(dest, (p as *mut u8).add(slop as usize) as *mut core::ffi::c_void, clen);
        early_memunmap(p, clen + slop);
        dest = (dest as *mut u8).add(clen as usize) as *mut core::ffi::c_void;
        src += clen;
        size -= clen;
    }
    0
}

#[cfg(not(CONFIG_MMU))]
pub unsafe extern "C" fn early_ioremap(phys_addr: resource_size_t, _size: libc::c_ulong) -> *mut core::ffi::c_void { phys_addr as *mut core::ffi::c_void }
#[cfg(not(CONFIG_MMU))]
pub unsafe extern "C" fn early_memremap(phys_addr: resource_size_t, _size: libc::c_ulong) -> *mut core::ffi::c_void { phys_addr as *mut core::ffi::c_void }
#[cfg(not(CONFIG_MMU))]
pub unsafe extern "C" fn early_memremap_ro(phys_addr: resource_size_t, _size: libc::c_ulong) -> *mut core::ffi::c_void { phys_addr as *mut core::ffi::c_void }
#[cfg(not(CONFIG_MMU))]
pub unsafe extern "C" fn early_iounmap(_addr: *mut core::ffi::c_void, _size: libc::c_ulong) {}

pub unsafe extern "C" fn early_memunmap(addr: *mut core::ffi::c_void, size: libc::c_ulong) {
    early_iounmap(addr, size);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
