// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */
/* Rust translation of testing/selftests/bpf/progs/cgroup_tcp_skb.c.
 *
 * C header dependencies:
 * linux/bpf.h, bpf/bpf_endian.h, bpf/bpf_helpers.h,
 * linux/if_ether.h, linux/in.h, linux/in6.h, linux/ipv6.h, linux/tcp.h,
 * sys/types.h, sys/socket.h, and "cgroup_tcp_skb.h".
 */

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut g_sock_port: u16 = 0;
#[unsafe(no_mangle)]
pub static mut g_sock_state: u32 = 0;
#[unsafe(no_mangle)]
pub static mut g_unexpected: i32 = 0;
#[unsafe(no_mangle)]
pub static mut g_packet_count: u32 = 0;

unsafe extern "C" {
    fn bpf_htons(x: u16) -> u16;
    fn bpf_skb_load_bytes(
        skb: *mut __sk_buff,
        offset: u32,
        to: *mut core::ffi::c_void,
        len: u32,
    ) -> i64;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn needed_tcp_pkt(skb: *mut __sk_buff, tcph: *mut tcphdr) -> i32 {
    let mut ip6h: ipv6hdr = core::mem::zeroed();

    if (*skb).protocol != bpf_htons(ETH_P_IPV6 as u16) {
        return 0;
    }
    if bpf_skb_load_bytes(
        skb,
        0,
        &mut ip6h as *mut ipv6hdr as *mut core::ffi::c_void,
        core::mem::size_of::<ipv6hdr>() as u32,
    ) != 0
    {
        return 0;
    }

    if ip6h.nexthdr != IPPROTO_TCP {
        return 0;
    }

    if bpf_skb_load_bytes(
        skb,
        core::mem::size_of::<ipv6hdr>() as u32,
        tcph as *mut core::ffi::c_void,
        core::mem::size_of::<tcphdr>() as u32,
    ) != 0
    {
        return 0;
    }

    if (*tcph).source != bpf_htons(g_sock_port) && (*tcph).dest != bpf_htons(g_sock_port) {
        return 0;
    }

    1
}

/* Run accept() on a socket in the cgroup to receive a new connection. */
unsafe fn egress_accept(tcph: *mut tcphdr) -> i32 {
    if g_sock_state == SYN_RECV_SENDING_SYN_ACK {
        if (*tcph).fin != 0 || (*tcph).syn == 0 || (*tcph).ack == 0 {
            g_unexpected += 1;
        } else {
            g_sock_state = SYN_RECV;
        }
        return 1;
    }

    0
}

unsafe fn ingress_accept(tcph: *mut tcphdr) -> i32 {
    match g_sock_state {
        INIT => {
            if (*tcph).syn == 0 || (*tcph).fin != 0 || (*tcph).ack != 0 {
                g_unexpected += 1;
            } else {
                g_sock_state = SYN_RECV_SENDING_SYN_ACK;
            }
        }
        SYN_RECV => {
            if (*tcph).fin != 0 || (*tcph).syn != 0 || (*tcph).ack == 0 {
                g_unexpected += 1;
            } else {
                g_sock_state = ESTABLISHED;
            }
        }
        _ => return 0,
    }

    1
}

/* Run connect() on a socket in the cgroup to start a new connection. */
unsafe fn egress_connect(tcph: *mut tcphdr) -> i32 {
    if g_sock_state == INIT {
        if (*tcph).syn == 0 || (*tcph).fin != 0 || (*tcph).ack != 0 {
            g_unexpected += 1;
        } else {
            g_sock_state = SYN_SENT;
        }
        return 1;
    }

    0
}

unsafe fn ingress_connect(tcph: *mut tcphdr) -> i32 {
    if g_sock_state == SYN_SENT {
        if (*tcph).fin != 0 || (*tcph).syn == 0 || (*tcph).ack == 0 {
            g_unexpected += 1;
        } else {
            g_sock_state = ESTABLISHED;
        }
        return 1;
    }

    0
}

/* The connection is closed by the peer outside the cgroup. */
unsafe fn egress_close_remote(tcph: *mut tcphdr) -> i32 {
    match g_sock_state {
        ESTABLISHED => {}
        CLOSE_WAIT_SENDING_ACK => {
            if (*tcph).fin != 0 || (*tcph).syn != 0 || (*tcph).ack == 0 {
                g_unexpected += 1;
            } else {
                g_sock_state = CLOSE_WAIT;
            }
        }
        CLOSE_WAIT => {
            if (*tcph).fin == 0 {
                g_unexpected += 1;
            } else {
                g_sock_state = LAST_ACK;
            }
        }
        _ => return 0,
    }

    1
}

unsafe fn ingress_close_remote(tcph: *mut tcphdr) -> i32 {
    match g_sock_state {
        ESTABLISHED => {
            if (*tcph).fin != 0 {
                g_sock_state = CLOSE_WAIT_SENDING_ACK;
            }
        }
        LAST_ACK => {
            if (*tcph).fin != 0 || (*tcph).syn != 0 || (*tcph).ack == 0 {
                g_unexpected += 1;
            } else {
                g_sock_state = CLOSED;
            }
        }
        _ => return 0,
    }

    1
}

/* The connection is closed by the endpoint inside the cgroup. */
unsafe fn egress_close_local(tcph: *mut tcphdr) -> i32 {
    match g_sock_state {
        ESTABLISHED => {
            if (*tcph).fin != 0 {
                g_sock_state = FIN_WAIT1;
            }
        }
        TIME_WAIT_SENDING_ACK => {
            if (*tcph).fin != 0 || (*tcph).syn != 0 || (*tcph).ack == 0 {
                g_unexpected += 1;
            } else {
                g_sock_state = TIME_WAIT;
            }
        }
        _ => return 0,
    }

    1
}

unsafe fn ingress_close_local(tcph: *mut tcphdr) -> i32 {
    match g_sock_state {
        ESTABLISHED => {}
        FIN_WAIT1 => {
            if (*tcph).fin != 0 || (*tcph).syn != 0 || (*tcph).ack == 0 {
                g_unexpected += 1;
            } else {
                g_sock_state = FIN_WAIT2;
            }
        }
        FIN_WAIT2 => {
            if (*tcph).fin == 0 || (*tcph).syn != 0 || (*tcph).ack == 0 {
                g_unexpected += 1;
            } else {
                g_sock_state = TIME_WAIT_SENDING_ACK;
            }
        }
        _ => return 0,
    }

    1
}

/* Check the types of outgoing packets of a server socket to make sure they
 * are consistent with the state of the server socket.
 *
 * The connection is closed by the client side.
 */
#[unsafe(link_section = "cgroup_skb/egress")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn server_egress(skb: *mut __sk_buff) -> i32 {
    let mut tcph: tcphdr = core::mem::zeroed();

    if needed_tcp_pkt(skb, &mut tcph) == 0 {
        return 1;
    }

    g_packet_count += 1;

    /* Egress of the server socket. */
    if egress_accept(&mut tcph) != 0 || egress_close_remote(&mut tcph) != 0 {
        return 1;
    }

    g_unexpected += 1;
    1
}

