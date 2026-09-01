// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

/* Depends on test_progs.h and trace_vprintk.lskel.h definitions. */

#[repr(C)]
pub struct bpf_prog_info {
    pub verified_insns: u32,
}

#[repr(C)]
pub struct trace_vprintk_lskel {
    pub progs: trace_vprintk_lskel_progs,
}

#[repr(C)]
pub struct trace_vprintk_lskel_progs {
    pub sys_enter: trace_vprintk_lskel_sys_enter,
}

#[repr(C)]
pub struct trace_vprintk_lskel_sys_enter {
    pub prog_fd: i32,
}

unsafe extern "C" {
    fn trace_vprintk_lskel__open_and_load() -> *mut trace_vprintk_lskel;
    fn trace_vprintk_lskel__destroy(skel: *mut trace_vprintk_lskel);
    fn bpf_prog_get_info_by_fd(
        prog_fd: i32,
        info: *mut bpf_prog_info,
        len: *mut u32,
    ) -> i32;
    fn ASSERT_OK_PTR(ptr: *mut trace_vprintk_lskel, name: *const i8) -> bool;
    fn ASSERT_OK(err: i32, name: *const i8) -> bool;
    fn ASSERT_GT(value: u32, threshold: u32, name: *const i8) -> bool;
}

#[no_mangle]
pub unsafe extern "C" fn test_verif_stats() {
    let mut len: u32 = ::core::mem::size_of::<bpf_prog_info>() as u32;
    let mut skel: *mut trace_vprintk_lskel;
    let mut info: bpf_prog_info = unsafe { ::core::mem::zeroed() };
    let err: i32;

    skel = unsafe { trace_vprintk_lskel__open_and_load() };
    if !unsafe { ASSERT_OK_PTR(skel, c"trace_vprintk__open_and_load".as_ptr()) } {
        unsafe { trace_vprintk_lskel__destroy(skel) };
        return;
    }

    err = unsafe {
        bpf_prog_get_info_by_fd(
            (*skel).progs.sys_enter.prog_fd,
            &mut info,
            &mut len,
        )
    };
    if !unsafe { ASSERT_OK(err, c"bpf_prog_get_info_by_fd".as_ptr()) } {
        unsafe { trace_vprintk_lskel__destroy(skel) };
        return;
    }

    if !unsafe { ASSERT_GT(info.verified_insns, 0, c"verified_insns".as_ptr()) } {
        unsafe { trace_vprintk_lskel__destroy(skel) };
        return;
    }

    unsafe { trace_vprintk_lskel__destroy(skel) };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
