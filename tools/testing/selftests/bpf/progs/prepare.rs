// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta */

/* Dependencies from the original C includes:
 * - <vmlinux.h> supplies __sk_buff and __u32.
 * - <bpf/bpf_helpers.h> supplies SEC(), __uint(), and __type().
 */

pub type __u32 = u32;

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[used]
#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut err: i32 = 0;

/* Original C map declaration:
 * struct {
 *      __uint(type, BPF_MAP_TYPE_RINGBUF);
 *      __uint(max_entries, 4096);
 * } ringbuf SEC(".maps");
 */
#[repr(C)]
pub struct ringbuf_def {
    pub type_: *const [i32; BPF_MAP_TYPE_RINGBUF as usize],
    pub max_entries: *const [i32; 4096],
}

/* BPF_MAP_TYPE_RINGBUF is supplied by external BPF headers. */
pub const BPF_MAP_TYPE_RINGBUF: i32 = 27;

#[used]
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut ringbuf: ringbuf_def = ringbuf_def {
    type_: core::ptr::null(),
    max_entries: core::ptr::null(),
};

/* Original C map declaration:
 * struct {
 *      __uint(type, BPF_MAP_TYPE_ARRAY);
 *      __uint(max_entries, 1);
 *      __type(key, __u32);
 *      __type(value, __u32);
 * } array_map SEC(".maps");
 */
#[repr(C)]
pub struct array_map_def {
    pub type_: *const [i32; BPF_MAP_TYPE_ARRAY as usize],
    pub max_entries: *const [i32; 1],
    pub key: *const __u32,
    pub value: *const __u32,
}

/* BPF_MAP_TYPE_ARRAY is supplied by external BPF headers. */
pub const BPF_MAP_TYPE_ARRAY: i32 = 2;

#[used]
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut array_map: array_map_def = array_map_def {
    type_: core::ptr::null(),
    max_entries: core::ptr::null(),
    key: core::ptr::null(),
    value: core::ptr::null(),
};

#[unsafe(link_section = "cgroup_skb/egress")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn program(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    err = 0;
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
