// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook

// Dependencies from the original C includes:
// <linux/ptrace.h>, <linux/bpf.h>, <bpf/bpf_helpers.h>, and "bpf_misc.h".
// `pt_regs` is supplied by <linux/ptrace.h> in the original program.

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct RdonlyValues {
    pub a: [u32; 4],
    /*
     * if the struct's size is multiple of 16, compiler will put it into
     * .rodata.cst16 section, which is not recognized by libbpf; work
     * around this by ensuring we don't have 16-aligned struct
     */
    pub _y: i8,
}

#[no_mangle]
pub static rdonly_values: RdonlyValues = RdonlyValues {
    a: [2, 3, 4, 5],
    _y: 0,
};

#[repr(C)]
pub struct Res {
    pub did_run: u32,
    pub iters: u32,
    pub sum: u32,
}

#[no_mangle]
pub static mut res: Res = Res {
    did_run: 0,
    iters: 0,
    sum: 0,
};

#[no_mangle]
#[link_section = "raw_tracepoint/sys_enter:skip_loop"]
pub unsafe extern "C" fn skip_loop(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;

    /* prevent compiler to optimize everything out */
    let mut p: *mut u32 = rdonly_values.a.as_ptr() as *mut u32;
    let mut iters: u32 = 0;
    let mut sum: u32 = 0;

    /* we should never enter this loop */
    while *p & 1 != 0 {
        iters = iters.wrapping_add(1);
        sum = sum.wrapping_add(*p);
        p = p.add(1);
    }
    res.did_run = 1;
    res.iters = iters;
    res.sum = sum;
    0
}

#[no_mangle]
#[link_section = "raw_tracepoint/sys_enter:part_loop"]
pub unsafe extern "C" fn part_loop(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;

    /* prevent compiler to optimize everything out */
    let mut p: *mut u32 = rdonly_values.a.as_ptr() as *mut u32;
    let mut iters: u32 = 0;
    let mut sum: u32 = 0;

    /* validate verifier can derive loop termination */
    while *p < 5 {
        iters = iters.wrapping_add(1);
        sum = sum.wrapping_add(*p);
        p = p.add(1);
    }
    res.did_run = 1;
    res.iters = iters;
    res.sum = sum;
    0
}

#[no_mangle]
#[link_section = "raw_tracepoint/sys_enter:full_loop"]
pub unsafe extern "C" fn full_loop(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;

    /* prevent compiler to optimize everything out */
    let mut p: *mut u32 = rdonly_values.a.as_ptr() as *mut u32;
    let mut i: i32 = rdonly_values.a.len() as i32;
    let mut iters: u32 = 0;
    let mut sum: u32 = 0;

    /* validate verifier can allow full loop as well */
    while i > 0 {
        iters = iters.wrapping_add(1);
        sum = sum.wrapping_add(*p);
        p = p.add(1);
        i -= 1;
    }
    res.did_run = 1;
    res.iters = iters;
    res.sum = sum;
    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
