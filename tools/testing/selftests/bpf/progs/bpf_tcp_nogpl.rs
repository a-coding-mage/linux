// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include "bpf_tracing_net.h"
// #include <bpf/bpf_tracing.h>

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [::core::ffi::c_char; 2] = [b'X' as ::core::ffi::c_char, 0];

unsafe extern "C" {
    pub type sock;
    pub type tcp_congestion_ops;
}

// SEC("struct_ops")
// void BPF_PROG(nogpltcp_init, struct sock *sk)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nogpltcp_init(_sk: *mut sock) {}

// SEC(".struct_ops")
// struct tcp_congestion_ops bpf_nogpltcp = {
//     .init           = (void *)nogpltcp_init,
//     .name           = "bpf_nogpltcp",
// };
#[repr(C)]
pub struct tcp_congestion_ops__local {
    pub init: *mut ::core::ffi::c_void,
    pub name: [::core::ffi::c_char; 13],
}

#[unsafe(link_section = ".struct_ops")]
#[unsafe(no_mangle)]
pub static mut bpf_nogpltcp: tcp_congestion_ops__local = tcp_congestion_ops__local {
    init: nogpltcp_init as *mut ::core::ffi::c_void,
    name: [
        b'b' as ::core::ffi::c_char,
        b'p' as ::core::ffi::c_char,
        b'f' as ::core::ffi::c_char,
        b'_' as ::core::ffi::c_char,
        b'n' as ::core::ffi::c_char,
        b'o' as ::core::ffi::c_char,
        b'g' as ::core::ffi::c_char,
        b'p' as ::core::ffi::c_char,
        b'l' as ::core::ffi::c_char,
        b't' as ::core::ffi::c_char,
        b'c' as ::core::ffi::c_char,
        b'p' as ::core::ffi::c_char,
        0,
    ],
};
