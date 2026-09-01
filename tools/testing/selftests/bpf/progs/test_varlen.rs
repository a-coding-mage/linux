// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

/* Dependencies in the original C source:
 * vmlinux.h, bpf/bpf_helpers.h, bpf/bpf_tracing.h, bpf/bpf_core_read.h
 */

const MAX_LEN: usize = 256;

type __u64 = u64;
type u32 = u32;

unsafe extern "C" {
    fn bpf_get_current_pid_tgid() -> __u64;
    fn bpf_probe_read_kernel_str(dst: *mut core::ffi::c_void, size: u32, unsafe_ptr: *const core::ffi::c_void) -> i64;
    fn bpf_probe_read_kernel(dst: *mut core::ffi::c_void, size: u32, unsafe_ptr: *const core::ffi::c_void) -> i64;
}

#[unsafe(no_mangle)]
static mut buf_in1: [core::ffi::c_char; MAX_LEN] = [0; MAX_LEN];
#[unsafe(no_mangle)]
static mut buf_in2: [core::ffi::c_char; MAX_LEN] = [0; MAX_LEN];

#[unsafe(no_mangle)]
static mut test_pid: core::ffi::c_int = 0;
#[unsafe(no_mangle)]
static mut capture: bool = false;

/* .bss */
#[unsafe(no_mangle)]
static mut payload1_len1: __u64 = 0;
#[unsafe(no_mangle)]
static mut payload1_len2: __u64 = 0;
#[unsafe(no_mangle)]
static mut total1: __u64 = 0;
#[unsafe(no_mangle)]
static mut payload1: [core::ffi::c_char; MAX_LEN + MAX_LEN] = [0; MAX_LEN + MAX_LEN];
#[unsafe(no_mangle)]
static mut ret_bad_read: __u64 = 0;

/* .data */
#[unsafe(no_mangle)]
static mut payload2_len1: core::ffi::c_int = -1;
#[unsafe(no_mangle)]
static mut payload2_len2: core::ffi::c_int = -1;
#[unsafe(no_mangle)]
static mut total2: core::ffi::c_int = -1;
#[unsafe(no_mangle)]
static mut payload2: [core::ffi::c_char; MAX_LEN + MAX_LEN] = {
    let mut a = [0; MAX_LEN + MAX_LEN];
    a[0] = 1;
    a
};

#[unsafe(no_mangle)]
static mut payload3_len1: core::ffi::c_int = -1;
#[unsafe(no_mangle)]
static mut payload3_len2: core::ffi::c_int = -1;
#[unsafe(no_mangle)]
static mut total3: core::ffi::c_int = -1;
#[unsafe(no_mangle)]
static mut payload3: [core::ffi::c_char; MAX_LEN + MAX_LEN] = {
    let mut a = [0; MAX_LEN + MAX_LEN];
    a[0] = 1;
    a
};

#[unsafe(no_mangle)]
static mut payload4_len1: core::ffi::c_int = -1;
#[unsafe(no_mangle)]
static mut payload4_len2: core::ffi::c_int = -1;
#[unsafe(no_mangle)]
static mut total4: core::ffi::c_int = -1;
#[unsafe(no_mangle)]
static mut payload4: [core::ffi::c_char; MAX_LEN + MAX_LEN] = {
    let mut a = [0; MAX_LEN + MAX_LEN];
    a[0] = 1;
    a
};

#[unsafe(no_mangle)]
static mut payload_bad: [core::ffi::c_char; 5] = [0x42, 0x42, 0x42, 0x42, 0x42];

#[unsafe(no_mangle)]
#[unsafe(link_section = "raw_tp/sys_enter")]
pub unsafe extern "C" fn handler64_unsigned(regs: *mut core::ffi::c_void) -> core::ffi::c_int {
    let pid: core::ffi::c_int = (bpf_get_current_pid_tgid() >> 32) as core::ffi::c_int;
    let mut payload: *mut core::ffi::c_void = core::ptr::addr_of_mut!(payload1) as *mut core::ffi::c_void;
    let mut len: i64;

    /* ignore irrelevant invocations */
    if test_pid != pid || !capture {
        return 0;
    }

    len = bpf_probe_read_kernel_str(
        payload,
        MAX_LEN as u32,
        core::ptr::addr_of!(buf_in1[0]) as *const core::ffi::c_void,
    );
    if len >= 0 {
        payload = (payload as *mut u8).add(len as usize) as *mut core::ffi::c_void;
        payload1_len1 = len as __u64;
    }

    len = bpf_probe_read_kernel_str(
        payload,
        MAX_LEN as u32,
        core::ptr::addr_of!(buf_in2[0]) as *const core::ffi::c_void,
    );
    if len >= 0 {
        payload = (payload as *mut u8).add(len as usize) as *mut core::ffi::c_void;
        payload1_len2 = len as __u64;
    }

    total1 = (payload as isize - core::ptr::addr_of!(payload1) as *const core::ffi::c_void as isize) as __u64;

    ret_bad_read = bpf_probe_read_kernel_str(
        core::ptr::addr_of_mut!(payload_bad[2]) as *mut core::ffi::c_void,
        1,
        (-1isize) as *const core::ffi::c_void,
    ) as __u64;

    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "raw_tp/sys_exit")]
