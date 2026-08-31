// SPDX-License-Identifier: GPL-2.0
// C dependencies: vmlinux.h, bpf/bpf_helpers.h, bpf/bpf_tracing.h,
// bpf/bpf_core_read.h.

type func_proto_typedef = Option<unsafe extern "C" fn(arg1: i64) -> i32>;
type func_proto_typedef_nested1 = Option<unsafe extern "C" fn(arg1: func_proto_typedef) -> i32>;
type func_proto_typedef_nested2 = Option<unsafe extern "C" fn(arg1: func_proto_typedef_nested1) -> i32>;

#[no_mangle]
pub static mut proto_out: i32 = 0;

extern "C" {
    fn bpf_core_type_exists<T>() -> i32;
}

#[no_mangle]
#[link_section = "raw_tracepoint/sys_enter"]
pub unsafe extern "C" fn core_relo_proto(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;

    proto_out = bpf_core_type_exists::<func_proto_typedef_nested2>();

    0
}

#[no_mangle]
#[link_section = "license"]
pub static LICENSE: [u8; 4] = *b"GPL\0";
