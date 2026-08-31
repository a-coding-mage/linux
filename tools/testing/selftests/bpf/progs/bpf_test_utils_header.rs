// SPDX-License-Identifier: GPL-2.0
// C dependencies: <bpf/bpf_helpers.h>, "bpf_misc.h"

unsafe extern "C" {
    fn bpf_strtoul(
        buf: *const core::ffi::c_char,
        buf_len: core::ffi::c_ulong,
        flags: core::ffi::c_ulonglong,
        res: *mut core::ffi::c_ulong,
    ) -> core::ffi::c_long;

    fn __sink(arg: core::ffi::c_ulong);
}

/* Clobber as many native registers and stack slots as possible. */
#[inline(always)]
pub unsafe fn clobber_regs_stack() {
    let mut tmp_str: [core::ffi::c_char; 10] = [
        b'1' as core::ffi::c_char,
        b'2' as core::ffi::c_char,
        b'3' as core::ffi::c_char,
        b'4' as core::ffi::c_char,
        b'5' as core::ffi::c_char,
        b'6' as core::ffi::c_char,
        b'7' as core::ffi::c_char,
        b'8' as core::ffi::c_char,
        b'9' as core::ffi::c_char,
        0,
    ];
    let mut tmp: core::ffi::c_ulong = 0;

    unsafe {
        bpf_strtoul(
            tmp_str.as_mut_ptr(),
            core::mem::size_of_val(&tmp_str) as core::ffi::c_ulong,
            0,
            &mut tmp,
        );
        __sink(tmp);
    }
}
