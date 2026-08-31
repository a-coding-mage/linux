// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
#[link_section = "lsm_cgroup/inet_csk_clone"]
pub unsafe extern "C" fn nonvoid_socket_clone(
    newsk: *mut sock,
    req: *const request_sock,
) -> i32 {
    let _ = newsk;
    let _ = req;

    /* Can not return any errors from void LSM hooks. */
    return 0;
}
