// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// - <linux/stddef.h>
// - <linux/bpf.h>
// - <linux/in.h>
// - <sys/socket.h>
// - <bpf/bpf_helpers.h>
// - <bpf/bpf_endian.h>
// - <bpf_sockopt_helpers.h>

const SERV4_IP: u32 = 0xc0a801fe; /* 192.168.1.254 */
const SERV4_PORT: u16 = 4040;

const AF_INET: u32 = 2;
const SOCK_STREAM: u32 = 1;
const SOCK_DGRAM: u32 = 2;

#[repr(C)]
pub struct bpf_sock {
    pub family: u32,
}

#[repr(C)]
pub struct bpf_sock_addr {
    pub user_family: u32,
    pub user_ip4: u32,
    pub user_ip6: [u32; 4],
    pub user_port: u32,
    pub family: u32,
    pub type_: u32,
    pub protocol: u32,
    pub msg_src_ip4: u32,
    pub msg_src_ip6: [u32; 4],
    pub sk: *mut bpf_sock,
}

unsafe extern "C" {
    fn get_set_sk_priority(ctx: *mut bpf_sock_addr) -> i32;
}

#[inline]
fn bpf_htonl(x: u32) -> u32 {
    x.to_be()
}

#[inline]
fn bpf_htons(x: u16) -> u16 {
    x.to_be()
}

#[unsafe(link_section = "cgroup/recvmsg4")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn recvmsg4_prog(ctx: *mut bpf_sock_addr) -> i32 {
    let sk: *mut bpf_sock;

    sk = unsafe { (*ctx).sk };
    if sk.is_null() {
        return 1;
    }

    if unsafe { (*sk).family } != AF_INET {
        return 1;
    }

    if unsafe { (*ctx).type_ } != SOCK_STREAM && unsafe { (*ctx).type_ } != SOCK_DGRAM {
        return 1;
    }

    if unsafe { get_set_sk_priority(ctx) } == 0 {
        return 1;
    }

    unsafe {
        (*ctx).user_ip4 = bpf_htonl(SERV4_IP);
        (*ctx).user_port = bpf_htons(SERV4_PORT) as u32;
    }

    1
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";
