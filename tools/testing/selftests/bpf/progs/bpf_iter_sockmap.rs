// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Cloudflare */
/*
 * Translated from C. Original dependencies:
 * <vmlinux.h>, "bpf_tracing_net.h", <bpf/bpf_helpers.h>,
 * <bpf/bpf_tracing.h>, and <errno.h>.
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::c_void;
use core::mem::size_of;

type __u32 = u32;
type __u64 = u64;

const BPF_MAP_TYPE_SOCKMAP: __u32 = 15;
const BPF_MAP_TYPE_SOCKHASH: __u32 = 18;
const ENOENT: i32 = 2;

#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_iter__sockmap {
    pub sk: *mut sock,
    pub key: *mut __u32,
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
    pub max_entries: __u32,
}

unsafe extern "C" {
    fn bpf_map_update_elem(
        map: *mut c_void,
        key: *const c_void,
        value: *const c_void,
        flags: __u64,
    ) -> i32;
    fn bpf_map_delete_elem(map: *mut c_void, key: *const c_void) -> i32;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut sockmap: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SOCKMAP,
    key_size: size_of::<__u32>() as __u32,
    value_size: size_of::<__u64>() as __u32,
    max_entries: 64,
};

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut sockhash: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SOCKHASH,
    key_size: size_of::<__u32>() as __u32,
    value_size: size_of::<__u64>() as __u32,
    max_entries: 64,
};

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut dst: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SOCKHASH,
    key_size: size_of::<__u32>() as __u32,
    value_size: size_of::<__u64>() as __u32,
    max_entries: 64,
};

#[unsafe(no_mangle)]
pub static mut elems: __u32 = 0;
#[unsafe(no_mangle)]
pub static mut socks: __u32 = 0;

#[unsafe(link_section = "iter/sockmap")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn copy(ctx: *mut bpf_iter__sockmap) -> i32 {
    let sk: *mut sock = unsafe { (*ctx).sk };
    let key: *mut __u32 = unsafe { (*ctx).key };
    let ret: i32;

    if key.is_null() {
        return 0;
    }

    unsafe {
        elems = elems.wrapping_add(1);
    }

    /* We need a temporary buffer on the stack, since the verifier doesn't
     * let us use the pointer from the context as an argument to the helper.
     */
    let tmp: __u32 = unsafe { *key };

    if !sk.is_null() {
        unsafe {
            socks = socks.wrapping_add(1);
        }
        return (unsafe {
            bpf_map_update_elem(
                &raw mut dst as *mut c_void,
                &tmp as *const __u32 as *const c_void,
                sk as *const c_void,
                0,
            )
        } != 0) as i32;
    }

    ret = unsafe {
        bpf_map_delete_elem(
            &raw mut dst as *mut c_void,
            &tmp as *const __u32 as *const c_void,
        )
    };
    (ret != 0 && ret != -ENOENT) as i32
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
