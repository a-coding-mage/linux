/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2012 Regents of the University of California */

/* Translated from the Linux RISC-V bitops header. */

#[cfg(any(not(all(CONFIG_RISCV_ISA_ZBB, CONFIG_TOOLCHAIN_HAS_ZBB)), NO_ALTERNATIVE))]
/* Generic __ffs/__fls/ffs/fls declarations are supplied by asm-generic bitops. */
;

#[cfg(all(CONFIG_RISCV_ISA_ZBB, CONFIG_TOOLCHAIN_HAS_ZBB, not(NO_ALTERNATIVE)))]
pub const __HAVE_ARCH___FFS: bool = true;
#[cfg(all(CONFIG_RISCV_ISA_ZBB, CONFIG_TOOLCHAIN_HAS_ZBB, not(NO_ALTERNATIVE)))]
pub const __HAVE_ARCH___FLS: bool = true;
#[cfg(all(CONFIG_RISCV_ISA_ZBB, CONFIG_TOOLCHAIN_HAS_ZBB, not(NO_ALTERNATIVE)))]
pub const __HAVE_ARCH_FFS: bool = true;
#[cfg(all(CONFIG_RISCV_ISA_ZBB, CONFIG_TOOLCHAIN_HAS_ZBB, not(NO_ALTERNATIVE)))]
pub const __HAVE_ARCH_FLS: bool = true;

#[cfg(all(CONFIG_RISCV_ISA_ZBB, CONFIG_TOOLCHAIN_HAS_ZBB, not(NO_ALTERNATIVE)))]
#[inline(always)]
pub unsafe fn variable__ffs(mut word: usize) -> usize {
    if !riscv_has_extension_likely(RISCV_ISA_EXT_ZBB) { return generic___ffs(word); }
    core::arch::asm!(".option push", ".option arch,+zbb", "ctz {0}, {0}", ".option pop", inout(reg) word);
    word
}

#[cfg(all(CONFIG_RISCV_ISA_ZBB, CONFIG_TOOLCHAIN_HAS_ZBB, not(NO_ALTERNATIVE)))]
#[inline(always)]
pub unsafe fn variable__fls(mut word: usize) -> usize {
    if !riscv_has_extension_likely(RISCV_ISA_EXT_ZBB) { return generic___fls(word); }
    core::arch::asm!(".option push", ".option arch,+zbb", "clz {0}, {0}", ".option pop", inout(reg) word);
    BITS_PER_LONG - 1 - word
}

#[cfg(all(CONFIG_RISCV_ISA_ZBB, CONFIG_TOOLCHAIN_HAS_ZBB, not(NO_ALTERNATIVE)))]
#[inline(always)]
pub unsafe fn variable_ffs(mut x: i32) -> i32 {
    if !riscv_has_extension_likely(RISCV_ISA_EXT_ZBB) { return generic_ffs(x); }
    if x == 0 { return 0; }
    core::arch::asm!(".option push", ".option arch,+zbb", "ctz {0}, {0}", ".option pop", inout(reg) x);
    x + 1
}

#[cfg(all(CONFIG_RISCV_ISA_ZBB, CONFIG_TOOLCHAIN_HAS_ZBB, not(NO_ALTERNATIVE)))]
#[inline(always)]
pub unsafe fn variable_fls(mut x: u32) -> i32 {
    if !riscv_has_extension_likely(RISCV_ISA_EXT_ZBB) { return generic_fls(x); }
    if x == 0 { return 0; }
    core::arch::asm!(".option push", ".option arch,+zbb", "clzw {0}, {0}", ".option pop", inout(reg) x);
    32 - x as i32
}

#[inline(always)]
pub unsafe fn arch_test_and_set_bit(nr: i32, addr: *mut usize) -> i32 {
    test_and_op_bit("or", nr, addr, usize::MAX)
}
#[inline(always)]
pub unsafe fn arch_test_and_clear_bit(nr: i32, addr: *mut usize) -> i32 {
    test_and_op_bit("and", nr, addr, !0usize)
}
#[inline(always)]
pub unsafe fn arch_test_and_change_bit(nr: i32, addr: *mut usize) -> i32 {
    test_and_op_bit("xor", nr, addr, usize::MAX)
}

#[inline(always)]
unsafe fn test_and_op_bit(_op: &str, nr: i32, addr: *mut usize, _modifier: usize) -> i32 {
    let word = addr.offset((nr as isize) / BITS_PER_LONG as isize);
    let mask = 1usize << ((nr as usize) % BITS_PER_LONG);
    let old = core::ptr::read_volatile(word);
    let new = if _op == "or" { old | mask } else if _op == "and" { old & !mask } else { old ^ mask };
    core::ptr::write_volatile(word, new);
    (old & mask != 0) as i32
}

#[inline(always)]
pub unsafe fn arch_set_bit(nr: i32, addr: *mut usize) { let _ = test_and_op_bit("or", nr, addr, usize::MAX); }
#[inline(always)]
pub unsafe fn arch_clear_bit(nr: i32, addr: *mut usize) { let _ = test_and_op_bit("and", nr, addr, !0usize); }
#[inline(always)]
pub unsafe fn arch_change_bit(nr: i32, addr: *mut usize) { let _ = test_and_op_bit("xor", nr, addr, usize::MAX); }

#[inline(always)]
pub unsafe fn arch_test_and_set_bit_lock(nr: usize, addr: *mut usize) -> i32 { test_and_op_bit("or", nr as i32, addr, usize::MAX) }
#[inline(always)]
pub unsafe fn arch_clear_bit_unlock(nr: usize, addr: *mut usize) { let _ = test_and_op_bit("and", nr as i32, addr, !0usize); }
#[inline(always)]
pub unsafe fn arch___clear_bit_unlock(nr: usize, addr: *mut usize) { arch_clear_bit_unlock(nr, addr); }

#[inline(always)]
pub unsafe fn arch_xor_unlock_is_negative_byte(mask: usize, addr: *mut usize) -> bool {
    let old = core::ptr::read_volatile(addr);
    core::ptr::write_volatile(addr, old ^ mask);
    (old & (1usize << 7)) != 0
}

/* External dependencies supplied by the surrounding kernel translation. */
extern "C" {
    fn riscv_has_extension_likely(ext: usize) -> bool;
    fn generic___ffs(word: usize) -> usize;
    fn generic___fls(word: usize) -> usize;
    fn generic_ffs(x: i32) -> i32;
    fn generic_fls(x: u32) -> i32;
}

/* BITS_PER_LONG, RISCV_ISA_EXT_ZBB, and related bitops are external dependencies. */
extern "Rust" {
    static BITS_PER_LONG: usize;
    static RISCV_ISA_EXT_ZBB: usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
