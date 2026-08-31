// SPDX-License-Identifier: GPL-2.0
// C dependencies: "vmlinux.h", <bpf/bpf_helpers.h>

extern "C" {
    fn bpf_redirect(ifindex: i32, flags: u64) -> i32;
}

#[repr(C)]
pub struct __sk_buff {
    _unused: [u8; 0],
}

const TCX_REDIRECT: i32 = 7;

#[no_mangle]
pub static mut redirect_ifindex: i32 = 1;

#[no_mangle]
pub static mut verdict_calls: u64 = 0;

#[no_mangle]
pub static mut helper_calls: u64 = 0;

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn qevent_redirect_verdict(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    unsafe {
        core::intrinsics::atomic_xadd_seqcst(&mut verdict_calls, 1);
    }
    TCX_REDIRECT
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn qevent_redirect_helper(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    unsafe {
        core::intrinsics::atomic_xadd_seqcst(&mut helper_calls, 1);
        bpf_redirect(redirect_ifindex, 0)
    }
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
