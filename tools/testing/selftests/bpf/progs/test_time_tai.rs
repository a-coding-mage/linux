// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2022 Linutronix GmbH */

// Dependency intent from C includes:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>

extern "C" {
    fn bpf_ktime_get_tai_ns() -> u64;
}

#[repr(C)]
pub struct __sk_buff {
    pub tstamp: u64,
    pub cb: [u32; 5],
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn time_tai(skb: *mut __sk_buff) -> i32 {
    let ts1: u64;
    let ts2: u64;

    /* Get TAI timestamps */
    ts1 = bpf_ktime_get_tai_ns();
    ts2 = bpf_ktime_get_tai_ns();

    /* Save TAI timestamps (Note: skb->hwtstamp is read-only) */
    (*skb).tstamp = ts1;
    (*skb).cb[0] = (ts2 & 0xffffffff) as u32;
    (*skb).cb[1] = (ts2 >> 32) as u32;

    return 0;
}
