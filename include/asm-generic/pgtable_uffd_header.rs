/*
 * Some platforms can customize the uffd PTE bit, making it unavailable
 * even if the architecture provides the resource.
 * Adding this API allows architectures to add their own checks for the
 * devices on which the kernel is running.
 * Note: When overriding it, please make sure the
 * CONFIG_HAVE_ARCH_USERFAULTFD_WP is part of this macro.
 */
/* Build-time CONFIG_* conditions are represented with Rust feature checks. */
#[macro_export]
macro_rules! pgtable_supports_uffd {
    () => {
        cfg!(feature = "CONFIG_HAVE_ARCH_USERFAULTFD_WP")
    };
}

#[inline]
pub fn uffd_supports_wp_marker() -> bool {
    pgtable_supports_uffd!() && cfg!(feature = "CONFIG_PTE_MARKER_UFFD_WP")
}

/* CONFIG_HAVE_ARCH_USERFAULTFD_WP is absent: use the generic no-op helpers. */
#[inline(always)]
pub unsafe fn pte_uffd(pte: pte_t) -> i32 {
    let _ = pte;
    0
}

#[inline(always)]
pub unsafe fn pmd_uffd(pmd: pmd_t) -> i32 {
    let _ = pmd;
    0
}

#[inline(always)]
pub unsafe fn pte_mkuffd(pte: pte_t) -> pte_t {
    pte
}

#[inline(always)]
pub unsafe fn pmd_mkuffd(pmd: pmd_t) -> pmd_t {
    pmd
}

#[inline(always)]
pub unsafe fn pte_clear_uffd(pte: pte_t) -> pte_t {
    pte
}

#[inline(always)]
pub unsafe fn pmd_clear_uffd(pmd: pmd_t) -> pmd_t {
    pmd
}

#[inline(always)]
pub unsafe fn pte_swp_mkuffd(pte: pte_t) -> pte_t {
    pte
}

#[inline(always)]
pub unsafe fn pte_swp_uffd(pte: pte_t) -> i32 {
    let _ = pte;
    0
}

#[inline(always)]
pub unsafe fn pte_swp_clear_uffd(pte: pte_t) -> pte_t {
    pte
}

#[inline]
pub unsafe fn pmd_swp_mkuffd(pmd: pmd_t) -> pmd_t {
    pmd
}

#[inline]
pub unsafe fn pmd_swp_uffd(pmd: pmd_t) -> i32 {
    let _ = pmd;
    0
}

#[inline]
pub unsafe fn pmd_swp_clear_uffd(pmd: pmd_t) -> pmd_t {
    pmd
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
