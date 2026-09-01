// SPDX-License-Identifier: GPL-2.0
// C dependencies: <linux/bpf.h>, <linux/pkt_cls.h>, <bpf/bpf_helpers.h>

#[repr(C)]
pub struct __sk_buff {
    _unused: [u8; 0],
}

#[unsafe(link_section = "freplace/global_func2")]
pub extern "C" fn test_freplace_int_with_void(_skb: *mut __sk_buff) {}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
