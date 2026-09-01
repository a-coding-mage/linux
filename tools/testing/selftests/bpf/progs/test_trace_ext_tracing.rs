// SPDX-License-Identifier: GPL-2.0

// Rust translation of BPF source depending on external kernel/BPF bindings:
// "vmlinux.h", <bpf/bpf_helpers.h>, and <bpf/bpf_tracing.h>.

#[repr(C)]
pub struct sk_buff {
    pub len: u32,
}

#[no_mangle]
pub static mut fentry_called: u64 = 0;

#[no_mangle]
#[link_section = "fentry/test_pkt_md_access_new"]
pub unsafe extern "C" fn fentry(skb: *mut sk_buff) -> i32 {
    fentry_called = (*skb).len as u64;
    0
}

#[no_mangle]
pub static mut fexit_called: u64 = 0;

#[no_mangle]
#[link_section = "fexit/test_pkt_md_access_new"]
pub unsafe extern "C" fn fexit(skb: *mut sk_buff) -> i32 {
    fexit_called = (*skb).len as u64;
    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
