// Translated from x86/include/asm/vmalloc.h.
//
// Dependencies supplied by other translation units:
// - `pgprot_t`
// - `boot_cpu_has`
// - `X86_FEATURE_GBPAGES`
// - `X86_FEATURE_PSE`

// #ifdef CONFIG_HAVE_ARCH_HUGE_VMAP

// #ifdef CONFIG_X86_64
// #define arch_vmap_pud_supported arch_vmap_pud_supported
#[inline]
pub unsafe fn arch_vmap_pud_supported(prot: pgprot_t) -> bool {
    let _ = prot;
    boot_cpu_has(X86_FEATURE_GBPAGES)
}
// #endif

// #define arch_vmap_pmd_supported arch_vmap_pmd_supported
#[inline]
pub unsafe fn arch_vmap_pmd_supported(prot: pgprot_t) -> bool {
    let _ = prot;
    boot_cpu_has(X86_FEATURE_PSE)
}

// #endif

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
