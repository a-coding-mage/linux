/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The rules for enabling IBT are:
 *
 *  - CC_HAS_IBT:         the toolchain supports it
 *  - X86_KERNEL_IBT:     it is selected in Kconfig
 *  - !__DISABLE_EXPORTS: this is regular kernel code
 *
 * When all the above are satisfied, HAS_KERNEL_IBT is 1, otherwise 0.
 * The C preprocessor conditions are represented here by Cargo cfg features.
 */

#[cfg(all(feature = "CONFIG_X86_KERNEL_IBT", not(feature = "__DISABLE_EXPORTS")))]
pub const HAS_KERNEL_IBT: u32 = 1;
#[cfg(not(all(feature = "CONFIG_X86_KERNEL_IBT", not(feature = "__DISABLE_EXPORTS"))))]
pub const HAS_KERNEL_IBT: u32 = 0;

#[cfg(all(feature = "CONFIG_X86_KERNEL_IBT", not(feature = "__DISABLE_EXPORTS"), feature = "CONFIG_X86_64"))]
pub const ASM_ENDBR: &str = "endbr64\n\t";
#[cfg(all(feature = "CONFIG_X86_KERNEL_IBT", not(feature = "__DISABLE_EXPORTS"), not(feature = "CONFIG_X86_64")))]
pub const ASM_ENDBR: &str = "endbr32\n\t";
#[cfg(not(all(feature = "CONFIG_X86_KERNEL_IBT", not(feature = "__DISABLE_EXPORTS"))))]
pub const ASM_ENDBR: &str = "";

#[cfg(all(feature = "CONFIG_X86_KERNEL_IBT", not(feature = "__DISABLE_EXPORTS")))]
#[inline(always)]
pub const unsafe fn gen_endbr() -> u32 {
    let mut endbr: u32;
    /* Generate ENDBR64 without producing an ENDBR64 immediate. */
    core::arch::asm!(
        "mov {0:e}, {1:e}",
        "not {0:e}",
        out(reg) endbr,
        const !0xfa1e0ff3u32,
        options(nostack, nomem, preserves_flags)
    );
    endbr
}

#[cfg(all(feature = "CONFIG_X86_KERNEL_IBT", not(feature = "__DISABLE_EXPORTS")))]
#[inline(always)]
pub const fn gen_endbr_poison() -> u32 {
    /* 4 byte NOP that is unique to former ENDBR sites and carries UDB. */
    0xd6401f0f /* nopl -42(%rax) */
}

#[cfg(all(feature = "CONFIG_X86_KERNEL_IBT", not(feature = "__DISABLE_EXPORTS")))]
#[inline]
pub unsafe fn __is_endbr(val: u32) -> bool {
    if val == gen_endbr_poison() {
        return true;
    }
    let val = val & !0x01000000u32; /* ENDBR32 -> ENDBR64 */
    val == gen_endbr()
}

#[cfg(all(feature = "CONFIG_X86_KERNEL_IBT", not(feature = "__DISABLE_EXPORTS")))]
extern "C" {
    pub fn is_endbr(val: *mut u32) -> bool;
    pub fn ibt_save(disable: bool) -> u64;
    pub fn ibt_restore(save: u64);
}

#[cfg(not(all(feature = "CONFIG_X86_KERNEL_IBT", not(feature = "__DISABLE_EXPORTS"))))]
#[inline]
pub unsafe fn is_endbr(_val: *mut u32) -> bool { false }

#[cfg(not(all(feature = "CONFIG_X86_KERNEL_IBT", not(feature = "__DISABLE_EXPORTS"))))]
#[inline]
pub unsafe fn ibt_save(_disable: bool) -> u64 { 0 }

#[cfg(not(all(feature = "CONFIG_X86_KERNEL_IBT", not(feature = "__DISABLE_EXPORTS"))))]
#[inline]
pub unsafe fn ibt_restore(_save: u64) {}

pub const ENDBR_INSN_SIZE: u32 = 4 * HAS_KERNEL_IBT;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
