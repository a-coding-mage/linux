// SPDX-License-Identifier: GPL-2.0
/* Copyright 2022 Sony Group Corporation */

type c_int = i32;
type c_long = i64;
type c_ulong = u64;
type pid_t = c_int;
type loff_t = i64;
type ssize_t = isize;
type __u64 = u64;

const SPLICE_F_NONBLOCK: c_uint = 0x02;
const EBADF: c_int = 9;

type c_uint = u32;

#[repr(C)]
pub struct bpf_syscall_macro {
    pub rodata: *mut bpf_syscall_macro_rodata,
    pub bss: *mut bpf_syscall_macro_bss,
}

#[repr(C)]
pub struct bpf_syscall_macro_rodata {
    pub filter_pid: pid_t,
}

#[repr(C)]
pub struct bpf_syscall_macro_bss {
    pub arg1: c_int,
    pub arg2: c_ulong,
    pub arg3: c_ulong,
    pub arg4_cx: c_ulong,
    pub arg4: c_ulong,
    pub arg5: c_ulong,
    pub arg1_core: c_int,
    pub arg2_core: c_ulong,
    pub arg3_core: c_ulong,
    pub arg4_core_cx: c_ulong,
    pub arg4_core: c_ulong,
    pub arg5_core: c_ulong,
    pub option_syscall: c_int,
    pub arg2_syscall: c_ulong,
    pub arg3_syscall: c_ulong,
    pub arg4_syscall: c_ulong,
    pub arg5_syscall: c_ulong,
    pub splice_fd_in: c_int,
    pub splice_off_in: __u64,
    pub splice_fd_out: c_int,
    pub splice_off_out: __u64,
    pub splice_len: c_ulong,
    pub splice_flags: c_uint,
}

unsafe extern "C" {
    fn bpf_syscall_macro__open() -> *mut bpf_syscall_macro;
    fn bpf_syscall_macro__load(skel: *mut bpf_syscall_macro) -> c_int;
    fn bpf_syscall_macro__attach(skel: *mut bpf_syscall_macro) -> c_int;
    fn bpf_syscall_macro__destroy(skel: *mut bpf_syscall_macro);

    fn getpid() -> pid_t;
    fn prctl(option: c_int, arg2: c_ulong, arg3: c_ulong, arg4: c_ulong, arg5: c_ulong) -> c_int;
    fn splice(
        fd_in: c_int,
        off_in: *mut loff_t,
        fd_out: c_int,
        off_out: *mut loff_t,
        len: c_ulong,
        flags: c_uint,
    ) -> ssize_t;

    static mut errno: c_int;

    fn ASSERT_OK_PTR(ptr: *const core::ffi::c_void, name: *const u8) -> bool;
    fn ASSERT_OK(err: c_int, name: *const u8) -> bool;
    fn ASSERT_EQ<T>(actual: T, expected: T, name: *const u8) -> bool;
    fn ASSERT_NEQ<T>(actual: T, expected: T, name: *const u8) -> bool;
}

