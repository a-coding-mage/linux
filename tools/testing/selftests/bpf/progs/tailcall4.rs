// SPDX-License-Identifier: GPL-2.0
// C source dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>

extern "C" {
    fn bpf_tail_call(ctx: *mut __sk_buff, prog_array_map: *mut core::ffi::c_void, index: __u32);
}

type __u32 = u32;

#[repr(C)]
pub struct __sk_buff {
    _unused: [u8; 0],
}

// BPF map definition equivalent to:
// struct {
//     __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
//     __uint(max_entries, 3);
//     __uint(key_size, sizeof(__u32));
//     __uint(value_size, sizeof(__u32));
// } jmp_table SEC(".maps");
#[repr(C)]
pub struct jmp_table_map {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
}

pub const BPF_MAP_TYPE_PROG_ARRAY: __u32 = 3;

#[no_mangle]
#[link_section = ".maps"]
pub static mut jmp_table: jmp_table_map = jmp_table_map {
    type_: BPF_MAP_TYPE_PROG_ARRAY,
    max_entries: 3,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u32>() as __u32,
};

#[no_mangle]
pub static mut selector: i32 = 0;

// #define TAIL_FUNC(x) SEC("tc") int classifier_##x(struct __sk_buff *skb) { return x; }
#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn classifier_0(_skb: *mut __sk_buff) -> i32 {
    return 0;
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn classifier_1(_skb: *mut __sk_buff) -> i32 {
    return 1;
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn classifier_2(_skb: *mut __sk_buff) -> i32 {
    return 2;
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn entry(skb: *mut __sk_buff) -> i32 {
    bpf_tail_call(
        skb,
        core::ptr::addr_of_mut!(jmp_table) as *mut core::ffi::c_void,
        selector as __u32,
    );
    return 3;
}

#[no_mangle]
#[link_section = "license"]
pub static __license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
