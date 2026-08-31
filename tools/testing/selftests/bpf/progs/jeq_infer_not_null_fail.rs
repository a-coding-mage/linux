// SPDX-License-Identifier: GPL-2.0

// C dependencies removed from executable Rust:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"
//
// Original C condition:
// #ifndef __clang__
// #pragma GCC diagnostic ignored "-Warray-bounds"
// #endif

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::arch::asm;
use core::ffi::c_void;

type u64 = u64;

const BPF_MAP_TYPE_HASH: u32 = 1;

#[repr(C)]
pub struct bpf_map {
    pub inner_map_meta: *mut bpf_map,
}

unsafe extern "C" {
    fn bpf_map_lookup_elem(map: *mut bpf_map, key: *const c_void) -> *mut c_void;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct m_hash_def {
    // __uint(type, BPF_MAP_TYPE_HASH);
    pub type_: u32,
    // __uint(max_entries, 1);
    pub max_entries: u32,
    // __type(key, u64);
    // __type(value, u64);
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut m_hash: m_hash_def = m_hash_def {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 1,
};

// SEC("?raw_tp")
// __failure __msg("R8 invalid mem access 'map_value_or_null")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn jeq_infer_not_null_ptr_to_btfid(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let map: *mut bpf_map = core::ptr::addr_of_mut!(m_hash).cast::<bpf_map>();
    let inner_map: *mut bpf_map = unsafe { (*map).inner_map_meta };
    let key: u64 = 0;
    let mut ret: u64 = 0;
    let val: *mut u64;

    val = unsafe {
        bpf_map_lookup_elem(
            map,
            core::ptr::addr_of!(key).cast::<c_void>(),
        )
        .cast::<u64>()
    };
    /* Do not mark ptr as non-null if one of them is
     * PTR_TO_BTF_ID (R9), reject because of invalid
     * access to map value (R8).
     *
     * Here, we need to inline those insns to access
     * R8 directly, since compiler may use other reg
     * once it figures out val==inner_map.
     */
    unsafe {
        asm!(
            "r8 = {val};",
            "r9 = {inner_map};",
            "if r8 != r9 goto +1;",
            "{ret} = *(u64 *)(r8 +0);",
            ret = inout(reg) ret,
            inner_map = in(reg) inner_map,
            val = in(reg) val,
            out("r8") _,
            out("r9") _,
        );
    }

    ret as i32
}
