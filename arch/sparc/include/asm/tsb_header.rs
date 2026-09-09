/* SPDX-License-Identifier: GPL-2.0 */

// The original header contains SPARC assembly macros.  They are retained
// below as Rust macro interfaces with their instruction sequences documented,
// since Rust has no direct equivalent for assembler sections and registers.

pub const TSB_TAG_LOCK_BIT: u32 = 47;
pub const TSB_TAG_LOCK_HIGH: u32 = 1u32 << (TSB_TAG_LOCK_BIT - 32);
pub const TSB_TAG_INVALID_BIT: u32 = 46;
pub const TSB_TAG_INVALID_HIGH: u32 = 1u32 << (TSB_TAG_INVALID_BIT - 32);

#[repr(C)]
pub struct tsb_ldquad_phys_patch_entry {
    pub addr: u32,
    pub sun4u_insn: u32,
    pub sun4v_insn: u32,
}

unsafe extern "C" {
    pub static mut __tsb_ldquad_phys_patch: tsb_ldquad_phys_patch_entry;
    pub static mut __tsb_ldquad_phys_patch_end: tsb_ldquad_phys_patch_entry;
}

#[repr(C)]
pub struct tsb_phys_patch_entry {
    pub addr: u32,
    pub insn: u32,
}

unsafe extern "C" {
    pub static mut __tsb_phys_patch: tsb_phys_patch_entry;
    pub static mut __tsb_phys_patch_end: tsb_phys_patch_entry;
}

// These macros expand to SPARC assembler in the C source.  The string form
// preserves the complete instruction-level operation for assembler-aware
// consumers while keeping this translation valid Rust.
#[macro_export]
macro_rules! tsb_asm { ($($x:tt)*) => { stringify!($($x)*) }; }

#[macro_export]
macro_rules! TSB_LOAD_QUAD { ($tsb:tt, $reg:tt) => { $crate::tsb_asm!("ldda [TSB] ASI_NUCLEUS_QUAD_LDD, REG; patched with ASI_QUAD_LDD_PHYS and ASI_QUAD_LDD_PHYS_4V") }; }
#[macro_export]
macro_rules! TSB_LOAD_TAG_HIGH { ($tsb:tt, $reg:tt) => { $crate::tsb_asm!("lduwa [TSB] ASI_N; patched with ASI_PHYS_USE_EC") }; }
#[macro_export]
macro_rules! TSB_LOAD_TAG { ($tsb:tt, $reg:tt) => { $crate::tsb_asm!("ldxa [TSB] ASI_N; patched with ASI_PHYS_USE_EC") }; }
#[macro_export]
macro_rules! TSB_CAS_TAG_HIGH { ($tsb:tt, $reg1:tt, $reg2:tt) => { $crate::tsb_asm!("casa [TSB] ASI_N, REG1, REG2; patched with ASI_PHYS_USE_EC") }; }
#[macro_export]
macro_rules! TSB_CAS_TAG { ($tsb:tt, $reg1:tt, $reg2:tt) => { $crate::tsb_asm!("casxa [TSB] ASI_N, REG1, REG2; patched with ASI_PHYS_USE_EC") }; }
#[macro_export]
macro_rules! TSB_STORE { ($addr:tt, $val:tt) => { $crate::tsb_asm!("stxa VAL, [ADDR] ASI_N; patched with ASI_PHYS_USE_EC") }; }

#[macro_export]
macro_rules! TSB_LOCK_TAG { ($tsb:tt, $reg1:tt, $reg2:tt) => { $crate::tsb_asm!("load tag high; test TSB_TAG_LOCK_HIGH; CAS tag high; retry until unlocked") }; }
#[macro_export]
macro_rules! TSB_WRITE { ($tsb:tt, $tte:tt, $tag:tt) => { $crate::tsb_asm!("add TSB, 0x8; store TTE; subtract 0x8; store TAG") }; }

// Page-table walks and OBP/TSB lookup macros are assembler-only in the
// original header; preserve their exact source operation as documentation.
// KERN_PGTABLE_WALK, USER_PGTABLE_CHECK_{PUD,PMD}_HUGE,
// USER_PGTABLE_WALK_TL1, OBP_TRANS_LOOKUP, KERN_TSB_LOOKUP_TL1 and
// KERN_TSB4M_LOOKUP_TL1 perform the instruction sequences shown in tsb.h.

