use core::ffi::c_int;
use core::ptr::null_mut;

// C dependencies translated from:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_endian.h>
//
// The following names are supplied by those dependencies in the original C
// source: BPF_MAP_TYPE_SOCKMAP, SK_DROP, and struct __sk_buff.

#[repr(C)]
pub struct sock_map_rx {
    pub r#type: *mut [c_int; BPF_MAP_TYPE_SOCKMAP as usize],
    pub max_entries: *mut [c_int; 20],
    pub key: *mut c_int,
    pub value: *mut c_int,
}

#[link_section = ".maps"]
pub static mut sock_map_rx: sock_map_rx = sock_map_rx {
    r#type: null_mut(),
    max_entries: null_mut(),
    key: null_mut(),
    value: null_mut(),
};

#[repr(C)]
pub struct sock_map_tx {
    pub r#type: *mut [c_int; BPF_MAP_TYPE_SOCKMAP as usize],
    pub max_entries: *mut [c_int; 20],
    pub key: *mut c_int,
    pub value: *mut c_int,
}

#[link_section = ".maps"]
pub static mut sock_map_tx: sock_map_tx = sock_map_tx {
    r#type: null_mut(),
    max_entries: null_mut(),
    key: null_mut(),
    value: null_mut(),
};

#[repr(C)]
pub struct sock_map_msg {
    pub r#type: *mut [c_int; BPF_MAP_TYPE_SOCKMAP as usize],
    pub max_entries: *mut [c_int; 20],
    pub key: *mut c_int,
    pub value: *mut c_int,
}

#[link_section = ".maps"]
pub static mut sock_map_msg: sock_map_msg = sock_map_msg {
    r#type: null_mut(),
    max_entries: null_mut(),
    key: null_mut(),
    value: null_mut(),
};

#[link_section = "sk_skb"]
pub unsafe extern "C" fn prog_skb_verdict(skb: *mut __sk_buff) -> c_int {
    SK_DROP
}

#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";
