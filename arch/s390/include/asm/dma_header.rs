/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: the C header includes <linux/io.h>, which supplies __va.

/*
 * MAX_DMA_ADDRESS is ambiguous because on s390 its completely unrelated
 * to DMA. It _is_ used for the s390 memory zone split at 2GB caused
 * by the 31 bit heritage.
 */
macro_rules! MAX_DMA_ADDRESS {
    () => {
        __va(0x8000_0000usize)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
