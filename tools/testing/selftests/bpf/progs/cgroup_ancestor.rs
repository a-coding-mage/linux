// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018 Facebook

// C dependencies: <vmlinux.h>, <bpf/bpf_helpers.h>, <bpf/bpf_core_read.h>,
// "bpf_tracing_net.h".

const NUM_CGROUP_LEVELS: usize = 4;

#[no_mangle]
pub static mut cgroup_ids: [__u64; NUM_CGROUP_LEVELS] = [0; NUM_CGROUP_LEVELS];
#[no_mangle]
pub static mut dport: __u16 = 0;

#[inline(always)]
unsafe fn log_nth_level(skb: *mut __sk_buff, level: __u32) {
    /* [1] &level passed to external function that may change it, it's
     *     incompatible with loop unroll.
     */
    cgroup_ids[level as usize] = bpf_skb_ancestor_cgroup_id(skb, level);
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn log_cgroup_id(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let mut sk: *mut sock = (*skb).sk as *mut sock;

    if sk.is_null() {
        return TC_ACT_OK;
    }

    sk = bpf_core_cast::<sock>(sk);
    if (*sk).sk_protocol as ::core::ffi::c_int == IPPROTO_UDP && (*sk).sk_dport == dport {
        log_nth_level(skb, 0);
        log_nth_level(skb, 1);
        log_nth_level(skb, 2);
        log_nth_level(skb, 3);
    }

    TC_ACT_OK
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [::core::ffi::c_char; 4] = [
    b'G' as ::core::ffi::c_char,
    b'P' as ::core::ffi::c_char,
    b'L' as ::core::ffi::c_char,
    0,
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
