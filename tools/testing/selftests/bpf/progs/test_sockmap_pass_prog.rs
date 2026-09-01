// Dependencies from the original C source:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_endian.h>

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub const BPF_MAP_TYPE_SOCKMAP: u32 = 15;
pub const BPF_F_INGRESS: u64 = 1;
pub const SK_PASS: i32 = 1;

#[repr(C)]
pub struct __sk_buff {
    pub len: u32,
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: u32,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
}

unsafe extern "C" {
    pub fn bpf_sk_redirect_map(
        skb: *mut __sk_buff,
        map: *mut bpf_map_def,
        key: u32,
        flags: u64,
    ) -> i32;
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut sock_map_rx: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SOCKMAP,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i32>() as u32,
    max_entries: 20,
};

#[no_mangle]
#[link_section = ".maps"]
pub static mut sock_map_tx: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SOCKMAP,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i32>() as u32,
    max_entries: 20,
};

#[no_mangle]
#[link_section = ".maps"]
pub static mut sock_map_msg: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SOCKMAP,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i32>() as u32,
    max_entries: 20,
};

#[no_mangle]
#[link_section = "sk_skb/stream_verdict"]
pub unsafe extern "C" fn prog_skb_verdict(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    SK_PASS
}

#[no_mangle]
pub static mut clone_called: i32 = 0;

#[no_mangle]
#[link_section = "sk_skb/stream_verdict"]
pub unsafe extern "C" fn prog_skb_verdict_clone(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    clone_called = 1;
    SK_PASS
}

#[no_mangle]
#[link_section = "sk_skb/stream_parser"]
pub unsafe extern "C" fn prog_skb_parser(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    SK_PASS
}

#[no_mangle]
#[link_section = "sk_skb/stream_verdict"]
pub unsafe extern "C" fn prog_skb_verdict_ingress(skb: *mut __sk_buff) -> i32 {
    let one: i32 = 1;

    bpf_sk_redirect_map(
        skb,
        core::ptr::addr_of_mut!(sock_map_rx),
        one as u32,
        BPF_F_INGRESS,
    )
}

#[no_mangle]
#[link_section = "sk_skb/stream_parser"]
pub unsafe extern "C" fn prog_skb_verdict_ingress_strp(skb: *mut __sk_buff) -> i32 {
    (*skb).len as i32
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
