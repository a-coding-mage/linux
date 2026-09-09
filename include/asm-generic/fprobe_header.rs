/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Generic arch dependent fprobe macros.
 */

/* The following items are present only when CONFIG_64BIT is enabled. */
#[cfg(CONFIG_64BIT)]
pub const ARCH_DEFINE_ENCODE_FPROBE_HEADER: bool = true;

#[cfg(CONFIG_64BIT)]
pub const FPROBE_HEADER_MSB_SIZE_SHIFT: usize =
    BITS_PER_LONG - FPROBE_DATA_SIZE_BITS;

#[cfg(CONFIG_64BIT)]
pub const FPROBE_HEADER_MSB_MASK: usize =
    ((1usize << FPROBE_HEADER_MSB_SIZE_SHIFT) - 1);

/*
 * By default, this expects the MSBs in the address of kprobe is 0xf.
 * If any arch needs another fixed pattern (e.g. s390 is zero filled),
 * override this.
 */
#[cfg(CONFIG_64BIT)]
pub const FPROBE_HEADER_MSB_PATTERN: usize =
    (!0usize) << FPROBE_HEADER_MSB_SIZE_SHIFT;

#[cfg(CONFIG_64BIT)]
#[inline]
pub unsafe fn arch_fprobe_header_encodable(fp: *const fprobe) -> bool {
    ((fp as usize) & !FPROBE_HEADER_MSB_MASK) == FPROBE_HEADER_MSB_PATTERN
}

#[cfg(CONFIG_64BIT)]
#[inline]
pub unsafe fn arch_encode_fprobe_header(fp: *const fprobe, size: usize) -> usize {
    ((fp as usize) & FPROBE_HEADER_MSB_MASK)
        | (size << FPROBE_HEADER_MSB_SIZE_SHIFT)
}

#[cfg(CONFIG_64BIT)]
#[inline]
pub fn arch_decode_fprobe_header_size(val: usize) -> usize {
    val >> FPROBE_HEADER_MSB_SIZE_SHIFT
}

#[cfg(CONFIG_64BIT)]
#[inline]
pub unsafe fn arch_decode_fprobe_header_fp(val: usize) -> *mut fprobe {
    ((val & FPROBE_HEADER_MSB_MASK) | FPROBE_HEADER_MSB_PATTERN) as *mut fprobe
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
