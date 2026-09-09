// SPDX-License-Identifier: GPL-2.0
/*
 * Generate definitions needed by the preprocessor.
 * This code generates raw asm output which is post-processed
 * to extract and format the required data.
 */

// __GENERATING_BOUNDS_H
// COMPILE_OFFSETS
// Dependency headers: linux/page-flags.h, linux/mmzone.h, linux/kbuild.h,
// linux/log2.h, linux/spinlock_types.h

unsafe extern "C" {
    fn DEFINE(name: *const core::ffi::c_char, value: usize);
    fn order_base_2(value: usize) -> usize;
}

pub unsafe fn main() -> core::ffi::c_int {
    /* The enum constants to put into include/generated/bounds.h */
    DEFINE(b"NR_PAGEFLAGS\0".as_ptr().cast(), __NR_PAGEFLAGS as usize);
    DEFINE(b"MAX_NR_ZONES\0".as_ptr().cast(), __MAX_NR_ZONES as usize);

    #[cfg(CONFIG_SMP)]
    DEFINE(
        b"NR_CPUS_BITS\0".as_ptr().cast(),
        order_base_2(CONFIG_NR_CPUS as usize),
    );

    DEFINE(
        b"SPINLOCK_SIZE\0".as_ptr().cast(),
        core::mem::size_of::<spinlock_t>(),
    );

    #[cfg(CONFIG_LRU_GEN)]
    {
        DEFINE(
            b"LRU_GEN_WIDTH\0".as_ptr().cast(),
            order_base_2((MAX_NR_GENS + 1) as usize),
        );
        DEFINE(
            b"__LRU_REFS_WIDTH\0".as_ptr().cast(),
            (MAX_NR_TIERS - 2) as usize,
        );
    }

    #[cfg(not(CONFIG_LRU_GEN))]
    {
        DEFINE(b"LRU_GEN_WIDTH\0".as_ptr().cast(), 0);
        DEFINE(b"__LRU_REFS_WIDTH\0".as_ptr().cast(), 0);
    }
    /* End of constants */

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
