// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2024 Andrea Righi <andrea.righi@canonical.com>

// C includes translated as external dependencies:
// <linux/bpf.h>, <sched.h>, <unistd.h>, <bpf/bpf_helpers.h>, "bpf_misc.h"

const TASK_COMM_LEN: usize = 16;
const BPF_MAP_TYPE_RINGBUF: u32 = 27;

#[repr(C)]
pub struct sample {
    pub pid: i32,
    pub value: i64,
    pub comm: [core::ffi::c_char; 16],
}

#[repr(C)]
pub struct ringbuf_map {
    pub type_: u32,
}

#[used]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[used]
#[unsafe(link_section = ".maps")]
pub static mut ringbuf: ringbuf_map = ringbuf_map {
    type_: BPF_MAP_TYPE_RINGBUF,
};

pub static mut pid: i32 = 0;
pub static mut value: i64 = 0;

unsafe extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_ringbuf_reserve(
        ringbuf: *mut ringbuf_map,
        size: usize,
        flags: u64,
    ) -> *mut core::ffi::c_void;
    fn bpf_get_current_comm(buf: *mut core::ffi::c_char, size_of_buf: u32) -> i64;
    fn bpf_ringbuf_submit(data: *mut core::ffi::c_void, flags: u64);
}

// SEC("fentry/" SYS_PREFIX "sys_getpgid")
#[unsafe(no_mangle)]
#[unsafe(link_section = "fentry/sys_getpgid")]
pub unsafe extern "C" fn test_ringbuf_n(ctx: *mut core::ffi::c_void) -> i32 {
    let cur_pid: i32 = (unsafe { bpf_get_current_pid_tgid() } >> 32) as i32;
    let sample: *mut sample;

    let _ = ctx;

    if cur_pid != unsafe { pid } {
        return 0;
    }

    sample = unsafe {
        bpf_ringbuf_reserve(
            &raw mut ringbuf,
            core::mem::size_of::<sample>(),
            0,
        ) as *mut sample
    };
    if sample.is_null() {
        return 0;
    }

    unsafe {
        (*sample).pid = pid;
        (*sample).value = value;
        bpf_get_current_comm(
            (*sample).comm.as_mut_ptr(),
            core::mem::size_of_val(&(*sample).comm) as u32,
        );

        bpf_ringbuf_submit(sample as *mut core::ffi::c_void, 0);
    }

    return 0;
}
