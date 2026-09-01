// SPDX-License-Identifier: GPL-2.0
// Dependencies from the original C source:
// <linux/bpf.h>, <bpf/bpf_helpers.h>, "bpf_misc.h", "bpf_test_utils.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub type __u32 = u32;

pub const BPF_MAP_TYPE_PROG_ARRAY: u32 = 3;

#[repr(C)]
pub struct __sk_buff {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct bpf_map_def_jmp_table {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub values: [*mut core::ffi::c_void; 1],
}

unsafe extern "C" {
    fn bpf_tail_call_static(
        skb: *mut __sk_buff,
        jmp_table: *mut core::ffi::c_void,
        index: __u32,
    );
    fn barrier_var(ret: i32);
    fn __sink(ret: i32);
    fn clobber_regs_stack();
}

unsafe extern "C" {
    pub fn classifier_0(skb: *mut __sk_buff) -> i32;
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut jmp_table0: bpf_map_def_jmp_table = bpf_map_def_jmp_table {
    type_: BPF_MAP_TYPE_PROG_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as u32,
    values: [classifier_0 as *mut core::ffi::c_void],
};

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut jmp_table1: bpf_map_def_jmp_table = bpf_map_def_jmp_table {
    type_: BPF_MAP_TYPE_PROG_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as u32,
    values: [classifier_0 as *mut core::ffi::c_void],
};

#[unsafe(no_mangle)]
pub static mut count: i32 = 0;

#[inline(never)]
unsafe fn subprog_tail(skb: *mut __sk_buff, jmp_table: *mut core::ffi::c_void) -> i32 {
    let ret: i32 = 0;

    unsafe {
        bpf_tail_call_static(skb, jmp_table, 0);
        barrier_var(ret);
    }
    ret
}

// __auxiliary
#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn classifier_0(skb: *mut __sk_buff) -> i32 {
    let ret1: i32;
    let ret2: i32;

    unsafe {
        count += 1;
        ret1 = subprog_tail(
            skb,
            &raw mut jmp_table0 as *mut core::ffi::c_void,
        );
        ret2 = subprog_tail(
            skb,
            &raw mut jmp_table1 as *mut core::ffi::c_void,
        );
        __sink(ret1);
        __sink(ret2);
        count
    }
}

// __success
// __retval(33)
#[unsafe(link_section = "tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tailcall_bpf2bpf_hierarchy_3(skb: *mut __sk_buff) -> i32 {
    let ret: i32 = 0;

    unsafe {
        clobber_regs_stack();

        bpf_tail_call_static(
            skb,
            &raw mut jmp_table0 as *mut core::ffi::c_void,
            0,
        );

        __sink(ret);
    }
    ret
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static __license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
