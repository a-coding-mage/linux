// SPDX-License-Identifier: GPL-2.0
// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>

extern "C" {
    pub type __sk_buff;
}

#[no_mangle]
#[link_section = "freplace/foo"]
pub unsafe extern "C" fn test_freplace_void(skb: *mut __sk_buff) {
    let _ = skb;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";
