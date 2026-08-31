// SPDX-License-Identifier: GPL-2.0

// Depends on definitions from vmlinux.h, bpf_helpers.h, and bpf_tracing.h.

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

extern "C" {
    pub type sock;
    pub type tcp_cc_info;
}

#[repr(C)]
pub struct tcp_congestion_ops {
    pub get_info: *mut core::ffi::c_void,
    pub name: [core::ffi::c_char; 16],
}

pub type u32 = core::ffi::c_uint;
pub type size_t = usize;

#[no_mangle]
#[link_section = "struct_ops"]
pub unsafe extern "C" fn unsupp_cong_op_get_info(
    sk: *mut sock,
    ext: u32,
    attr: *mut core::ffi::c_int,
    info: *mut tcp_cc_info,
) -> size_t {
    let _ = sk;
    let _ = ext;
    let _ = attr;
    let _ = info;

    0
}

#[no_mangle]
#[link_section = ".struct_ops"]
pub static mut unsupp_cong_op: tcp_congestion_ops = tcp_congestion_ops {
    get_info: unsupp_cong_op_get_info as *mut core::ffi::c_void,
    name: [
        b'b' as core::ffi::c_char,
        b'p' as core::ffi::c_char,
        b'f' as core::ffi::c_char,
        b'_' as core::ffi::c_char,
        b'u' as core::ffi::c_char,
        b'n' as core::ffi::c_char,
        b's' as core::ffi::c_char,
        b'u' as core::ffi::c_char,
        b'p' as core::ffi::c_char,
        b'p' as core::ffi::c_char,
        b'_' as core::ffi::c_char,
        b'o' as core::ffi::c_char,
        b'p' as core::ffi::c_char,
        0,
        0,
        0,
    ],
};
