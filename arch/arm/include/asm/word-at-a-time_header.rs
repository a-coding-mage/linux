/* SPDX-License-Identifier: GPL-2.0 */

/* Little-endian word-at-a-time zero byte handling. */

#[cfg(target_endian = "little")]
pub struct word_at_a_time {
    pub one_bits: usize,
    pub high_bits: usize,
}

#[cfg(target_endian = "little")]
pub const WORD_AT_A_TIME_CONSTANTS: word_at_a_time = word_at_a_time {
    one_bits: usize::MAX / 0xff * 0x01,
    high_bits: usize::MAX / 0xff * 0x80,
};

#[cfg(target_endian = "little")]
#[inline]
pub unsafe fn has_zero(a: usize, bits: *mut usize, c: *const word_at_a_time) -> usize {
    let mask = a.wrapping_sub((*c).one_bits) & !a & (*c).high_bits;
    *bits = mask;
    mask
}

#[cfg(target_endian = "little")]
#[inline]
pub fn prep_zero_mask(_a: usize, bits: usize, _c: &word_at_a_time) -> usize {
    bits
}

#[cfg(target_endian = "little")]
#[inline]
pub fn create_zero_mask(mut bits: usize) -> usize {
    bits = bits.wrapping_sub(1) & !bits;
    bits >> 7
}

#[cfg(target_endian = "little")]
#[inline]
pub fn find_zero(mask: usize) -> usize {
    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    {
        /* Linux ARM architectures with __LINUX_ARM_ARCH__ >= 5 have clz. */
        if mask != 0 {
            return (usize::BITS as usize - mask.leading_zeros() as usize) >> 3;
        }
        return 0;
    }

    #[cfg(not(any(target_arch = "arm", target_arch = "aarch64")))]
    {
        /* (000000 0000ff 00ffff ffffff) -> ( 1 1 2 3 ) */
        let mut ret = (0x0ff0001usize.wrapping_add(mask)) >> 23;
        /* Fix the 1 for 00 case */
        ret &= mask;
        ret
    }
}

#[cfg(target_endian = "little")]
#[inline]
pub fn zero_bytemask(mask: usize) -> usize {
    mask
}

#[cfg(target_endian = "big")]
/* The big-endian implementation is supplied by asm-generic/word-at-a-time.h. */

#[cfg(all(feature = "CONFIG_DCACHE_WORD_ACCESS", target_arch = "arm"))]
#[inline]
pub unsafe fn load_unaligned_zeropad(addr: *const core::ffi::c_void) -> usize {
    let mut ret: usize;
    let mut tmp: usize;
    core::arch::asm!(
        "1: ldr {ret}, [{addr}]",
        "2:",
        ".pushsection .text.fixup,\"ax\"",
        ".align 2",
        "3: bic {tmp}, {addr}, #0x3",
        "ldr {ret}, [{tmp}]",
        "and {tmp}, {addr}, #0x3",
        "lsl {tmp}, {tmp}, #0x3",
        "lsr {ret}, {ret}, {tmp}",
        "b 2b",
        ".popsection",
        ".pushsection __ex_table,\"a\"",
        ".align 3",
        ".long 1b, 3b",
        ".popsection",
        ret = out(reg) ret,
        tmp = out(reg) tmp,
        addr = in(reg) addr,
        options(nostack)
    );
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
