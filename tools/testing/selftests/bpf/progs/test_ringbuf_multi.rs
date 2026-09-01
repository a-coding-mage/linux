// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Facebook

// C dependencies translated as external BPF/libbpf-provided symbols:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>

#[used]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct sample {
    pub pid: i32,
    pub seq: i32,
    pub value: i64,
    pub comm: [i8; 16],
}

// Map type constants are provided by <linux/bpf.h> in the original C source.
unsafe extern "C" {
    pub static BPF_MAP_TYPE_RINGBUF: u32;
    pub static BPF_MAP_TYPE_ARRAY_OF_MAPS: u32;
    pub static BPF_MAP_TYPE_HASH_OF_MAPS: u32;
}

#[repr(C)]
pub struct ringbuf_map {
    // __uint(type, BPF_MAP_TYPE_RINGBUF);
    pub type_: *const u32,
    // libbpf will adjust to valid page size
    // __uint(max_entries, 1000);
    pub max_entries: *const u32,
}

#[used]
#[unsafe(link_section = ".maps")]
pub static mut ringbuf1: ringbuf_map = ringbuf_map {
    type_: unsafe { &BPF_MAP_TYPE_RINGBUF as *const u32 },
    max_entries: 1000 as *const u32,
};

#[used]
#[unsafe(link_section = ".maps")]
pub static mut ringbuf2: ringbuf_map = ringbuf_map {
    type_: unsafe { &BPF_MAP_TYPE_RINGBUF as *const u32 },
    max_entries: 1000 as *const u32,
};

#[repr(C)]
pub struct ringbuf_arr_t {
    // __uint(type, BPF_MAP_TYPE_ARRAY_OF_MAPS);
    pub type_: *const u32,
    // __uint(max_entries, 4);
    pub max_entries: *const u32,
    // __type(key, int);
    pub key: *const i32,
    // __array(values, struct ringbuf_map);
    pub values: [*mut ringbuf_map; 4],
}

#[used]
#[unsafe(link_section = ".maps")]
pub static mut ringbuf_arr: ringbuf_arr_t = ringbuf_arr_t {
    type_: unsafe { &BPF_MAP_TYPE_ARRAY_OF_MAPS as *const u32 },
    max_entries: 4 as *const u32,
    key: core::ptr::null(),
    values: [
        unsafe { &mut ringbuf1 as *mut ringbuf_map },
        core::ptr::null_mut(),
        unsafe { &mut ringbuf2 as *mut ringbuf_map },
        core::ptr::null_mut(),
    ],
};

#[repr(C)]
pub struct ringbuf_hash_t {
    // __uint(type, BPF_MAP_TYPE_HASH_OF_MAPS);
    pub type_: *const u32,
    // __uint(max_entries, 1);
    pub max_entries: *const u32,
    // __type(key, int);
    pub key: *const i32,
    // __array(values, struct ringbuf_map);
    pub values: [*mut ringbuf_map; 1],
}

#[used]
#[unsafe(link_section = ".maps")]
pub static mut ringbuf_hash: ringbuf_hash_t = ringbuf_hash_t {
    type_: unsafe { &BPF_MAP_TYPE_HASH_OF_MAPS as *const u32 },
    max_entries: 1 as *const u32,
    key: core::ptr::null(),
    values: [unsafe { &mut ringbuf1 as *mut ringbuf_map }],
};

/* inputs */
pub static mut pid: i32 = 0;
pub static mut target_ring: i32 = 0;
pub static mut value: i64 = 0;

/* outputs */
pub static mut total: i64 = 0;
pub static mut dropped: i64 = 0;
pub static mut skipped: i64 = 0;

unsafe extern "C" {
    pub fn bpf_get_current_pid_tgid() -> u64;
    pub fn bpf_map_lookup_elem(map: *const core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    pub fn bpf_ringbuf_reserve(ringbuf: *mut core::ffi::c_void, size: u64, flags: u64) -> *mut core::ffi::c_void;
    pub fn bpf_get_current_comm(buf: *mut core::ffi::c_void, size_of_buf: u32) -> i64;
    pub fn bpf_ringbuf_submit(data: *mut core::ffi::c_void, flags: u64);
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "tp/syscalls/sys_enter_getpgid")]
pub unsafe extern "C" fn test_ringbuf(ctx: *mut core::ffi::c_void) -> i32 {
    let cur_pid: i32 = (unsafe { bpf_get_current_pid_tgid() } >> 32) as i32;
    let mut sample: *mut sample;
    let rb: *mut core::ffi::c_void;

    let _ = ctx;

    if cur_pid != unsafe { pid } {
        return 0;
    }

    rb = unsafe {
        bpf_map_lookup_elem(
            &raw const ringbuf_arr as *const core::ffi::c_void,
            &raw const target_ring as *const core::ffi::c_void,
        )
    };
    if rb.is_null() {
        unsafe {
            skipped += 1;
        }
        return 1;
    }

    sample = unsafe {
        bpf_ringbuf_reserve(
            rb,
            core::mem::size_of::<sample>() as u64,
            0,
        ) as *mut sample
    };
    if sample.is_null() {
        unsafe {
            dropped += 1;
        }
        return 1;
    }

    unsafe {
        (*sample).pid = pid;
        bpf_get_current_comm(
            (*sample).comm.as_mut_ptr() as *mut core::ffi::c_void,
            core::mem::size_of_val(&(*sample).comm) as u32,
        );
        (*sample).value = value;

        (*sample).seq = total as i32;
        total += 1;

        bpf_ringbuf_submit(sample as *mut core::ffi::c_void, 0);
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
