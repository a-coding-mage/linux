// SPDX-License-Identifier: GPL-2.0
// C dependencies: "vmlinux.h" and <bpf/bpf_helpers.h>

// Original BPF map declaration:
// struct {
//     __uint(type, BPF_MAP_TYPE_ARRAY);
//     __uint(max_entries, 3);
//     __type(key, __u32);
//     __type(value, __u64);
// } arraymap SEC(".maps");
//
// Original BPF map declaration:
// struct {
//     __uint(type, BPF_MAP_TYPE_HASH);
//     __uint(max_entries, 5);
//     __type(key, __u32);
//     __type(value, __u64);
// } hashmap SEC(".maps");

extern "C" {
    static mut arraymap: bpf_map;
    static mut hashmap: bpf_map;

    fn bpf_for_each_map_elem(
        map: *mut bpf_map,
        callback: Option<
            unsafe extern "C" fn(
                map: *mut bpf_map,
                key: *mut u32,
                val: *mut u64,
                data: *mut callback_ctx,
            ) -> u64,
        >,
        callback_ctx: *mut callback_ctx,
        flags: u64,
    ) -> i64;
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
struct callback_ctx {
    output: i32,
}

#[no_mangle]
pub static mut data_output: u32 = 0;

#[no_mangle]
pub static mut use_array: i32 = 0;

unsafe extern "C" fn check_map_elem(
    _map: *mut bpf_map,
    _key: *mut u32,
    val: *mut u64,
    data: *mut callback_ctx,
) -> u64 {
    (*data).output += *val as i32;
    0
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn test_pkt_access(_skb: *mut __sk_buff) -> i32 {
    let mut data: callback_ctx = callback_ctx { output: 0 };

    data.output = 0;
    if use_array != 0 {
        bpf_for_each_map_elem(
            &mut arraymap,
            Some(check_map_elem),
            &mut data as *mut callback_ctx,
            0,
        );
    } else {
        bpf_for_each_map_elem(
            &mut hashmap,
            Some(check_map_elem),
            &mut data as *mut callback_ctx,
            0,
        );
    }
    data_output = data.output as u32;

    0
}
