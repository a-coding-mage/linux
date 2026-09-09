/* SPDX-License-Identifier: GPL-2.0 */

/*
 * "o,w" is sufficient to ensure that all writes to the device have completed
 * before the write to the spinlock is allowed to commit.
 *
 * The C header includes linux/smp.h and asm-generic/mmiowb.h; their symbols
 * are supplied by other translated dependencies.
 */
macro_rules! mmiowb {
    () => {
        RISCV_FENCE!(o, w)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
