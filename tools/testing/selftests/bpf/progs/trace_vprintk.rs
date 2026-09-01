// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

// C dependencies intentionally preserved as external build context:
// "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>, "bpf_misc.h".

unsafe extern "C" {
    fn bpf_printk(fmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn __bpf_vprintk(
        fmt: *const ::core::ffi::c_char,
        args: ...
    ) -> ::core::ffi::c_int;
    fn bpf_trace_vprintk(
        fmt: *const ::core::ffi::c_char,
        fmt_size: ::core::ffi::c_uint,
        data: *const ::core::ffi::c_void,
        data_len: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
}

// SEC("license")
#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [::core::ffi::c_char; 4] = [b'G' as _, b'P' as _, b'L' as _, 0];

#[unsafe(no_mangle)]
pub static mut null_data_vprintk_ret: ::core::ffi::c_int = 0;
#[unsafe(no_mangle)]
pub static mut trace_vprintk_ret: ::core::ffi::c_int = 0;
#[unsafe(no_mangle)]
pub static mut trace_vprintk_ran: ::core::ffi::c_int = 0;

// SEC("fentry/" SYS_PREFIX "sys_nanosleep")
#[unsafe(link_section = "fentry/sys_nanosleep")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_enter(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    static one: [::core::ffi::c_char; 2] = [b'1' as _, 0];
    static three: [::core::ffi::c_char; 2] = [b'3' as _, 0];
    static five: [::core::ffi::c_char; 2] = [b'5' as _, 0];
    static seven: [::core::ffi::c_char; 2] = [b'7' as _, 0];
    static nine: [::core::ffi::c_char; 2] = [b'9' as _, 0];
    static f: [::core::ffi::c_char; 5] = [b'%' as _, b'p' as _, b'S' as _, b'\n' as _, 0];

    let _ = ctx;

    /* runner doesn't search for \t, just ensure it compiles */
    bpf_printk(c"\t".as_ptr());

    trace_vprintk_ran += 1;
    trace_vprintk_ret = __bpf_vprintk(
        c"%s,%d,%s,%d,%s,%d,%s,%d,%s,%d %d\n".as_ptr(),
        one.as_ptr(),
        2,
        three.as_ptr(),
        4,
        five.as_ptr(),
        6,
        seven.as_ptr(),
        8,
        nine.as_ptr(),
        10,
        trace_vprintk_ran,
    );

    /* non-NULL fmt w/ NULL data should result in error */
    null_data_vprintk_ret =
        bpf_trace_vprintk(f.as_ptr(), ::core::mem::size_of_val(&f) as _, ::core::ptr::null(), 0);
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
