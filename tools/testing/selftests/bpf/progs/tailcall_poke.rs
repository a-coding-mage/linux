// SPDX-License-Identifier: GPL-2.0
// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

use core::ffi::c_void;

#[used]
#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

extern "C" {
    static BPF_MAP_TYPE_PROG_ARRAY: u32;

    fn bpf_tail_call_static(ctx: *mut c_void, map: *const JmpTable, index: u32);
}

#[repr(C)]
pub struct JmpTable {
    pub r#type: *const [i32; 0],
    pub max_entries: *const [i32; 1],
    pub key_size: *const [i32; core::mem::size_of::<u32>()],
    pub value_size: *const [i32; core::mem::size_of::<u32>()],
}

#[used]
#[no_mangle]
#[link_section = ".maps"]
pub static mut jmp_table: JmpTable = JmpTable {
    r#type: BPF_MAP_TYPE_PROG_ARRAY as *const [i32; 0],
    max_entries: core::ptr::null(),
    key_size: core::ptr::null(),
    value_size: core::ptr::null(),
};

// SEC("?fentry/bpf_fentry_test1")
// Original declaration: int BPF_PROG(test, int a)
#[no_mangle]
#[link_section = "?fentry/bpf_fentry_test1"]
pub unsafe extern "C" fn test(ctx: *mut c_void) -> i32 {
    bpf_tail_call_static(ctx, core::ptr::addr_of!(jmp_table), 0);
    0
}

// SEC("fentry/bpf_fentry_test1")
// Original declaration: int BPF_PROG(call1, int a)
#[no_mangle]
#[link_section = "fentry/bpf_fentry_test1"]
pub unsafe extern "C" fn call1(_ctx: *mut c_void) -> i32 {
    0
}

// SEC("fentry/bpf_fentry_test1")
// Original declaration: int BPF_PROG(call2, int a)
#[no_mangle]
#[link_section = "fentry/bpf_fentry_test1"]
pub unsafe extern "C" fn call2(_ctx: *mut c_void) -> i32 {
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