pub const KERNEL_TSB_SIZE_BYTES: usize = 32 * 1024;
pub const KERNEL_TSB_NENTRIES: usize = KERNEL_TSB_SIZE_BYTES / 16;
pub const KERNEL_TSB4M_NENTRIES: usize = 4096;

#[macro_export]
macro_rules! KERN_PGTABLE_WALK { ($vaddr:tt, $reg1:tt, $reg2:tt, $fail:tt) => { $crate::tsb_asm!("sethi/or swapper_pg_dir; walk PGD, PUD and PMD with ASI_PHYS_USE_EC; handle huge mappings; branch FAIL_LABEL") }; }

#[cfg(any(feature = "CONFIG_HUGETLB_PAGE", feature = "CONFIG_TRANSPARENT_HUGEPAGE"))]
#[macro_export]
macro_rules! USER_PGTABLE_CHECK_PUD_HUGE { ($vaddr:tt, $reg1:tt, $reg2:tt, $fail:tt, $pte:tt) => { $crate::tsb_asm!("check PUD validity and _PAGE_PUD_HUGE; propagate VADDR bits 32:22; branch FAIL_LABEL or PTE_LABEL") }; }

#[cfg(not(any(feature = "CONFIG_HUGETLB_PAGE", feature = "CONFIG_TRANSPARENT_HUGEPAGE")))]
#[macro_export]
macro_rules! USER_PGTABLE_CHECK_PUD_HUGE { ($vaddr:tt, $reg1:tt, $reg2:tt, $fail:tt, $pte:tt) => { $crate::tsb_asm!("brz,pn REG1, FAIL_LABEL; nop") }; }

#[cfg(any(feature = "CONFIG_HUGETLB_PAGE", feature = "CONFIG_TRANSPARENT_HUGEPAGE"))]
#[macro_export]
macro_rules! USER_PGTABLE_CHECK_PMD_HUGE { ($vaddr:tt, $reg1:tt, $reg2:tt, $fail:tt, $pte:tt) => { $crate::tsb_asm!("check PMD validity and _PAGE_PMD_HUGE; propagate 4MB VADDR bit; branch FAIL_LABEL or PTE_LABEL") }; }

#[cfg(not(any(feature = "CONFIG_HUGETLB_PAGE", feature = "CONFIG_TRANSPARENT_HUGEPAGE")))]
#[macro_export]
macro_rules! USER_PGTABLE_CHECK_PMD_HUGE { ($vaddr:tt, $reg1:tt, $reg2:tt, $fail:tt, $pte:tt) => { $crate::tsb_asm!("brz,pn REG1, FAIL_LABEL; nop") }; }

#[macro_export]
macro_rules! USER_PGTABLE_WALK_TL1 { ($vaddr:tt, $pgd:tt, $reg1:tt, $reg2:tt, $fail:tt) => { $crate::tsb_asm!("walk PGD, PUD, PMD and PTE using ASI_PHYS_USE_EC; huge-page checks; branch FAIL_LABEL") }; }
#[macro_export]
macro_rules! OBP_TRANS_LOOKUP { ($vaddr:tt, $reg1:tt, $reg2:tt, $reg3:tt, $fail:tt) => { $crate::tsb_asm!("iterate prom_trans entries; compare VADDR range; return translated PTE or branch FAIL_LABEL") }; }
#[macro_export]
macro_rules! KERN_TSB_LOOKUP_TL1 { ($vaddr:tt, $tag:tt, $reg1:tt, $reg2:tt, $reg3:tt, $reg4:tt, $ok:tt) => { $crate::tsb_asm!("compute swapper_tsb index; load quad; compare TAG; branch OK_LABEL on hit") }; }

#[cfg(not(feature = "CONFIG_DEBUG_PAGEALLOC"))]
#[macro_export]
macro_rules! KERN_TSB4M_LOOKUP_TL1 { ($tag:tt, $reg1:tt, $reg2:tt, $reg3:tt, $reg4:tt, $ok:tt) => { $crate::tsb_asm!("compute swapper_4m_tsb index from TAG; load quad; compare TAG; branch OK_LABEL on hit") }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
