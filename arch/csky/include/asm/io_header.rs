/* SPDX-License-Identifier: GPL-2.0 */

/*
 * I/O memory access primitives. Reads are ordered relative to any
 * following Normal memory access. Writes are ordered relative to any prior
 * Normal memory access.
 *
 * For CACHEV1 (807, 810), store instruction could fast retire, so we need
 * another mb() to prevent st fast retire.
 *
 * For CACHEV2 (860), store instruction with PAGE_ATTR_NO_BUFFERABLE won't
 * fast retire.
 */

extern "C" {
    pub fn readb_relaxed(c: *const core::ffi::c_void) -> u8;
    pub fn readw_relaxed(c: *const core::ffi::c_void) -> u16;
    pub fn readl_relaxed(c: *const core::ffi::c_void) -> u32;

    pub fn writeb_relaxed(v: u8, c: *mut core::ffi::c_void);
    pub fn writew_relaxed(v: u16, c: *mut core::ffi::c_void);
    pub fn writel_relaxed(v: u32, c: *mut core::ffi::c_void);

    pub fn rmb();
    pub fn wmb();
    pub fn mb();

    pub fn ioremap_prot(
        addr: usize,
        size: usize,
        prot: usize,
    ) -> *mut core::ffi::c_void;
}

#[inline]
pub unsafe fn readb(c: *const core::ffi::c_void) -> u8 {
    let v = readb_relaxed(c);
    rmb();
    v
}

#[inline]
pub unsafe fn readw(c: *const core::ffi::c_void) -> u16 {
    let v = readw_relaxed(c);
    rmb();
    v
}

#[inline]
pub unsafe fn readl(c: *const core::ffi::c_void) -> u32 {
    let v = readl_relaxed(c);
    rmb();
    v
}

/* CONFIG_CPU_HAS_CACHEV2 selects the first branch at build time. */
#[cfg(CONFIG_CPU_HAS_CACHEV2)]
#[inline]
pub unsafe fn writeb(v: u8, c: *mut core::ffi::c_void) {
    wmb();
    writeb_relaxed(v, c);
}

#[cfg(CONFIG_CPU_HAS_CACHEV2)]
#[inline]
pub unsafe fn writew(v: u16, c: *mut core::ffi::c_void) {
    wmb();
    writew_relaxed(v, c);
}

#[cfg(CONFIG_CPU_HAS_CACHEV2)]
#[inline]
pub unsafe fn writel(v: u32, c: *mut core::ffi::c_void) {
    wmb();
    writel_relaxed(v, c);
}

#[cfg(not(CONFIG_CPU_HAS_CACHEV2))]
#[inline]
pub unsafe fn writeb(v: u8, c: *mut core::ffi::c_void) {
    wmb();
    writeb_relaxed(v, c);
    mb();
}

#[cfg(not(CONFIG_CPU_HAS_CACHEV2))]
#[inline]
pub unsafe fn writew(v: u16, c: *mut core::ffi::c_void) {
    wmb();
    writew_relaxed(v, c);
    mb();
}

#[cfg(not(CONFIG_CPU_HAS_CACHEV2))]
#[inline]
pub unsafe fn writel(v: u32, c: *mut core::ffi::c_void) {
    wmb();
    writel_relaxed(v, c);
    mb();
}

/* I/O memory mapping functions. */
#[inline]
pub unsafe fn ioremap_wc(addr: usize, size: usize) -> *mut core::ffi::c_void {
    ioremap_prot(
        addr,
        size,
        (_PAGE_IOREMAP & !_CACHE_MASK) | _CACHE_UNCACHED,
    )
}

/* _PAGE_IOREMAP, _CACHE_MASK, and _CACHE_UNCACHED are supplied externally. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
