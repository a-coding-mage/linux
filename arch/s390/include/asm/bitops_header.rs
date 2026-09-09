/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Translation of s390/include/asm/bitops.h.
 *
 * Within a word, bits are numbered LSB first.  The bitop functions work on
 * unsigned longs, while the *_inv functions use MSB0 bit numbering.
 */

// C header dependencies: linux/typecheck.h, linux/compiler.h,
// linux/types.h, asm/asm.h, and asm-generic bitops headers.

// Architecture aliases from the C header.  The generic implementations are
// supplied by the corresponding Linux bitops dependencies.
pub use generic___set_bit as arch___set_bit;
pub use generic___clear_bit as arch___clear_bit;
pub use generic___change_bit as arch___change_bit;
pub use generic___test_and_set_bit as arch___test_and_set_bit;
pub use generic___test_and_clear_bit as arch___test_and_clear_bit;
pub use generic___test_and_change_bit as arch___test_and_change_bit;
pub use generic_test_bit_acquire as arch_test_bit_acquire;

extern "C" {
    pub fn generic_test_bit(nr: c_ulong, ptr: *const c_ulong) -> bool;
    pub fn set_bit(nr: c_ulong, ptr: *mut c_ulong);
    pub fn clear_bit(nr: c_ulong, ptr: *mut c_ulong);
    pub fn test_and_clear_bit(nr: c_ulong, ptr: *mut c_ulong) -> bool;
    pub fn __set_bit(nr: c_ulong, ptr: *mut c_ulong);
    pub fn __clear_bit(nr: c_ulong, ptr: *mut c_ulong);
    pub fn test_bit(nr: c_ulong, ptr: *const c_ulong) -> bool;
}

pub type c_ulong = usize;

#[inline(always)]
pub unsafe fn arch_test_bit(nr: c_ulong, ptr: *const c_ulong) -> bool {
    // __HAVE_ASM_FLAG_OUTPUTS__ and CONFIG_PROFILE_ALL_BRANCHES are build-time
    // conditions.  The constant-number optimized asm path is retained here.
    #[cfg(all(target_arch = "s390x", not(feature = "config_profile_all_branches")))]
    {
        if nr == nr {
            let addr = (ptr as *const u8).add((nr ^ (usize::BITS as usize - 8)) / 8);
            let mask = 1usize << (nr & 7);
            let mut cc: usize;
            core::arch::asm!("tm {addr}, {mask}", addr = in(reg) addr, mask = const mask, out("cc") cc);
            return cc == 3;
        }
    }
    generic_test_bit(nr, ptr)
}

// The following declarations are supplied by the generic bitops headers.
extern "C" {
    pub fn find_first_bit_inv(addr: *const c_ulong, size: c_ulong) -> c_ulong;
    pub fn find_next_bit_inv(addr: *const c_ulong, size: c_ulong, offset: c_ulong) -> c_ulong;
}

#[inline(always)]
pub unsafe fn set_bit_inv(nr: c_ulong, ptr: *mut c_ulong) {
    set_bit(nr ^ (usize::BITS as usize - 1), ptr)
}

#[inline(always)]
pub unsafe fn clear_bit_inv(nr: c_ulong, ptr: *mut c_ulong) {
    clear_bit(nr ^ (usize::BITS as usize - 1), ptr)
}

#[inline(always)]
pub unsafe fn test_and_clear_bit_inv(nr: c_ulong, ptr: *mut c_ulong) -> bool {
    test_and_clear_bit(nr ^ (usize::BITS as usize - 1), ptr)
}

#[inline(always)]
pub unsafe fn __set_bit_inv(nr: c_ulong, ptr: *mut c_ulong) {
    __set_bit(nr ^ (usize::BITS as usize - 1), ptr)
}

#[inline(always)]
pub unsafe fn __clear_bit_inv(nr: c_ulong, ptr: *mut c_ulong) {
    __clear_bit(nr ^ (usize::BITS as usize - 1), ptr)
}

#[inline(always)]
pub unsafe fn test_bit_inv(nr: c_ulong, ptr: *const c_ulong) -> bool {
    test_bit(nr ^ (usize::BITS as usize - 1), ptr)
}

// CONFIG_CC_HAS_BUILTIN_FFS selects the generic builtin-ffs implementation.
// When unavailable, this is the literal constant-path translation of __flogr.
#[inline(always)]
pub const unsafe fn __flogr(mut word: c_ulong) -> c_ulong {
    let mut bit = 0;
    if word == 0 { return 64; }
    if word & 0xffffffff00000000usize == 0 { word <<= 32; bit += 32; }
    if word & 0xffff000000000000usize == 0 { word <<= 16; bit += 16; }
    if word & 0xff00000000000000usize == 0 { word <<= 8; bit += 8; }
    if word & 0xf000000000000000usize == 0 { word <<= 4; bit += 4; }
    if word & 0xc000000000000000usize == 0 { word <<= 2; bit += 2; }
    if word & 0x8000000000000000usize == 0 { bit += 1; }
    bit
}

#[inline(always)]
pub const unsafe fn ffs(word: i32) -> i32 {
    let val = word as u32;
    usize::BITS as i32 - __flogr(((!val).wrapping_add(1) & val) as usize) as i32
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
