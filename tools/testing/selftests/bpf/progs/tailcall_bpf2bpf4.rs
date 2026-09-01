// SPDX-License-Identifier: GPL-2.0
// Original C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

pub type __u32 = u32;

pub const BPF_MAP_TYPE_ARRAY: __u32 = 2;
pub const BPF_MAP_TYPE_PROG_ARRAY: __u32 = 3;

#[repr(C)]
pub struct __sk_buff {
    pub len: __u32,
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
}

unsafe extern "C" {
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void)
        -> *mut core::ffi::c_void;
    fn bpf_tail_call_static(ctx: *mut __sk_buff, map: *mut core::ffi::c_void, index: __u32);
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut nop_table: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u32>() as __u32,
};

#[no_mangle]
#[link_section = ".maps"]
pub static mut jmp_table: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PROG_ARRAY,
    max_entries: 3,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u32>() as __u32,
};

#[no_mangle]
pub static mut count: i32 = 0;

#[no_mangle]
pub static mut noise: i32 = 0;

#[inline(always)]
unsafe fn subprog_noise() -> i32 {
    let key: __u32 = 0;

    unsafe {
        bpf_map_lookup_elem(
            core::ptr::addr_of_mut!(nop_table).cast::<core::ffi::c_void>(),
            core::ptr::addr_of!(key).cast::<core::ffi::c_void>(),
        );
    }
    0
}

#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn subprog_tail_2(skb: *mut __sk_buff) -> i32 {
    unsafe {
        if noise != 0 {
            subprog_noise();
        }
        bpf_tail_call_static(
            skb,
            core::ptr::addr_of_mut!(jmp_table).cast::<core::ffi::c_void>(),
            2,
        );
        ((*skb).len).wrapping_mul(3) as i32
    }
}

#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn subprog_tail_1(skb: *mut __sk_buff) -> i32 {
    unsafe {
        bpf_tail_call_static(
            skb,
            core::ptr::addr_of_mut!(jmp_table).cast::<core::ffi::c_void>(),
            1,
        );
        ((*skb).len).wrapping_mul(2) as i32
    }
}

#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn subprog_tail(skb: *mut __sk_buff) -> i32 {
    unsafe {
        bpf_tail_call_static(
            skb,
            core::ptr::addr_of_mut!(jmp_table).cast::<core::ffi::c_void>(),
            0,
        );
        (*skb).len as i32
    }
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn classifier_1(skb: *mut __sk_buff) -> i32 {
    unsafe { subprog_tail_2(skb) }
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn classifier_2(skb: *mut __sk_buff) -> i32 {
    unsafe {
        count = count.wrapping_add(1);
        subprog_tail_2(skb)
    }
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn classifier_0(skb: *mut __sk_buff) -> i32 {
    unsafe { subprog_tail_1(skb) }
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn entry(skb: *mut __sk_buff) -> i32 {
    unsafe { subprog_tail(skb) }
}

#[no_mangle]
#[link_section = "license"]
pub static __license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
