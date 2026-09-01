// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_compiler.h"

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn process(skb: *mut __sk_buff) -> i32 {
    // __pragma_loop_unroll_full
    for i in 0usize..5usize {
        if (*skb).cb[i] != (i as u32).wrapping_add(1) {
            return 1;
        }
        (*skb).cb[i] = (*skb).cb[i].wrapping_add(1);
    }
    (*skb).priority = (*skb).priority.wrapping_add(1);
    (*skb).tstamp = (*skb).tstamp.wrapping_add(1);
    (*skb).mark = (*skb).mark.wrapping_add(1);

    if (*skb).wire_len != 100 {
        return 1;
    }
    if (*skb).gso_segs != 8 {
        return 1;
    }
    if (*skb).gso_size != 10 {
        return 1;
    }
    if (*skb).ingress_ifindex != 11 {
        return 1;
    }
    if (*skb).ifindex != 1 {
        return 1;
    }
    if (*skb).hwtstamp != 11 {
        return 1;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
