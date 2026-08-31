// SPDX-License-Identifier: GPL-2.0

// C dependencies: <string.h>, <linux/stddef.h>, <linux/bpf.h>,
// <sys/socket.h>, <bpf/bpf_helpers.h>, <bpf/bpf_endian.h>

const VERDICT_REJECT: i32 = 0;
const VERDICT_PROCEED: i32 = 1;

extern "C" {
    static SOCK_STREAM: i32;

    fn bpf_htons(x: i32) -> u16;
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
}

#[no_mangle]
pub static mut port: i32 = 0;

#[no_mangle]
#[link_section = "cgroup/connect4"]
pub unsafe extern "C" fn connect_v4_dropper(ctx: *mut bpf_sock_addr) -> i32 {
    if (*ctx).type_ != SOCK_STREAM as u32 {
        return VERDICT_PROCEED;
    }
    if (*ctx).user_port == bpf_htons(port) as u32 {
        return VERDICT_REJECT;
    }
    VERDICT_PROCEED
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