/* Check the types of incoming packets of a server socket to make sure they
 * are consistent with the state of the server socket.
 *
 * The connection is closed by the client side.
 */
#[unsafe(link_section = "cgroup_skb/ingress")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn server_ingress(skb: *mut __sk_buff) -> i32 {
    let mut tcph: tcphdr = core::mem::zeroed();

    if needed_tcp_pkt(skb, &mut tcph) == 0 {
        return 1;
    }

    g_packet_count += 1;

    /* Ingress of the server socket. */
    if ingress_accept(&mut tcph) != 0 || ingress_close_remote(&mut tcph) != 0 {
        return 1;
    }

    g_unexpected += 1;
    1
}

/* Check the types of outgoing packets of a server socket to make sure they
 * are consistent with the state of the server socket.
 *
 * The connection is closed by the server side.
 */
#[unsafe(link_section = "cgroup_skb/egress")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn server_egress_srv(skb: *mut __sk_buff) -> i32 {
    let mut tcph: tcphdr = core::mem::zeroed();

    if needed_tcp_pkt(skb, &mut tcph) == 0 {
        return 1;
    }

    g_packet_count += 1;

    /* Egress of the server socket. */
    if egress_accept(&mut tcph) != 0 || egress_close_local(&mut tcph) != 0 {
        return 1;
    }

    g_unexpected += 1;
    1
}

