/* SPDX-License-Identifier: GPL-2.0 */

/* C header guard removed in Rust translation: _TOOLS_ASM_X86_SPECIAL_INSNS_H */

use core::arch::asm;

#[repr(C)]
struct Movdir64bBlock {
    _: [core::ffi::c_char; 64],
}

/* The dst parameter must be 64-bytes aligned */
pub unsafe fn movdir64b(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
    let __src = src as *const Movdir64bBlock;
    let __dst = dst as *mut Movdir64bBlock;

    /*
     * MOVDIR64B %(rdx), rax.
     *
     * Both __src and __dst must be memory constraints in order to tell the
     * compiler that no other memory accesses should be reordered around
     * this one.
     *
     * Also, both must be supplied as lvalues because this tells
     * the compiler what the object is (its size) the instruction accesses.
     * I.e., not the pointers but what they point to, thus the deref'ing '*'.
     */
    asm!(
        ".byte 0x66, 0x0f, 0x38, 0xf8, 0x02",
        in("rax") __dst,
        in("rdx") __src,
        inout("memory") *__dst,
        in("memory") *__src,
        options(nostack, preserves_flags),
    );
}
