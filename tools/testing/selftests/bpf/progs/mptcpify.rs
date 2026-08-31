// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023, SUSE. */

// C dependencies in the original source:
// #include "vmlinux.h"
// #include <bpf/bpf_tracing.h>
// #include "bpf_tracing_net.h"

unsafe extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut pid: i32 = 0;

#[no_mangle]
#[link_section = "fmod_ret/update_socket_protocol"]
pub unsafe extern "C" fn mptcpify(family: i32, type_: i32, protocol: i32) -> i32 {
    if (bpf_get_current_pid_tgid() >> 32) != pid as u64 {
        return protocol;
    }

    if (family == AF_INET || family == AF_INET6)
        && (type_ & SOCK_TYPE_MASK) == SOCK_STREAM
        && (protocol == 0 || protocol == IPPROTO_TCP)
    {
        return IPPROTO_MPTCP;
    }

    protocol
}
