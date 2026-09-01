/* SPDX-License-Identifier: GPL-2.0 */
/*
 * FIXME: This came from tools/perf/perf-sys.h, where it was first introduced
 * in c1e028ef40b8d6943b767028ba17d4f2ba020edb, more work needed to make it
 * more closely follow the Linux kernel arch/mips/include/asm/barrier.h file.
 * Probably when we continue work on tools/ Kconfig support to have all the
 * CONFIG_ needed for properly doing that.
 */

#[inline(always)]
pub unsafe fn mb() {
    unsafe {
        core::arch::asm!(
            ".set	mips2",
            "sync",
            ".set	mips0",
            options(nostack, preserves_flags)
        );
    }
}

#[inline(always)]
pub unsafe fn wmb() {
    unsafe {
        mb();
    }
}

#[inline(always)]
pub unsafe fn rmb() {
    unsafe {
        mb();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
