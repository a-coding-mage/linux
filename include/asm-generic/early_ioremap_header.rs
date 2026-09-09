/* SPDX-License-Identifier: GPL-2.0 */

/*
 * early_ioremap() and early_iounmap() are for temporary early boot-time
 * mappings, before the real ioremap() is functional.
 *
 * The C __iomem annotation is a type-checking annotation and has no direct
 * Rust representation; the pointer remains a raw pointer here.
 */
extern "C" {
    pub fn early_ioremap(phys_addr: resource_size_t, size: ::core::ffi::c_ulong)
        -> *mut ::core::ffi::c_void;
    pub fn early_memremap(phys_addr: resource_size_t, size: ::core::ffi::c_ulong)
        -> *mut ::core::ffi::c_void;
    pub fn early_memremap_ro(phys_addr: resource_size_t, size: ::core::ffi::c_ulong)
        -> *mut ::core::ffi::c_void;
    pub fn early_memremap_prot(
        phys_addr: resource_size_t,
        size: ::core::ffi::c_ulong,
        prot_val: ::core::ffi::c_ulong,
    ) -> *mut ::core::ffi::c_void;
    pub fn early_iounmap(addr: *mut ::core::ffi::c_void, size: ::core::ffi::c_ulong);
    pub fn early_memunmap(addr: *mut ::core::ffi::c_void, size: ::core::ffi::c_ulong);
}

/* CONFIG_GENERIC_EARLY_IOREMAP && CONFIG_MMU */
#[cfg(all(
    feature = "CONFIG_GENERIC_EARLY_IOREMAP",
    feature = "CONFIG_MMU"
))]
extern "C" {
    /* Arch-specific initialization */
    pub fn early_ioremap_init();

    /* Generic initialization called by architecture code */
    pub fn early_ioremap_setup();

    /*
     * Called as last step in paging_init() so library can act
     * accordingly for subsequent map/unmap requests.
     */
    pub fn early_ioremap_reset();

    /* Early copy from unmapped memory to kernel mapped memory. */
    pub fn copy_from_early_mem(
        dest: *mut ::core::ffi::c_void,
        src: phys_addr_t,
        size: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
}

#[cfg(not(all(
    feature = "CONFIG_GENERIC_EARLY_IOREMAP",
    feature = "CONFIG_MMU"
)))]
#[inline]
pub unsafe fn early_ioremap_init() {}

#[cfg(not(all(
    feature = "CONFIG_GENERIC_EARLY_IOREMAP",
    feature = "CONFIG_MMU"
)))]
#[inline]
pub unsafe fn early_ioremap_setup() {}

#[cfg(not(all(
    feature = "CONFIG_GENERIC_EARLY_IOREMAP",
    feature = "CONFIG_MMU"
)))]
#[inline]
pub unsafe fn early_ioremap_reset() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
