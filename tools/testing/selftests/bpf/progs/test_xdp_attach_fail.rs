// SPDX-License-Identifier: GPL-2.0
/* Copyright Leon Hwang */

// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>

const ERRMSG_LEN: usize = 64;

#[repr(C)]
pub struct xdp_errmsg {
    pub msg: [::core::ffi::c_char; ERRMSG_LEN],
}

// Original C BPF map declaration:
// struct {
//     __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
//     __type(key, int);
//     __type(value, int);
// } xdp_errmsg_pb SEC(".maps");
#[repr(C)]
pub struct xdp_errmsg_pb_map {
    _private: [u8; 0],
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut xdp_errmsg_pb: xdp_errmsg_pb_map = xdp_errmsg_pb_map { _private: [] };

#[repr(C)]
pub struct xdp_attach_error_ctx {
    pub unused: ::core::ffi::c_ulong,

    /*
     * bpf does not support tracepoint __data_loc directly.
     *
     * Actually, this field is a 32 bit integer whose value encodes
     * information on where to find the actual data. The first 2 bytes is
     * the size of the data. The last 2 bytes is the offset from the start
     * of the tracepoint struct where the data begins.
     * -- https://github.com/iovisor/bpftrace/pull/1542
     */
    pub msg: u32, // __data_loc char[] msg;
}

extern "C" {
    fn bpf_probe_read_kernel_str(
        dst: *mut ::core::ffi::c_void,
        size: u32,
        unsafe_ptr: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_long;

    fn bpf_perf_event_output(
        ctx: *mut ::core::ffi::c_void,
        map: *mut ::core::ffi::c_void,
        flags: u64,
        data: *mut ::core::ffi::c_void,
        size: u64,
    ) -> ::core::ffi::c_long;
}

extern "C" {
    static BPF_F_CURRENT_CPU: u64;
}

/*
 * Catch the error message at the tracepoint.
 */

#[link_section = "tp/xdp/bpf_xdp_link_attach_failed"]
#[no_mangle]
pub unsafe extern "C" fn tp__xdp__bpf_xdp_link_attach_failed(
    ctx: *mut xdp_attach_error_ctx,
) -> ::core::ffi::c_int {
    let msg: *mut ::core::ffi::c_char =
        (ctx as *mut u8).add((*ctx).msg as u16 as usize) as *mut ::core::ffi::c_char;
    let mut errmsg: xdp_errmsg = xdp_errmsg {
        msg: [0; ERRMSG_LEN],
    };

    bpf_probe_read_kernel_str(
        errmsg.msg.as_mut_ptr() as *mut ::core::ffi::c_void,
        ERRMSG_LEN as u32,
        msg as *const ::core::ffi::c_void,
    );
    bpf_perf_event_output(
        ctx as *mut ::core::ffi::c_void,
        &mut xdp_errmsg_pb as *mut _ as *mut ::core::ffi::c_void,
        BPF_F_CURRENT_CPU,
        &mut errmsg as *mut _ as *mut ::core::ffi::c_void,
        ERRMSG_LEN as u64,
    );
    0
}

/*
 * Reuse the XDP program in xdp_dummy.c.
 */

#[link_section = "license"]
#[no_mangle]
pub static LICENSE: [::core::ffi::c_char; 4] = [b'G' as ::core::ffi::c_char, b'P' as ::core::ffi::c_char, b'L' as ::core::ffi::c_char, 0];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
