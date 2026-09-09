/* SPDX-License-Identifier: GPL-2.0-only */

// C header guard: _ASM_RISCV_VMALLOC_H

// C build-time condition: CONFIG_HAVE_ARCH_HUGE_VMAP
#[cfg(CONFIG_HAVE_ARCH_HUGE_VMAP)]
extern "C" {
    pub static mut pgtable_l4_enabled: bool;
    pub static mut pgtable_l5_enabled: bool;
}

#[cfg(CONFIG_HAVE_ARCH_HUGE_VMAP)]
pub const IOREMAP_MAX_ORDER: usize = PUD_SHIFT;

// C self-referential function-like macro: arch_vmap_pud_supported
#[cfg(CONFIG_HAVE_ARCH_HUGE_VMAP)]
#[inline]
pub unsafe fn arch_vmap_pud_supported(prot: pgprot_t) -> bool {
    let _ = prot;
    pgtable_l4_enabled || pgtable_l5_enabled
}

// C self-referential function-like macro: arch_vmap_pmd_supported
#[cfg(CONFIG_HAVE_ARCH_HUGE_VMAP)]
#[inline]
pub unsafe fn arch_vmap_pmd_supported(prot: pgprot_t) -> bool {
    let _ = prot;
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
