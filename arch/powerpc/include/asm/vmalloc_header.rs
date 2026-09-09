// Translated from asm/powerpc/vmalloc.h.
//
// The original header includes asm/mmu.h and asm/page.h; their Rust-provided
// declarations are expected to supply `pgprot_t`.

#[cfg(feature = "CONFIG_HAVE_ARCH_HUGE_VMAP")]
extern "C" {
    fn radix_enabled() -> bool;
}

#[cfg(feature = "CONFIG_HAVE_ARCH_HUGE_VMAP")]
#[inline(always)]
fn arch_vmap_pud_supported(_prot: pgprot_t) -> bool {
    // HPT does not cope with large pages in the vmalloc area.
    unsafe { radix_enabled() }
}

#[cfg(feature = "CONFIG_HAVE_ARCH_HUGE_VMAP")]
#[inline(always)]
fn arch_vmap_pmd_supported(_prot: pgprot_t) -> bool {
    unsafe { radix_enabled() }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