/* Check the types of incoming packets of a server socket to make sure they
 * are consistent with the state of the server socket.
 *
 * The connection is closed by the server side.
 */
#[unsafe(link_section = "cgroup_skb/ingress")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn server_ingress_srv(skb: *mut __sk_buff) -> i32 {
    let mut tcph: tcphdr = core::mem::zeroed();

    if needed_tcp_pkt(skb, &mut tcph) == 0 {
        return 1;
    }

    g_packet_count += 1;

    /* Ingress of the server socket. */
    if ingress_accept(&mut tcph) != 0 || ingress_close_local(&mut tcph) != 0 {
        return 1;
    }

    g_unexpected += 1;
    1
}

/* Check the types of outgoing packets of a client socket to make sure they
 * are consistent with the state of the client socket.
 *
 * The connection is closed by the server side.
 */
#[unsafe(link_section = "cgroup_skb/egress")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn client_egress_srv(skb: *mut __sk_buff) -> i32 {
    let mut tcph: tcphdr = core::mem::zeroed();

    if needed_tcp_pkt(skb, &mut tcph) == 0 {
        return 1;
    }

    g_packet_count += 1;

    /* Egress of the server socket. */
    if egress_connect(&mut tcph) != 0 || egress_close_remote(&mut tcph) != 0 {
        return 1;
    }

    g_unexpected += 1;
    1
}

/* Check the types of incoming packets of a client socket to make sure they
 * are consistent with the state of the client socket.
 *
 * The connection is closed by the server side.
 */
#[unsafe(link_section = "cgroup_skb/ingress")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn client_ingress_srv(skb: *mut __sk_buff) -> i32 {
    let mut tcph: tcphdr = core::mem::zeroed();

    if needed_tcp_pkt(skb, &mut tcph) == 0 {
        return 1;
    }

    g_packet_count += 1;

    /* Ingress of the server socket. */
    if ingress_connect(&mut tcph) != 0 || ingress_close_remote(&mut tcph) != 0 {
        return 1;
    }

    g_unexpected += 1;
    1
}

/* Check the types of outgoing packets of a client socket to make sure they
 * are consistent with the state of the client socket.
 *
 * The connection is closed by the client side.
 */
#[unsafe(link_section = "cgroup_skb/egress")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn client_egress(skb: *mut __sk_buff) -> i32 {
    let mut tcph: tcphdr = core::mem::zeroed();

    if needed_tcp_pkt(skb, &mut tcph) == 0 {
        return 1;
    }

    g_packet_count += 1;

    /* Egress of the server socket. */
    if egress_connect(&mut tcph) != 0 || egress_close_local(&mut tcph) != 0 {
        return 1;
    }

    g_unexpected += 1;
    1
}

/* Check the types of incoming packets of a client socket to make sure they
 * are consistent with the state of the client socket.
 *
 * The connection is closed by the client side.
 */
#[unsafe(link_section = "cgroup_skb/ingress")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn client_ingress(skb: *mut __sk_buff) -> i32 {
    let mut tcph: tcphdr = core::mem::zeroed();

    if needed_tcp_pkt(skb, &mut tcph) == 0 {
        return 1;
    }

    g_packet_count += 1;

    /* Ingress of the server socket. */
    if ingress_connect(&mut tcph) != 0 || ingress_close_local(&mut tcph) != 0 {
        return 1;
    }

    g_unexpected += 1;
    1
}
