// SPDX-License-Identifier: GPL-2.0
// C dependencies: "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

extern "C" {
    fn PT_REGS_IP(regs: *const core::ffi::c_void) -> uintptr_t;
}

type uintptr_t = usize;

#[no_mangle]
pub static mut ip: uintptr_t = 0;

#[no_mangle]
#[link_section = "perf_event"]
pub unsafe extern "C" fn handler(data: *mut bpf_perf_event_data) -> core::ffi::c_int {
    /* Skip events that have the correct ip. */
    (ip != PT_REGS_IP(core::ptr::addr_of!((*data).regs).cast())) as core::ffi::c_int
}

#[repr(C)]
pub struct bpf_perf_event_data {
    pub regs: core::ffi::c_void,
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";
