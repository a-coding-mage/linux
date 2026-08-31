// Translated from:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_endian.h>
// #include "bpf_misc.h"

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

type __u32 = u32;
type __u8 = u8;

const BPF_MAP_TYPE_SOCKMAP: u32 = 15;
const BPF_MAP_TYPE_ARRAY: u32 = 2;
const SK_DROP: i32 = 0;

#[repr(C)]
pub struct __sk_buff {
    pub data: __u32,
    pub data_end: __u32,
    _unused0: [u8; 0],
    pub local_port: __u32,
    pub remote_port: __u32,
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

// SEC(".maps")
#[no_mangle]
pub static mut sock_map_rx: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SOCKMAP,
    max_entries: 20,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i32>() as u32,
};

// SEC(".maps")
#[no_mangle]
pub static mut sock_map_tx: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SOCKMAP,
    max_entries: 20,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i32>() as u32,
};

// SEC(".maps")
#[no_mangle]
pub static mut sock_map_msg: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_SOCKMAP,
    max_entries: 20,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i32>() as u32,
};

// SEC(".maps")
#[no_mangle]
pub static mut sock_map_break: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 20,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i32>() as u32,
};

extern "C" {
    fn bpf_skb_pull_data(skb: *mut __sk_buff, len: u32) -> i64;
    fn bpf_sk_redirect_map(
        skb: *mut __sk_buff,
        map: *mut bpf_map_def,
        key: u32,
        flags: u64,
    ) -> i32;
    fn __sink(x: __u32);
}

// SEC("sk_skb2")
#[no_mangle]
pub unsafe extern "C" fn bpf_prog2(skb: *mut __sk_buff) -> i32 {
    let mut data_end: *mut core::ffi::c_void = (*skb).data_end as usize as *mut core::ffi::c_void;
    let mut data: *mut core::ffi::c_void = (*skb).data as usize as *mut core::ffi::c_void;
    let lport: __u32 = (*skb).local_port;
    let rport: __u32 = (*skb).remote_port;
    let mut d: *mut __u8 = data as *mut __u8;
    let sk: __u8;
    let map: __u8;

    __sink(lport);
    __sink(rport);

    if data.add(8) > data_end {
        if bpf_skb_pull_data(skb, 8) != 0 {
            return SK_DROP;
        }

        data = (*skb).data as usize as *mut core::ffi::c_void;
        data_end = (*skb).data_end as usize as *mut core::ffi::c_void;

        if data.add(8) > data_end {
            return SK_DROP;
        }

        d = data as *mut __u8;
    }

    map = *d.add(0);
    sk = *d.add(1);

    *d.add(0) = 0xd;
    *d.add(1) = 0xe;
    *d.add(2) = 0xa;
    *d.add(3) = 0xd;
    *d.add(4) = 0xb;
    *d.add(5) = 0xe;
    *d.add(6) = 0xe;
    *d.add(7) = 0xf;

    if map == 0 {
        return bpf_sk_redirect_map(skb, &mut sock_map_rx, sk as u32, 0);
    }
    bpf_sk_redirect_map(skb, &mut sock_map_tx, sk as u32, 0)
}

// SEC("license")
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";
