// SPDX-License-Identifier: GPL-2.0
// C source included "vmlinux.h" and <bpf/bpf_helpers.h>; the BPF helper
// macros, section annotations, and kernel types are expected from the build
// environment that consumes this translated source.

#[repr(C)]
pub struct sock_map {
    // __uint(type, BPF_MAP_TYPE_SOCKMAP);
    // __uint(max_entries, 2);
    // __type(key, __u32);
    // __type(value, __u64);
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut sock_map: sock_map = sock_map {};

unsafe extern "C" {
    pub type __sk_buff;
}

pub const SK_DROP: i32 = 0;

#[unsafe(link_section = "sk_skb/verdict")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn prog_skb_verdict(skb: *mut __sk_buff) -> ::core::ffi::c_int {
    let _ = skb;
    return SK_DROP;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [::core::ffi::c_char; 4] = [b'G' as ::core::ffi::c_char, b'P' as ::core::ffi::c_char, b'L' as ::core::ffi::c_char, 0];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
