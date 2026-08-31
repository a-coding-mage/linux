// SPDX-License-Identifier: GPL-2.0
/* fails to load without expected_attach_type = BPF_XDP_DEVMAP
 * because of access to egress_ifindex
 */
// C dependencies translated as external Rust dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>

extern "C" {
    fn bpf_trace_printk(fmt: *const u8, fmt_size: u32, ...) -> i64;
}

#[no_mangle]
#[link_section = "xdp"]
pub unsafe extern "C" fn xdpdm_devlog(ctx: *mut xdp_md) -> i32 {
    let fmt = *b"devmap redirect: dev %u -> dev %u len %u\n\0";
    let data_end = (*ctx).data_end as usize as *mut core::ffi::c_void;
    let data = (*ctx).data as usize as *mut core::ffi::c_void;
    let len: u32 = (data_end as usize).wrapping_sub(data as usize) as u32;

    bpf_trace_printk(
        fmt.as_ptr(),
        core::mem::size_of_val(&fmt) as u32,
        (*ctx).ingress_ifindex,
        (*ctx).egress_ifindex,
        len,
    );

    XDP_PASS
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";
