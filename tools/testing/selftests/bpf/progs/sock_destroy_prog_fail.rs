// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_tracing.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

extern "C" {
    pub fn bpf_sock_destroy(sk: *mut sock_common) -> i32;
}

// SEC("tp_btf/tcp_destroy_sock")
// __failure __msg("calling kernel function bpf_sock_destroy is not allowed")
#[no_mangle]
#[link_section = "tp_btf/tcp_destroy_sock"]
pub unsafe extern "C" fn trace_tcp_destroy_sock(sk: *mut sock) -> i32 {
    /* should not load */
    bpf_sock_destroy(sk as *mut sock_common);

    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
