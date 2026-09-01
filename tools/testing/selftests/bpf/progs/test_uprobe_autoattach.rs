// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022, Oracle and/or its affiliates. */

/* Dependencies in the original C source:
 * - "vmlinux.h"
 * - <bpf/bpf_core_read.h>
 * - <bpf/bpf_helpers.h>
 * - <bpf/bpf_tracing.h>
 * - "bpf_misc.h"
 */

type u64 = ::core::ffi::c_ulonglong;

#[repr(C)]
pub struct pt_regs {
    _unused: [u8; 0],
}

unsafe extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
    fn PT_REGS_PARM1_CORE(ctx: *mut pt_regs) -> ::core::ffi::c_int;
    fn PT_REGS_RC_CORE(ctx: *mut pt_regs) -> ::core::ffi::c_int;
}

#[unsafe(no_mangle)]
pub static mut uprobe_byname_parm1: ::core::ffi::c_int = 0;
#[unsafe(no_mangle)]
pub static mut uprobe_byname_ran: ::core::ffi::c_int = 0;
#[unsafe(no_mangle)]
pub static mut uretprobe_byname_rc: ::core::ffi::c_int = 0;
#[unsafe(no_mangle)]
pub static mut uretprobe_byname_ret: ::core::ffi::c_int = 0;
#[unsafe(no_mangle)]
pub static mut uretprobe_byname_ran: ::core::ffi::c_int = 0;
#[unsafe(no_mangle)]
pub static mut uprobe_byname2_parm1: u64 = 0;
#[unsafe(no_mangle)]
pub static mut uprobe_byname2_ran: ::core::ffi::c_int = 0;
#[unsafe(no_mangle)]
pub static mut uretprobe_byname2_rc: u64 = 0;
#[unsafe(no_mangle)]
pub static mut uretprobe_byname2_ran: ::core::ffi::c_int = 0;

#[unsafe(no_mangle)]
pub static mut test_pid: ::core::ffi::c_int = 0;

#[unsafe(no_mangle)]
pub static mut a: [::core::ffi::c_int; 8] = [0; 8];

/* This program cannot auto-attach, but that should not stop other
 * programs from attaching.
 */
/* SEC("uprobe") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_uprobe_noautoattach(_ctx: *mut pt_regs) -> ::core::ffi::c_int {
    return 0;
}

/* SEC("uprobe//proc/self/exe:autoattach_trigger_func")
 *
 * Original C used BPF_UPROBE(handle_uprobe_byname, int arg1, int arg2, int arg3,
 * plus arg4..arg8 under FUNC_REG_ARG_CNT > 3 .. > 7).
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_uprobe_byname(
    ctx: *mut pt_regs,
    arg1: ::core::ffi::c_int,
    arg2: ::core::ffi::c_int,
    arg3: ::core::ffi::c_int,
    arg4: ::core::ffi::c_int,
    arg5: ::core::ffi::c_int,
    arg6: ::core::ffi::c_int,
    arg7: ::core::ffi::c_int,
    arg8: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    uprobe_byname_parm1 = PT_REGS_PARM1_CORE(ctx);
    uprobe_byname_ran = 1;

    a[0] = arg1;
    a[1] = arg2;
    a[2] = arg3;
    /* Original C condition: #if FUNC_REG_ARG_CNT > 3 */
    a[3] = arg4;
    /* Original C condition: #if FUNC_REG_ARG_CNT > 4 */
    a[4] = arg5;
    /* Original C condition: #if FUNC_REG_ARG_CNT > 5 */
    a[5] = arg6;
    /* Original C condition: #if FUNC_REG_ARG_CNT > 6 */
    a[6] = arg7;
    /* Original C condition: #if FUNC_REG_ARG_CNT > 7 */
    a[7] = arg8;
    return 0;
}

/* SEC("uretprobe//proc/self/exe:autoattach_trigger_func") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_uretprobe_byname(
    ctx: *mut pt_regs,
    ret: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    uretprobe_byname_rc = PT_REGS_RC_CORE(ctx);
    uretprobe_byname_ret = ret;
    uretprobe_byname_ran = 2;

    return 0;
}

/* SEC("uprobe/libc.so.6:fopen") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_uprobe_byname2(
    _ctx: *mut pt_regs,
    pathname: *const ::core::ffi::c_char,
    _mode: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let pid: ::core::ffi::c_int = (bpf_get_current_pid_tgid() >> 32) as ::core::ffi::c_int;

    /* ignore irrelevant invocations */
    if test_pid != pid {
        return 0;
    }
    uprobe_byname2_parm1 = pathname as isize as u64;
    uprobe_byname2_ran = 3;
    return 0;
}

/* SEC("uretprobe/libc.so.6:fopen") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_uretprobe_byname2(
    _ctx: *mut pt_regs,
    ret: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let pid: ::core::ffi::c_int = (bpf_get_current_pid_tgid() >> 32) as ::core::ffi::c_int;

    /* ignore irrelevant invocations */
    if test_pid != pid {
        return 0;
    }
    uretprobe_byname2_rc = ret as isize as u64;
    uretprobe_byname2_ran = 4;
    return 0;
}

/* char _license[] SEC("license") = "GPL"; */
#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static _license: [::core::ffi::c_char; 4] = [b'G' as ::core::ffi::c_char, b'P' as ::core::ffi::c_char, b'L' as ::core::ffi::c_char, 0];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
