/* SPDX-License-Identifier: GPL-2.0-only */
/* Bit operations for the Hexagon architecture. */

// C dependencies: linux/compiler.h, asm/byteorder.h, asm/atomic.h,
// asm/barrier.h, and the asm-generic bitops headers.

#[inline]
pub unsafe fn test_and_clear_bit(nr: i32, addr: *mut core::ffi::c_void) -> i32 {
    let oldval: i32;
    core::arch::asm!(
        "{{R10 = {addr}; R11 = asr({nr},#5); }}\n",
        "{{R10 += asl(R11,#2); R11 = and({nr},#0x1f)}}\n",
        "1: R12 = memw_locked(R10);\n",
        "{{ P0 = tstbit(R12,R11); R12 = clrbit(R12,R11); }}\n",
        "memw_locked(R10,P1) = R12;\n",
        "{{if (!P1) jump 1b; {out} = mux(P0,#1,#0);}}\n",
        addr = in(reg) addr, nr = in(reg) nr, out = lateout(reg) oldval,
        options(nostack, preserves_flags)
    );
    oldval
}

#[inline]
pub unsafe fn test_and_set_bit(nr: i32, addr: *mut core::ffi::c_void) -> i32 {
    let oldval: i32;
    core::arch::asm!(
        "{{R10 = {addr}; R11 = asr({nr},#5); }}\n{{R10 += asl(R11,#2); R11 = and({nr},#0x1f)}}\n1: R12 = memw_locked(R10);\n{{ P0 = tstbit(R12,R11); R12 = setbit(R12,R11); }}\nmemw_locked(R10,P1) = R12;\n{{if (!P1) jump 1b; {out} = mux(P0,#1,#0);}}",
        addr = in(reg) addr, nr = in(reg) nr, out = lateout(reg) oldval,
        options(nostack, preserves_flags)
    );
    oldval
}

#[inline]
pub unsafe fn test_and_change_bit(nr: i32, addr: *mut core::ffi::c_void) -> i32 {
    let oldval: i32;
    core::arch::asm!(
        "{{R10 = {addr}; R11 = asr({nr},#5); }}\n{{R10 += asl(R11,#2); R11 = and({nr},#0x1f)}}\n1: R12 = memw_locked(R10);\n{{ P0 = tstbit(R12,R11); R12 = togglebit(R12,R11); }}\nmemw_locked(R10,P1) = R12;\n{{if (!P1) jump 1b; {out} = mux(P0,#1,#0);}}",
        addr = in(reg) addr, nr = in(reg) nr, out = lateout(reg) oldval,
        options(nostack, preserves_flags)
    );
    oldval
}

#[inline] pub unsafe fn clear_bit(nr: i32, addr: *mut core::ffi::c_void) { test_and_clear_bit(nr, addr); }
#[inline] pub unsafe fn set_bit(nr: i32, addr: *mut core::ffi::c_void) { test_and_set_bit(nr, addr); }
#[inline] pub unsafe fn change_bit(nr: i32, addr: *mut core::ffi::c_void) { test_and_change_bit(nr, addr); }

#[inline] pub unsafe fn arch___clear_bit(nr: usize, addr: *mut usize) { test_and_clear_bit(nr as i32, addr.cast()); }
#[inline] pub unsafe fn arch___set_bit(nr: usize, addr: *mut usize) { test_and_set_bit(nr as i32, addr.cast()); }
#[inline] pub unsafe fn arch___change_bit(nr: usize, addr: *mut usize) { test_and_change_bit(nr as i32, addr.cast()); }
#[inline] pub unsafe fn arch___test_and_clear_bit(nr: usize, addr: *mut usize) -> bool { test_and_clear_bit(nr as i32, addr.cast()) != 0 }
#[inline] pub unsafe fn arch___test_and_set_bit(nr: usize, addr: *mut usize) -> bool { test_and_set_bit(nr as i32, addr.cast()) != 0 }
#[inline] pub unsafe fn arch___test_and_change_bit(nr: usize, addr: *mut usize) -> bool { test_and_change_bit(nr as i32, addr.cast()) != 0 }

#[inline]
pub unsafe fn arch_test_bit(nr: usize, addr: *const usize) -> bool {
    let word = *addr.add(nr / BITS_PER_LONG);
    ((word >> (nr % BITS_PER_LONG)) & 1) != 0
}
#[inline]
pub unsafe fn arch_test_bit_acquire(nr: usize, addr: *const usize) -> bool {
    core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
    arch_test_bit(nr, addr)
}

// bits_per_long is assumed to be 32, as in the original Hexagon header.
pub const BITS_PER_LONG: usize = 32;

#[inline] pub const fn ffz(x: i32) -> i64 { (!x as u32).trailing_zeros() as i64 }
#[inline] pub const fn fls(x: u32) -> i32 { if x == 0 { 0 } else { 32 - x.leading_zeros() as i32 } }
#[inline] pub const fn ffs(x: i32) -> i32 { if x == 0 { 0 } else { x.trailing_zeros() as i32 + 1 } }
#[inline] pub const fn __ffs(word: usize) -> usize { word.trailing_zeros() as usize }
#[inline] pub const fn __fls(word: usize) -> usize { (31 - word.leading_zeros()) as usize }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
