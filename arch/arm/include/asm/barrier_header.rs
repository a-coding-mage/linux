/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from the non-assembly portion of asm/barrier.h. */

#[inline(always)]
pub unsafe fn nop() {
    core::arch::asm!("mov r0, r0", options(nostack, preserves_flags));
}

/* __LINUX_ARM_ARCH__ >= 7, or ARMv6K. */
#[inline(always)]
pub unsafe fn sev() {
    core::arch::asm!("sev", options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn wfe() {
    core::arch::asm!("wfe", options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn wfi() {
    core::arch::asm!("wfi", options(nostack, preserves_flags));
}

#[inline(always)]
pub fn wfe_noop() {}

#[inline(always)]
pub unsafe fn isb(_option: impl Copy) {
    core::arch::asm!("isb", options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn dsb(_option: impl Copy) {
    core::arch::asm!("dsb", options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn dmb(_option: impl Copy) {
    core::arch::asm!("dmb", options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn csdb() {
    /* CSDB is .inst.w 0xf3af8014 for Thumb-2, otherwise .inst 0xe320f014. */
    core::arch::asm!(".inst 0xe320f014", options(nostack, preserves_flags));
}

#[cfg(feature = "arm-heavy-mb")]
unsafe extern "C" {
    pub static mut soc_mb: Option<unsafe extern "C" fn()>;
    pub fn arm_heavy_mb();
}

#[cfg(feature = "arm-heavy-mb")]
#[inline(always)]
pub unsafe fn __arm_heavy_mb<T: Copy>(option: T) {
    dsb(option);
    arm_heavy_mb();
}

#[cfg(not(feature = "arm-heavy-mb"))]
#[inline(always)]
pub unsafe fn __arm_heavy_mb<T: Copy>(option: T) {
    dsb(option);
}

/* CONFIG_ARM_DMA_MEM_BUFFERABLE || CONFIG_SMP selects the heavy barriers. */
#[inline(always)]
pub unsafe fn mb() { __arm_heavy_mb(()); }

#[inline(always)]
pub unsafe fn rmb() { dsb(()); }

#[inline(always)]
pub unsafe fn wmb() { __arm_heavy_mb(()); }

#[inline(always)]
pub unsafe fn dma_rmb() { dmb(()); }

#[inline(always)]
pub unsafe fn dma_wmb() { dmb(()); }

#[inline(always)]
pub unsafe fn __smp_mb() { dmb(()); }

#[inline(always)]
pub unsafe fn __smp_rmb() { __smp_mb(); }

#[inline(always)]
pub unsafe fn __smp_wmb() { dmb(()); }

#[cfg(feature = "cpu-spectre")]
#[inline(always)]
pub unsafe fn array_index_mask_nospec(idx: usize, sz: usize) -> usize {
    let mask: usize;
    core::arch::asm!(
        "cmp {idx}, {sz}",
        "sbc {mask}, {idx}, {idx}",
        ".inst 0xe320f014",
        idx = in(reg) idx,
        sz = in(reg) sz,
        mask = lateout(reg) mask,
        options(nostack)
    );
    mask
}

/* Dependency: <asm-generic/barrier.h> supplies the generic barrier layer. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
