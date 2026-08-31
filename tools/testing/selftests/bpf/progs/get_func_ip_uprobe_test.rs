// SPDX-License-Identifier: GPL-2.0
// C dependencies: "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

use core::ffi::c_void;

#[allow(non_camel_case_types)]
type __u64 = u64;

#[allow(non_camel_case_types)]
type c_ulong = usize;

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

extern "C" {
    fn bpf_get_func_ip(ctx: *mut pt_regs) -> __u64;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut uprobe_trigger_body: c_ulong = 0;

#[no_mangle]
pub static mut test1_result: __u64 = 0;

#[no_mangle]
#[link_section = "uprobe//proc/self/exe:uprobe_trigger_body+1"]
pub unsafe extern "C" fn test1(ctx: *mut pt_regs) -> i32 {
    let addr: __u64 = bpf_get_func_ip(ctx);

    test1_result = ((addr as *const c_void) == ((uprobe_trigger_body as *const c_void).add(1))) as __u64;
    return 0;
}
