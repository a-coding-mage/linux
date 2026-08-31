// SPDX-License-Identifier: GPL-2.0
// Depends on Linux BPF bindings for `__sk_buff` and section placement
// equivalent to the C `SEC(...)` macro from bpf_helpers.h.

#[unsafe(no_mangle)]
#[unsafe(link_section = "tc")]
pub unsafe extern "C" fn entry(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    return 1;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static __license: [u8; 4] = *b"GPL\0";
