// SPDX-License-Identifier: GPL-2.0

// C dependencies: "vmlinux.h", <bpf/bpf_helpers.h>, "bpf_misc.h"

extern "C" {
    fn bpf_check_mtu(
        ctx: *mut __sk_buff,
        ifindex: __u32,
        mtu_len: *mut __u32,
        len_diff: __s32,
        flags: __u64,
    ) -> __s64;
}

type __u32 = u32;
type __s32 = i32;
type __u64 = u64;
type __s64 = i64;

#[repr(C)]
pub struct __sk_buff {
    _unused: [u8; 0],
}

const CAP_BPF: u64 = 39;
const CAP_NET_ADMIN: u64 = 12;
const TCX_PASS: i32 = 0;

// SEC("tc/ingress")
// __description("uninit/mtu: write rejected")
// __success
// __caps_unpriv(CAP_BPF|CAP_NET_ADMIN)
// __failure_unpriv __msg_unpriv("invalid read from stack")
#[no_mangle]
pub unsafe extern "C" fn tc_uninit_mtu(ctx: *mut __sk_buff) -> i32 {
    let mut mtu: __u32;

    bpf_check_mtu(ctx, 0, &mut mtu, 0, 0);
    TCX_PASS
}

#[no_mangle]
#[link_section = "license"]
pub static mut LICENSE: [u8; 4] = *b"GPL\0";
