// SPDX-License-Identifier: GPL-2.0
// C dependencies: "vmlinux.h" and <bpf/bpf_helpers.h>

/*
 * One explicit array map and no global variables, so the generated loader
 * has exactly one map to create (no .rodata/.bss). prog_tests/signed_loader.c
 * uses this to check that a signed loader ignores ctx-supplied max_entries:
 * the map must keep its attested size (4), not whatever the host puts in
 * the loader ctx.
 */

pub type __u32 = u32;
pub type __u64 = u64;

pub const BPF_MAP_TYPE_ARRAY: u32 = 2;

#[repr(C)]
pub struct amap {
    pub type_: u32,
    pub max_entries: u32,
    pub key: __u32,
    pub value: __u64,
}

#[link_section = ".maps"]
#[no_mangle]
pub static amap: amap = amap {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 4,
    key: 0,
    value: 0,
};

extern "C" {
    pub fn bpf_map_lookup_elem(map: *const amap, key: *const __u32) -> *mut __u64;
}

#[link_section = "socket"]
#[no_mangle]
pub unsafe extern "C" fn probe(ctx: *mut core::ffi::c_void) -> i32 {
    let key: __u32 = 0;
    let val: *mut __u64 = bpf_map_lookup_elem(&amap, &key);

    if !val.is_null() {
        *val as i32
    } else {
        0
    }
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";
