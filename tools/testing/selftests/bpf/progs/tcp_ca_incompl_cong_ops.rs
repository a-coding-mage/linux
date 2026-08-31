// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include "bpf_tracing_net.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [core::ffi::c_char; 4] = [
    b'G' as core::ffi::c_char,
    b'P' as core::ffi::c_char,
    b'L' as core::ffi::c_char,
    0,
];

unsafe extern "C" {
    fn tcp_sk(sk: *mut sock) -> *mut tcp_sock;
}

#[unsafe(link_section = "struct_ops")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn incompl_cong_ops_ssthresh(sk: *mut sock) -> __u32 {
    unsafe { (*tcp_sk(sk)).snd_ssthresh }
}

#[unsafe(link_section = "struct_ops")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn incompl_cong_ops_undo_cwnd(sk: *mut sock) -> __u32 {
    unsafe { (*tcp_sk(sk)).snd_cwnd }
}

#[unsafe(link_section = ".struct_ops")]
#[unsafe(no_mangle)]
pub static mut incompl_cong_ops: tcp_congestion_ops = tcp_congestion_ops {
    /* Intentionally leaving out any of the required cong_avoid() and
     * cong_control() here.
     */
    ssthresh: Some(incompl_cong_ops_ssthresh),
    undo_cwnd: Some(incompl_cong_ops_undo_cwnd),
    name: *b"bpf_incompl_ops\0",
};
