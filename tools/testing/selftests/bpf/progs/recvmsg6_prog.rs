// SPDX-License-Identifier: GPL-2.0

// C dependencies: linux/stddef.h, linux/bpf.h, linux/in6.h, sys/socket.h,
// bpf/bpf_helpers.h, bpf/bpf_endian.h, bpf_sockopt_helpers.h.

use core::ffi::c_int;

const SERV6_IP_0: u32 = 0xfaceb00c; /* face:b00c:1234:5678::abcd */
const SERV6_IP_1: u32 = 0x12345678;
const SERV6_IP_2: u32 = 0x00000000;
const SERV6_IP_3: u32 = 0x0000abcd;
const SERV6_PORT: u16 = 6060;

#[no_mangle]
#[link_section = "cgroup/recvmsg6"]
pub unsafe extern "C" fn recvmsg6_prog(ctx: *mut bpf_sock_addr) -> c_int {
    let sk: *mut bpf_sock;

    sk = (*ctx).sk;
    if sk.is_null() {
        return 1;
    }

    if (*sk).family != AF_INET6 {
        return 1;
    }

    if (*ctx).type_ != SOCK_STREAM && (*ctx).type_ != SOCK_DGRAM {
        return 1;
    }

    if !get_set_sk_priority(ctx) {
        return 1;
    }

    (*ctx).user_ip6[0] = bpf_htonl(SERV6_IP_0);
    (*ctx).user_ip6[1] = bpf_htonl(SERV6_IP_1);
    (*ctx).user_ip6[2] = bpf_htonl(SERV6_IP_2);
    (*ctx).user_ip6[3] = bpf_htonl(SERV6_IP_3);
    (*ctx).user_port = bpf_htons(SERV6_PORT);

    1
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
