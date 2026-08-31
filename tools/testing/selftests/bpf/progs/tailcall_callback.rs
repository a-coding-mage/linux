// SPDX-License-Identifier: GPL-2.0
// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>, "bpf_misc.h",
// "bpf_test_utils.h"

pub const BPF_MAP_TYPE_PROG_ARRAY: u32 = 3;

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct JmpTable {
    // __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
    pub type_: u32,
    // __uint(max_entries, 1);
    pub max_entries: u32,
    // __uint(key_size, sizeof(__u32));
    pub key_size: u32,
    // __array(values, void (void));
    pub values: [*mut core::ffi::c_void; 1],
}

extern "C" {
    fn bpf_tail_call_static(
        ctx: *mut __sk_buff,
        map: *mut JmpTable,
        index: i32,
    );
    fn bpf_loop(
        nr_loops: u32,
        callback_fn: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> i32,
        callback_ctx: *mut core::ffi::c_void,
        flags: u64,
    ) -> i64;
    fn clobber_regs_stack();
}

#[inline(always)]
unsafe fn barrier_var<T>(value: T) -> T {
    core::ptr::read_volatile(&value)
}

#[inline(always)]
unsafe fn __sink<T>(value: T) {
    core::ptr::read_volatile(&value);
}

// SEC(".maps")
#[no_mangle]
pub static mut jmp_table: JmpTable = JmpTable {
    type_: BPF_MAP_TYPE_PROG_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<u32>() as u32,
    values: [classifier_0 as *mut core::ffi::c_void],
};

// __auxiliary
// SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn classifier_0(_skb: *mut __sk_buff) -> i32 {
    0
}

// static __noinline
#[inline(never)]
unsafe fn subprog_tail0(skb: *mut __sk_buff) -> i32 {
    let ret: i32 = 0;

    bpf_tail_call_static(skb, &mut jmp_table, 0);
    barrier_var(ret);
    ret
}

// static __noinline
#[inline(never)]
unsafe extern "C" fn callback_loop(
    _index: i32,
    cb_ctx: *mut core::ffi::c_void,
) -> i32 {
    let mut ret: i32;

    ret = subprog_tail0(*(cb_ctx as *mut *mut __sk_buff));
    ret = barrier_var(ret);
    if ret != 0 { 1 } else { 0 }
}

// static __noinline
#[inline(never)]
unsafe extern "C" fn callback_empty(
    _index: i32,
    _data: *mut core::ffi::c_void,
) -> i32 {
    0
}

/* callback involving subprog with tail call is rejected */
// SEC("tc")
// __failure __msg("cannot tail call within callback")
#[no_mangle]
pub unsafe extern "C" fn tailcall_callback_1(skb: *mut __sk_buff) -> i32 {
    clobber_regs_stack();

    let mut skb = skb;
    bpf_loop(
        1,
        callback_loop,
        &mut skb as *mut *mut __sk_buff as *mut core::ffi::c_void,
        0,
    );
    0
}

/* subprogs with tailcall do not affect no-tailcall callback */
// SEC("tc")
// __success
// __retval(0)
#[no_mangle]
pub unsafe extern "C" fn tailcall_callback_2(skb: *mut __sk_buff) -> i32 {
    let ret: i32;

    clobber_regs_stack();

    ret = subprog_tail0(skb);
    __sink(ret);

    bpf_loop(1, callback_empty, core::ptr::null_mut(), 0);
    0
}

// SEC("license")
#[no_mangle]
pub static __license: [u8; 4] = *b"GPL\0";
