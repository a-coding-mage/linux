// Dependency supplied by <asm/page.h>.

// The C header defines pgprot_framebuffer as a static inline helper and then
// aliases the macro to the function name.
#[inline]
pub unsafe fn pgprot_framebuffer(
    prot: pgprot_t,
    _vm_start: core::ffi::c_ulong,
    _vm_end: core::ffi::c_ulong,
    _offset: core::ffi::c_ulong,
) -> pgprot_t {
    pgprot_noncached(prot)
}

// MIPS does not define __raw_ I/O macros, so these helpers provide fb_readq()
// and fb_writeq(). The C header's fb_readq/fb_writeq self-alias macros are
// represented directly by the Rust functions.

// Build-time condition preserved from #ifdef CONFIG_64BIT.
#[cfg(CONFIG_64BIT)]
#[inline]
pub unsafe fn fb_readq(addr: *const core::ffi::c_void) -> u64 {
    __raw_readq(addr)
}

#[cfg(CONFIG_64BIT)]
#[inline]
pub unsafe fn fb_writeq(b: u64, addr: *mut core::ffi::c_void) {
    __raw_writeq(b, addr);
}

// External declarations supplied by the included architecture headers.
extern "C" {
    fn pgprot_noncached(prot: pgprot_t) -> pgprot_t;

    #[cfg(CONFIG_64BIT)]
    fn __raw_readq(addr: *const core::ffi::c_void) -> u64;

    #[cfg(CONFIG_64BIT)]
    fn __raw_writeq(b: u64, addr: *mut core::ffi::c_void);
}

// Declarations supplied by <asm/page.h> and <asm-generic/video.h> are not
// duplicated here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