pub unsafe extern "C" fn handler64_signed(regs: *mut core::ffi::c_void) -> core::ffi::c_int {
    let pid: core::ffi::c_int = (bpf_get_current_pid_tgid() >> 32) as core::ffi::c_int;
    let mut payload: *mut core::ffi::c_void = core::ptr::addr_of_mut!(payload3) as *mut core::ffi::c_void;
    let mut len: i64;

    /* ignore irrelevant invocations */
    if test_pid != pid || !capture {
        return 0;
    }

    len = bpf_probe_read_kernel_str(
        payload,
        MAX_LEN as u32,
        core::ptr::addr_of!(buf_in1[0]) as *const core::ffi::c_void,
    );
    if len >= 0 {
        payload = (payload as *mut u8).add(len as usize) as *mut core::ffi::c_void;
        payload3_len1 = len as core::ffi::c_int;
    }
    len = bpf_probe_read_kernel_str(
        payload,
        MAX_LEN as u32,
        core::ptr::addr_of!(buf_in2[0]) as *const core::ffi::c_void,
    );
    if len >= 0 {
        payload = (payload as *mut u8).add(len as usize) as *mut core::ffi::c_void;
        payload3_len2 = len as core::ffi::c_int;
    }
    total3 = (payload as isize - core::ptr::addr_of!(payload3) as *const core::ffi::c_void as isize) as core::ffi::c_int;

    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "tp/raw_syscalls/sys_enter")]
pub unsafe extern "C" fn handler32_unsigned(regs: *mut core::ffi::c_void) -> core::ffi::c_int {
    let pid: core::ffi::c_int = (bpf_get_current_pid_tgid() >> 32) as core::ffi::c_int;
    let mut payload: *mut core::ffi::c_void = core::ptr::addr_of_mut!(payload2) as *mut core::ffi::c_void;
    let mut len: u32;

    /* ignore irrelevant invocations */
    if test_pid != pid || !capture {
        return 0;
    }

    len = bpf_probe_read_kernel_str(
        payload,
        MAX_LEN as u32,
        core::ptr::addr_of!(buf_in1[0]) as *const core::ffi::c_void,
    ) as u32;
    if len <= MAX_LEN as u32 {
        payload = (payload as *mut u8).add(len as usize) as *mut core::ffi::c_void;
        payload2_len1 = len as core::ffi::c_int;
    }

    len = bpf_probe_read_kernel_str(
        payload,
        MAX_LEN as u32,
        core::ptr::addr_of!(buf_in2[0]) as *const core::ffi::c_void,
    ) as u32;
    if len <= MAX_LEN as u32 {
        payload = (payload as *mut u8).add(len as usize) as *mut core::ffi::c_void;
        payload2_len2 = len as core::ffi::c_int;
    }

    total2 = (payload as isize - core::ptr::addr_of!(payload2) as *const core::ffi::c_void as isize) as core::ffi::c_int;

    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "tp/raw_syscalls/sys_exit")]
pub unsafe extern "C" fn handler32_signed(regs: *mut core::ffi::c_void) -> core::ffi::c_int {
    let pid: core::ffi::c_int = (bpf_get_current_pid_tgid() >> 32) as core::ffi::c_int;
    let mut payload: *mut core::ffi::c_void = core::ptr::addr_of_mut!(payload4) as *mut core::ffi::c_void;
    let mut len: i64;

    /* ignore irrelevant invocations */
    if test_pid != pid || !capture {
        return 0;
    }

    len = bpf_probe_read_kernel_str(
        payload,
        MAX_LEN as u32,
        core::ptr::addr_of!(buf_in1[0]) as *const core::ffi::c_void,
    );
    if len >= 0 {
        payload = (payload as *mut u8).add(len as usize) as *mut core::ffi::c_void;
        payload4_len1 = len as core::ffi::c_int;
    }
    len = bpf_probe_read_kernel_str(
        payload,
        MAX_LEN as u32,
        core::ptr::addr_of!(buf_in2[0]) as *const core::ffi::c_void,
    );
    if len >= 0 {
        payload = (payload as *mut u8).add(len as usize) as *mut core::ffi::c_void;
        payload4_len2 = len as core::ffi::c_int;
    }
    total4 = (payload as isize - core::ptr::addr_of!(payload4) as *const core::ffi::c_void as isize) as core::ffi::c_int;

    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "tp/syscalls/sys_exit_getpid")]
pub unsafe extern "C" fn handler_exit(regs: *mut core::ffi::c_void) -> core::ffi::c_int {
    let mut bla: i64 = 0;

    if bpf_probe_read_kernel(
        core::ptr::addr_of_mut!(bla) as *mut core::ffi::c_void,
        core::mem::size_of_val(&bla) as u32,
        core::ptr::null(),
    ) != 0 {
        return 1;
    } else {
        return 0;
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
static mut LICENSE: [core::ffi::c_char; 4] = [b'G' as core::ffi::c_char, b'P' as core::ffi::c_char, b'L' as core::ffi::c_char, 0];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
