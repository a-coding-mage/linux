// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Facebook

// Dependencies from the original C includes:
// <linux/bpf.h>, <bpf/bpf_helpers.h>, and "bpf_misc.h".

#[repr(C)]
pub struct sample {
    pub pid: i32,
    pub seq: i32,
    pub value: i64,
    pub comm: [i8; 16],
}

#[repr(C)]
pub struct ringbuf_map {
    // Original C used __uint(type, BPF_MAP_TYPE_RINGBUF).
    pub type_: u32,
}

unsafe extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_ringbuf_reserve(ringbuf: *mut ringbuf_map, size: u64, flags: u64) -> *mut core::ffi::c_void;
    fn bpf_get_current_comm(buf: *mut i8, size_of_buf: u32) -> i64;
    fn bpf_ringbuf_output(
        ringbuf: *mut ringbuf_map,
        data: *const core::ffi::c_void,
        size: u64,
        flags: u64,
    ) -> i64;
    fn bpf_ringbuf_discard(data: *mut core::ffi::c_void, flags: u64);
    fn bpf_ringbuf_submit(data: *mut core::ffi::c_void, flags: u64);
    fn bpf_ringbuf_query(ringbuf: *mut ringbuf_map, flags: u64) -> u64;
}

unsafe extern "C" {
    static BPF_MAP_TYPE_RINGBUF: u32;
    static BPF_RB_AVAIL_DATA: u64;
    static BPF_RB_RING_SIZE: u64;
    static BPF_RB_CONS_POS: u64;
    static BPF_RB_PROD_POS: u64;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut ringbuf: ringbuf_map = ringbuf_map {
    type_: 0, // BPF_MAP_TYPE_RINGBUF, supplied by BPF map metadata in the original macro.
};

/* inputs */
#[unsafe(no_mangle)]
pub static mut pid: i32 = 0;
#[unsafe(no_mangle)]
pub static mut value: i64 = 0;
#[unsafe(no_mangle)]
pub static mut flags: i64 = 0;

/* outputs */
#[unsafe(no_mangle)]
pub static mut total: i64 = 0;
#[unsafe(no_mangle)]
pub static mut discarded: i64 = 0;
#[unsafe(no_mangle)]
pub static mut dropped: i64 = 0;

#[unsafe(no_mangle)]
pub static mut avail_data: i64 = 0;
#[unsafe(no_mangle)]
pub static mut ring_size: i64 = 0;
#[unsafe(no_mangle)]
pub static mut cons_pos: i64 = 0;
#[unsafe(no_mangle)]
pub static mut prod_pos: i64 = 0;

/* inner state */
#[unsafe(no_mangle)]
pub static mut seq: i64 = 0;

// Original section: SEC("fentry/" SYS_PREFIX "sys_getpgid")
#[unsafe(link_section = "fentry/sys_getpgid")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_ringbuf(ctx: *mut core::ffi::c_void) -> i32 {
    let cur_pid: i32 = (unsafe { bpf_get_current_pid_tgid() } >> 32) as i32;
    let sample: *mut sample;

    let _ = ctx;

    if cur_pid != unsafe { pid } {
        return 0;
    }

    sample = unsafe {
        bpf_ringbuf_reserve(
            core::ptr::addr_of_mut!(ringbuf),
            core::mem::size_of::<sample>() as u64,
            0,
        ) as *mut sample
    };
    if sample.is_null() {
        unsafe {
            let old = core::ptr::read_volatile(core::ptr::addr_of!(dropped));
            core::ptr::write_volatile(core::ptr::addr_of_mut!(dropped), old.wrapping_add(1));
        }
        return 0;
    }

    unsafe {
        (*sample).pid = pid;
        bpf_get_current_comm(
            (*sample).comm.as_mut_ptr(),
            core::mem::size_of_val(&(*sample).comm) as u32,
        );
        (*sample).value = value;

        (*sample).seq = seq as i32;
        seq = seq.wrapping_add(1);
        let old = core::ptr::read_volatile(core::ptr::addr_of!(total));
        core::ptr::write_volatile(core::ptr::addr_of_mut!(total), old.wrapping_add(1));

        if ((*sample).seq & 1) != 0 {
            /* copy from reserved sample to a new one... */
            bpf_ringbuf_output(
                core::ptr::addr_of_mut!(ringbuf),
                sample as *const core::ffi::c_void,
                core::mem::size_of::<sample>() as u64,
                flags as u64,
            );
            /* ...and then discard reserved sample */
            bpf_ringbuf_discard(sample as *mut core::ffi::c_void, flags as u64);
            let old = core::ptr::read_volatile(core::ptr::addr_of!(discarded));
            core::ptr::write_volatile(core::ptr::addr_of_mut!(discarded), old.wrapping_add(1));
        } else {
            bpf_ringbuf_submit(sample as *mut core::ffi::c_void, flags as u64);
        }

        avail_data = bpf_ringbuf_query(core::ptr::addr_of_mut!(ringbuf), BPF_RB_AVAIL_DATA) as i64;
        ring_size = bpf_ringbuf_query(core::ptr::addr_of_mut!(ringbuf), BPF_RB_RING_SIZE) as i64;
        cons_pos = bpf_ringbuf_query(core::ptr::addr_of_mut!(ringbuf), BPF_RB_CONS_POS) as i64;
        prod_pos = bpf_ringbuf_query(core::ptr::addr_of_mut!(ringbuf), BPF_RB_PROD_POS) as i64;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
