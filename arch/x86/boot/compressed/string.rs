// SPDX-License-Identifier: GPL-2.0
/*
 * Freestanding compressed-boot memory primitives. Inline assembly prevents
 * LLVM from lowering the implementation back into memcpy/memmove/memset and
 * keeps the pre-decompression path free of vector instructions.
 *
 * boot.rs also includes ../string.rs, matching the C textual dependency.
 */

use core::arch::asm;
use core::ffi::{c_int, c_void};

#[cfg(CONFIG_X86_32)]
unsafe fn ____memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void {
    // SAFETY: callers choose forward-safe copy ranges and the boot ABI has
    // DF=0. ESI is saved because LLVM may reserve it in 32-bit PIC code.
    unsafe {
        asm!(
            "push esi",
            "mov esi, {source:e}",
            "rep movsd",
            "mov ecx, {tail:e}",
            "rep movsb",
            "pop esi",
            source = in(reg) src,
            // ESI is changed manually, so a late-used operand cannot use it.
            tail = in(reg_abcd) n & 3,
            inout("ecx") n >> 2 => _,
            inout("edi") dest => _,
        );
    }
    dest
}

#[cfg(not(CONFIG_X86_32))]
unsafe fn ____memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void {
    // SAFETY: callers choose forward-safe copy ranges and the boot ABI has
    // DF=0. Early-clobber register operands preserve tail through REP MOVSQ.
    unsafe {
        asm!(
            "rep movsq",
            "mov rcx, {tail}",
            "rep movsb",
            tail = in(reg) n & 7,
            inout("rcx") n >> 3 => _,
            inout("rdi") dest => _,
            inout("rsi") src => _,
            options(nostack),
        );
    }
    dest
}

/// # Safety
/// The C loop's addressed bytes must be writable. The native C implementation
/// indexes with signed int, which is retained rather than widened to size_t.
#[no_mangle]
pub(crate) unsafe extern "C" fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void {
    let mut i: c_int = 0;
    // SAFETY: the caller supplies writable bytes. The opaque scalar store
    // prevents LLVM's loop-idiom pass from emitting a recursive memset call.
    unsafe {
        while (i as usize) < n {
            asm!(
                "mov byte ptr [{byte}], {value}",
                byte = in(reg) s.cast::<u8>().offset(i as isize),
                value = in(reg_byte) c as u8,
                options(nostack, preserves_flags),
            );
            i = i.wrapping_add(1);
        }
    }
    s
}

/// # Safety
/// `src` and `dest` cover `n` readable and writable bytes respectively. Their
/// ranges may overlap. The boot calling convention has DF clear.
#[no_mangle]
pub(crate) unsafe extern "C" fn memmove(
    dest: *mut c_void,
    src: *const c_void,
    mut n: usize,
) -> *mut c_void {
    // SAFETY: forward copying is selected exactly as in the C source. The
    // backwards branch uses one opaque byte load/store per original step.
    unsafe {
        let d = dest.cast::<u8>();
        let s = src.cast::<u8>();
        if (d as usize) <= (s as usize) || (d as usize).wrapping_sub(s as usize) >= n {
            return ____memcpy(dest, src, n);
        }
        while n != 0 {
            n -= 1;
            asm!(
                "mov {value}, byte ptr [{source}]",
                "mov byte ptr [{destination}], {value}",
                source = in(reg) s.add(n),
                destination = in(reg) d.add(n),
                value = out(reg_byte) _,
                options(nostack, preserves_flags),
            );
        }
    }
    dest
}

/// # Safety
/// The ranges satisfy `memmove`'s contract. Overlap with dest above src emits
/// the same warning and then uses memmove; every other case copies forwards.
#[no_mangle]
pub(crate) unsafe extern "C" fn memcpy(
    dest: *mut c_void,
    src: *const c_void,
    n: usize,
) -> *mut c_void {
    // SAFETY: preserve the original address predicate and warning order.
    unsafe {
        if (dest as usize) > (src as usize) && (dest as usize).wrapping_sub(src as usize) < n {
            crate::error::warn(c"Avoiding potentially unsafe overlapping memcpy()!".as_ptr());
            return memmove(dest, src, n);
        }
        ____memcpy(dest, src, n)
    }
}

// Match the source's KASAN aliases as actual symbol aliases, retaining address
// identity instead of adding forwarding functions with different addresses.
#[cfg(CONFIG_KASAN)]
core::arch::global_asm!(
    ".globl __memset",
    ".set __memset, memset",
    ".globl __memmove",
    ".set __memmove, memmove",
    ".globl __memcpy",
    ".set __memcpy, memcpy",
);

// SOURCE-COMMIT: 8e8505218ff400546323d71109207c781769e54e