pub unsafe fn test_bpf_syscall_macro() {
    let mut skel: *mut bpf_syscall_macro = core::ptr::null_mut();
    let mut err: c_int;
    let exp_arg1: c_int = 1001;
    let exp_arg2: c_ulong = 12;
    let exp_arg3: c_ulong = 13;
    let exp_arg4: c_ulong = 14;
    let exp_arg5: c_ulong = 15;
    let mut off_in: loff_t = core::mem::zeroed();
    let mut off_out: loff_t = core::mem::zeroed();
    let r: ssize_t;

    /* check whether it can open program */
    skel = bpf_syscall_macro__open();
    if !ASSERT_OK_PTR(skel as *const core::ffi::c_void, b"bpf_syscall_macro__open\0".as_ptr()) {
        return;
    }

    (*(*skel).rodata).filter_pid = getpid();

    /* check whether it can load program */
    err = bpf_syscall_macro__load(skel);
    if !ASSERT_OK(err, b"bpf_syscall_macro__load\0".as_ptr()) {
        bpf_syscall_macro__destroy(skel);
        return;
    }

    /* check whether it can attach kprobe */
    err = bpf_syscall_macro__attach(skel);
    if !ASSERT_OK(err, b"bpf_syscall_macro__attach\0".as_ptr()) {
        bpf_syscall_macro__destroy(skel);
        return;
    }

    /* check whether args of syscall are copied correctly */
    prctl(exp_arg1, exp_arg2, exp_arg3, exp_arg4, exp_arg5);

    ASSERT_EQ((*(*skel).bss).arg1, exp_arg1, b"syscall_arg1\0".as_ptr());
    ASSERT_EQ((*(*skel).bss).arg2, exp_arg2, b"syscall_arg2\0".as_ptr());
    ASSERT_EQ((*(*skel).bss).arg3, exp_arg3, b"syscall_arg3\0".as_ptr());
    /* it cannot copy arg4 when uses PT_REGS_PARM4 on x86_64 */
    #[cfg(target_arch = "x86_64")]
    {
        ASSERT_NEQ((*(*skel).bss).arg4_cx, exp_arg4, b"syscall_arg4_from_cx\0".as_ptr());
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        ASSERT_EQ((*(*skel).bss).arg4_cx, exp_arg4, b"syscall_arg4_from_cx\0".as_ptr());
    }
    ASSERT_EQ((*(*skel).bss).arg4, exp_arg4, b"syscall_arg4\0".as_ptr());
    ASSERT_EQ((*(*skel).bss).arg5, exp_arg5, b"syscall_arg5\0".as_ptr());

    /* check whether args of syscall are copied correctly for CORE variants */
    ASSERT_EQ((*(*skel).bss).arg1_core, exp_arg1, b"syscall_arg1_core_variant\0".as_ptr());
    ASSERT_EQ((*(*skel).bss).arg2_core, exp_arg2, b"syscall_arg2_core_variant\0".as_ptr());
    ASSERT_EQ((*(*skel).bss).arg3_core, exp_arg3, b"syscall_arg3_core_variant\0".as_ptr());
    /* it cannot copy arg4 when uses PT_REGS_PARM4_CORE on x86_64 */
    #[cfg(target_arch = "x86_64")]
    {
        ASSERT_NEQ(
            (*(*skel).bss).arg4_core_cx,
            exp_arg4,
            b"syscall_arg4_from_cx_core_variant\0".as_ptr(),
        );
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        ASSERT_EQ(
            (*(*skel).bss).arg4_core_cx,
            exp_arg4,
            b"syscall_arg4_from_cx_core_variant\0".as_ptr(),
        );
    }
    ASSERT_EQ((*(*skel).bss).arg4_core, exp_arg4, b"syscall_arg4_core_variant\0".as_ptr());
    ASSERT_EQ((*(*skel).bss).arg5_core, exp_arg5, b"syscall_arg5_core_variant\0".as_ptr());

    ASSERT_EQ((*(*skel).bss).option_syscall, exp_arg1, b"BPF_KPROBE_SYSCALL_option\0".as_ptr());
    ASSERT_EQ((*(*skel).bss).arg2_syscall, exp_arg2, b"BPF_KPROBE_SYSCALL_arg2\0".as_ptr());
    ASSERT_EQ((*(*skel).bss).arg3_syscall, exp_arg3, b"BPF_KPROBE_SYSCALL_arg3\0".as_ptr());
    ASSERT_EQ((*(*skel).bss).arg4_syscall, exp_arg4, b"BPF_KPROBE_SYSCALL_arg4\0".as_ptr());
    ASSERT_EQ((*(*skel).bss).arg5_syscall, exp_arg5, b"BPF_KPROBE_SYSCALL_arg5\0".as_ptr());

    r = splice(
        -42,
        &mut off_in,
        42,
        &mut off_out,
        0x12340000,
        SPLICE_F_NONBLOCK,
    );
    err = -errno;
    ASSERT_EQ(r, -1, b"splice_res\0".as_ptr());
    ASSERT_EQ(err, -EBADF, b"splice_err\0".as_ptr());

    ASSERT_EQ((*(*skel).bss).splice_fd_in, -42, b"splice_arg1\0".as_ptr());
    ASSERT_EQ(
        (*(*skel).bss).splice_off_in,
        &mut off_in as *mut loff_t as __u64,
        b"splice_arg2\0".as_ptr(),
    );
    ASSERT_EQ((*(*skel).bss).splice_fd_out, 42, b"splice_arg3\0".as_ptr());
    ASSERT_EQ(
        (*(*skel).bss).splice_off_out,
        &mut off_out as *mut loff_t as __u64,
        b"splice_arg4\0".as_ptr(),
    );
    ASSERT_EQ((*(*skel).bss).splice_len, 0x12340000, b"splice_arg5\0".as_ptr());
    ASSERT_EQ((*(*skel).bss).splice_flags, SPLICE_F_NONBLOCK, b"splice_arg6\0".as_ptr());

    bpf_syscall_macro__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
