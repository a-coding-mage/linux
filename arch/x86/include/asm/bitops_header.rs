/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of x86/include/asm/bitops.h. */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external (linux/compiler.h, asm/alternative.h, asm/rmwcc.h, asm/barrier.h).

#[cfg(target_pointer_width = "32")]
pub const _BITOPS_LONG_SHIFT: u32 = 5;
#[cfg(target_pointer_width = "64")]
pub const _BITOPS_LONG_SHIFT: u32 = 6;

pub const fn bit_64(n: u32) -> u64 { 1u64 << n }

#[inline(always)]
pub unsafe fn arch_set_bit(nr: isize, addr: *mut usize) {
    let p = (addr as *mut u8).offset(nr >> 3);
    let mask = 1u8 << (nr & 7);
    core::ptr::write_volatile(p, core::ptr::read_volatile(p) | mask);
}

#[inline(always)]
pub unsafe fn arch___set_bit(nr: usize, addr: *mut usize) {
    let p = (addr as *mut u8).add(nr >> 3);
    let mask = 1u8 << (nr & 7);
    core::ptr::write_volatile(p, core::ptr::read_volatile(p) | mask);
}

#[inline(always)]
pub unsafe fn arch_clear_bit(nr: isize, addr: *mut usize) {
    let p = (addr as *mut u8).offset(nr >> 3);
    let mask = !(1u8 << (nr & 7));
    core::ptr::write_volatile(p, core::ptr::read_volatile(p) & mask);
}

#[inline(always)]
pub unsafe fn arch_clear_bit_unlock(nr: isize, addr: *mut usize) { core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst); arch_clear_bit(nr, addr); }

#[inline(always)]
pub unsafe fn arch___clear_bit(nr: usize, addr: *mut usize) { let p=(addr as *mut u8).add(nr>>3); let m=!(1u8<<(nr&7)); core::ptr::write_volatile(p, core::ptr::read_volatile(p)&m); }

#[inline(always)]
pub unsafe fn arch_xor_unlock_is_negative_byte(mask: usize, addr: *mut usize) -> bool { let p=addr as *mut u8; let v=core::ptr::read_volatile(p) ^ mask as u8; core::ptr::write_volatile(p,v); (v as i8) < 0 }

#[inline(always)]
pub unsafe fn arch___clear_bit_unlock(nr: isize, addr: *mut usize) { arch___clear_bit(nr as usize, addr); }

#[inline(always)]
pub unsafe fn arch___change_bit(nr: usize, addr: *mut usize) { let p=(addr as *mut u8).add(nr>>3); let m=1u8<<(nr&7); core::ptr::write_volatile(p,core::ptr::read_volatile(p)^m); }

#[inline(always)]
pub unsafe fn arch_change_bit(nr: isize, addr: *mut usize) { arch___change_bit(nr as usize, addr); }

#[inline(always)]
pub unsafe fn arch_test_and_set_bit(nr: isize, addr: *mut usize) -> bool { let p=(addr as *mut u8).offset(nr>>3); let m=1u8<<(nr&7); let v=core::ptr::read_volatile(p); core::ptr::write_volatile(p,v|m); v&m != 0 }
#[inline(always)]
pub unsafe fn arch_test_and_set_bit_lock(nr: isize, addr: *mut usize) -> bool { arch_test_and_set_bit(nr,addr) }
#[inline(always)]
pub unsafe fn arch___test_and_set_bit(nr: usize, addr: *mut usize) -> bool { arch_test_and_set_bit(nr as isize,addr) }
#[inline(always)]
pub unsafe fn arch_test_and_clear_bit(nr: isize, addr: *mut usize) -> bool { let p=(addr as *mut u8).offset(nr>>3); let m=1u8<<(nr&7); let v=core::ptr::read_volatile(p); core::ptr::write_volatile(p,v&!m); v&m != 0 }
#[inline(always)]
pub unsafe fn arch___test_and_clear_bit(nr: usize, addr: *mut usize) -> bool { arch_test_and_clear_bit(nr as isize,addr) }
#[inline(always)]
pub unsafe fn arch___test_and_change_bit(nr: usize, addr: *mut usize) -> bool { let p=(addr as *mut u8).add(nr>>3); let m=1u8<<(nr&7); let v=core::ptr::read_volatile(p); core::ptr::write_volatile(p,v^m); v&m != 0 }
#[inline(always)]
pub unsafe fn arch_test_and_change_bit(nr: isize, addr: *mut usize) -> bool { arch___test_and_change_bit(nr as usize,addr) }

#[inline(always)]
pub unsafe fn constant_test_bit(nr: isize, addr: *const usize) -> bool { (*addr.add((nr as usize)>>_BITOPS_LONG_SHIFT)) & (1usize << ((nr as usize)&(usize::BITS as usize-1))) != 0 }
#[inline(always)]
pub unsafe fn constant_test_bit_acquire(nr: isize, addr: *const usize) -> bool { constant_test_bit(nr,addr) }
#[inline(always)]
pub unsafe fn variable_test_bit(nr: isize, addr: *const usize) -> bool { constant_test_bit(nr,addr) }
#[inline(always)]
pub unsafe fn arch_test_bit(nr: usize, addr: *const usize) -> bool { variable_test_bit(nr as isize,addr) }
#[inline(always)]
pub unsafe fn arch_test_bit_acquire(nr: usize, addr: *const usize) -> bool { variable_test_bit(nr as isize,addr) }

#[inline(always)]
pub const fn variable__ffs(word: usize) -> usize { word.trailing_zeros() as usize }
#[inline(always)]
pub const fn __ffs(word: usize) -> usize { variable__ffs(word) }
#[inline(always)]
pub const fn variable_ffz(word: usize) -> usize { variable__ffs(!word) }
#[inline(always)]
pub const fn ffz(word: usize) -> usize { variable_ffz(word) }
#[inline(always)]
pub const fn __fls(word: usize) -> usize { (usize::BITS - 1 - word.leading_zeros()) as usize }

#[inline(always)]
pub const fn variable_ffs(x: i32) -> i32 { if x == 0 { 0 } else { x.trailing_zeros() as i32 + 1 } }
#[inline(always)]
pub const fn ffs(x: i32) -> i32 { variable_ffs(x) }
#[inline(always)]
pub const fn fls(x: u32) -> i32 { if x == 0 { 0 } else { 32 - x.leading_zeros() as i32 } }
#[inline(always)]
pub const fn fls64(x: u64) -> i32 { if x == 0 { 0 } else { 64 - x.leading_zeros() as i32 } }

// Additional generic bitops declarations included by the original header:
// asm-generic/bitops/{fls64,sched,const_hweight,instrumented-atomic,
// instrumented-non-atomic,instrumented-lock,le,ext2-atomic-setbit}.h and
// asm/arch_hweight.h.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
