/* SPDX-License-Identifier: GPL-2.0 */

// The original header includes <asm/string_32.h> when CONFIG_X86_32 is
// enabled, and <asm/string_64.h> otherwise. Those build-time dependencies
// are supplied by the surrounding translation unit.

#[inline(always)]
unsafe fn __inline_memcpy(
    to: *mut core::ffi::c_void,
    from: *const core::ffi::c_void,
    len: usize,
) -> *mut core::ffi::c_void {
    let ret = to;

    core::arch::asm!(
        "rep movsb",
        inout("rdi") to => _,
        inout("rsi") from => _,
        inout("rcx") len => _,
        options(nostack, preserves_flags),
    );
    ret
}

#[inline(always)]
unsafe fn __inline_memset(
    s: *mut core::ffi::c_void,
    v: i32,
    n: usize,
) -> *mut core::ffi::c_void {
    let ret = s;

    core::arch::asm!(
        "rep stosb",
        inout("rdi") s => _,
        inout("rcx") n => _,
        in("rax") (v as u8),
        options(nostack, preserves_flags),
    );
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
