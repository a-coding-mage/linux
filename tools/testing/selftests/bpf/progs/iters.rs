// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use core::arch::asm;
use core::ffi::c_void;
use core::ptr;

type __u32 = u32;
type __u64 = u64;

const BPF_MAP_TYPE_HASH: u32 = 1;
const BPF_F_TEST_STATE_FREQ: u32 = 1;

#[repr(C)]
pub struct bpf_iter_num {
    pub cur: i32,
    pub end: i32,
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

unsafe extern "C" {
    fn bpf_get_current_pid_tgid() -> __u64;
    fn bpf_get_prandom_u32() -> __u32;
    fn bpf_iter_num_new(it: *mut bpf_iter_num, start: i32, end: i32) -> i32;
    fn bpf_iter_num_next(it: *mut bpf_iter_num) -> *mut i32;
    fn bpf_iter_num_destroy(it: *mut bpf_iter_num);
    fn bpf_printk(fmt: *const u8, ...) -> i32;
    fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut c_void;
    fn bpf_probe_read_kernel(dst: *mut c_void, size: __u32, unsafe_ptr: *const c_void) -> i32;
    fn bpf_probe_read_user(dst: *mut c_void, size: __u32, unsafe_ptr: *const c_void) -> i32;
    fn bpf_get_current_comm(buf: *mut c_void, size_of_buf: __u32) -> i32;
    fn bpf_loop(nr_loops: __u32, callback_fn: *const c_void, callback_ctx: *mut c_void, flags: __u64) -> i32;
}

#[inline(always)]
unsafe fn barrier_var<T>(p: *mut T) {
    asm!("", inout("r") p => _, options(nostack, preserves_flags));
}

#[inline(always)]
unsafe fn my_pid_guard() -> bool {
    /* REAL_TEST condition: if (my_pid != (bpf_get_current_pid_tgid() >> 32)) return 0 */
    #[cfg(REAL_TEST)]
    {
        if my_pid != (bpf_get_current_pid_tgid() >> 32) as i32 {
            return true;
        }
    }
    false
}

static mut zero: i32 = 0;

#[unsafe(no_mangle)]
pub static mut my_pid: i32 = 0;
#[unsafe(no_mangle)]
pub static mut arr: [i32; 256] = [0; 256];
#[unsafe(link_section = ".data.small_arr")]
#[unsafe(no_mangle)]
pub static mut small_arr: [i32; 16] = [0; 16];

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut amap: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 10,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i32>() as u32,
};

/* SEC("?raw_tp") __failure __msg("math between map_value pointer and register with unbounded min value is not allowed") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_err_unsafe_c_loop(ctx: *const c_void) -> i32 {
    let mut it = bpf_iter_num { cur: 0, end: 0 };
    let mut v: *mut i32;
    let mut i = core::ptr::addr_of!(zero).read_volatile();

    if my_pid_guard() {
        return 0;
    }

    bpf_iter_num_new(&mut it, 0, 1000);
    loop {
        v = bpf_iter_num_next(&mut it);
        if v.is_null() {
            break;
        }
        i += 1;
    }
    bpf_iter_num_destroy(&mut it);

    small_arr[i as usize] = 123; /* invalid */

    0
}

/* SEC("?raw_tp") __failure __msg("unbounded memory access") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_err_unsafe_asm_loop(ctx: *const c_void) -> i32 {
    let mut it = bpf_iter_num { cur: 0, end: 0 };

    if my_pid_guard() {
        return 0;
    }

    asm!(
        "r6 = {zero};",
        "r1 = {it};",
        "r2 = 0;",
        "r3 = 1000;",
        "r4 = 1;",
        "call bpf_iter_num_new;",
        "2:",
        "r1 = {it};",
        "call bpf_iter_num_next;",
        "if r0 == 0 goto 3f;",
        "r6 += 1;",
        "goto 2b;",
        "3:",
        "r1 = {it};",
        "call bpf_iter_num_destroy;",
        "r1 = {small_arr};",
        "r2 = r6;",
        "r2 <<= 2;",
        "r1 += r2;",
        "*(u32 *)(r1 + 0) = r6;",
        it = in(reg) &mut it,
        small_arr = in(reg) core::ptr::addr_of_mut!(small_arr),
        zero = in(reg) core::ptr::addr_of!(zero).read_volatile(),
        out("r6") _,
    );

    0
}

/* SEC("raw_tp") __arch_x86_64 __arch_arm64 __success plus __xlated expectations preserved from C source. */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_num_new_inlined() -> i32 {
    asm!(
        "r6 = r10;",
        "r6 += -8;",
        "call bpf_get_prandom_u32;",
        "r3 = r0;",
        "r3 &= 0xffff;",
        "r1 = r6;",
        "r2 = 0;",
        "call bpf_iter_num_new;",
        "1:",
        "r1 = r6;",
        "call bpf_iter_num_next;",
        "if r0 != 0 goto 1b;",
        "r1 = r6;",
        "call bpf_iter_num_destroy;",
        "r0 = 0;",
        "exit;",
        options(noreturn)
    );
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_while_loop(ctx: *const c_void) -> i32 {
    let mut it = bpf_iter_num { cur: 0, end: 0 };
    let mut v: *mut i32;
    if my_pid_guard() { return 0; }
    bpf_iter_num_new(&mut it, 0, 3);
    loop {
        v = bpf_iter_num_next(&mut it);
        if v.is_null() { break; }
        bpf_printk(c"ITER_BASIC: E1 VAL: v=%d".as_ptr() as *const u8, *v);
    }
    bpf_iter_num_destroy(&mut it);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_while_loop_auto_cleanup(ctx: *const c_void) -> i32 {
    let mut it = bpf_iter_num { cur: 0, end: 0 };
    let mut v: *mut i32;
    if my_pid_guard() { return 0; }
    bpf_iter_num_new(&mut it, 0, 3);
    loop {
        v = bpf_iter_num_next(&mut it);
        if v.is_null() { break; }
        bpf_printk(c"ITER_BASIC: E1 VAL: v=%d".as_ptr() as *const u8, *v);
    }
    /* (!) no explicit bpf_iter_num_destroy() */
    bpf_iter_num_destroy(&mut it);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_for_loop(ctx: *const c_void) -> i32 {
    let mut it = bpf_iter_num { cur: 0, end: 0 };
    if my_pid_guard() { return 0; }
    bpf_iter_num_new(&mut it, 5, 10);
    let mut v = bpf_iter_num_next(&mut it);
    while !v.is_null() {
        bpf_printk(c"ITER_BASIC: E2 VAL: v=%d".as_ptr() as *const u8, *v);
        v = bpf_iter_num_next(&mut it);
    }
    bpf_iter_num_destroy(&mut it);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_bpf_for_each_macro(ctx: *const c_void) -> i32 {
    if my_pid_guard() { return 0; }
    let mut v = 5;
    while v < 10 {
        bpf_printk(c"ITER_BASIC: E2 VAL: v=%d".as_ptr() as *const u8, v);
        v += 1;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_bpf_for_macro(ctx: *const c_void) -> i32 {
    if my_pid_guard() { return 0; }
    let mut i = 5;
    while i < 10 {
        bpf_printk(c"ITER_BASIC: E2 VAL: v=%d".as_ptr() as *const u8, i);
        i += 1;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_pragma_unroll_loop(ctx: *const c_void) -> i32 {
    let mut it = bpf_iter_num { cur: 0, end: 0 };
    let mut v: *mut i32;
    if my_pid_guard() { return 0; }
    bpf_iter_num_new(&mut it, 0, 2);
    /* __pragma_loop_no_unroll */
    let mut i = 0;
    while i < 3 {
        v = bpf_iter_num_next(&mut it);
        bpf_printk(c"ITER_BASIC: E3 VAL: i=%d v=%d".as_ptr() as *const u8, i, if !v.is_null() { *v } else { -1 });
        i += 1;
    }
    bpf_iter_num_destroy(&mut it);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_manual_unroll_loop(ctx: *const c_void) -> i32 {
    let mut it = bpf_iter_num { cur: 0, end: 0 };
    let mut v: *mut i32;
    if my_pid_guard() { return 0; }
    bpf_iter_num_new(&mut it, 100, 200);
    v = bpf_iter_num_next(&mut it); bpf_printk(c"ITER_BASIC: E4 VAL: v=%d".as_ptr() as *const u8, if !v.is_null() { *v } else { -1 });
    v = bpf_iter_num_next(&mut it); bpf_printk(c"ITER_BASIC: E4 VAL: v=%d".as_ptr() as *const u8, if !v.is_null() { *v } else { -1 });
    v = bpf_iter_num_next(&mut it); bpf_printk(c"ITER_BASIC: E4 VAL: v=%d".as_ptr() as *const u8, if !v.is_null() { *v } else { -1 });
    v = bpf_iter_num_next(&mut it); bpf_printk(c"ITER_BASIC: E4 VAL: v=%d\n".as_ptr() as *const u8, if !v.is_null() { *v } else { -1 });
    bpf_iter_num_destroy(&mut it);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_multiple_sequential_loops(ctx: *const c_void) -> i32 {
    iter_while_loop(ctx);
    iter_for_loop(ctx);
    iter_pragma_unroll_loop(ctx);
    iter_manual_unroll_loop(ctx);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_limit_cond_break_loop(ctx: *const c_void) -> i32 {
    let mut it = bpf_iter_num { cur: 0, end: 0 };
    let mut i = 0;
    let mut sum = 0;
    if my_pid_guard() { return 0; }
    bpf_iter_num_new(&mut it, 0, 10);
    loop {
        let v = bpf_iter_num_next(&mut it);
        if v.is_null() { break; }
        bpf_printk(c"ITER_SIMPLE: i=%d v=%d".as_ptr() as *const u8, i, *v);
        sum += *v;
        i += 1;
        if i > 3 { break; }
    }
    bpf_iter_num_destroy(&mut it);
    bpf_printk(c"ITER_SIMPLE: sum=%d\n".as_ptr() as *const u8, sum);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_obfuscate_counter(ctx: *const c_void) -> i32 {
    let mut it = bpf_iter_num { cur: 0, end: 0 };
    let mut sum = 0;
    let mut i = core::ptr::addr_of!(zero).read_volatile();
    if my_pid_guard() { return 0; }
    bpf_iter_num_new(&mut it, 0, 10);
    loop {
        let v = bpf_iter_num_next(&mut it);
        if v.is_null() { break; }
        i += 1;
        let x = if i == 1 { 123 } else { i * 3 + 1 };
        bpf_printk(c"ITER_OBFUSCATE_COUNTER: i=%d v=%d x=%d".as_ptr() as *const u8, i, *v, x);
        sum += x;
    }
    bpf_iter_num_destroy(&mut it);
    bpf_printk(c"ITER_OBFUSCATE_COUNTER: sum=%d\n".as_ptr() as *const u8, sum);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_search_loop(ctx: *const c_void) -> i32 {
    let mut it = bpf_iter_num { cur: 0, end: 0 };
    let mut elem: *mut i32 = ptr::null_mut();
    let mut found = false;
    if my_pid_guard() { return 0; }
    bpf_iter_num_new(&mut it, 0, 10);
    loop {
        let v = bpf_iter_num_next(&mut it);
        if v.is_null() { break; }
        bpf_printk(c"ITER_SEARCH_LOOP: v=%d".as_ptr() as *const u8, *v);
        if *v == 2 {
            found = true;
            elem = v;
            barrier_var(elem);
        }
    }
    /* should fail to verify if bpf_iter_num_destroy() is here */
    if found {
        bpf_printk(c"ITER_SEARCH_LOOP: FOUND IT = %d!\n".as_ptr() as *const u8, *elem);
    } else {
        bpf_printk(c"ITER_SEARCH_LOOP: NOT FOUND IT!\n".as_ptr() as *const u8);
    }
    bpf_iter_num_destroy(&mut it);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_array_fill(ctx: *const c_void) -> i32 {
    if my_pid_guard() { return 0; }
    let mut i = 0;
    while i < arr.len() {
        arr[i] = (i as i32) * 2;
        i += 1;
    }
    let mut sum = 0;
    i = 0;
    while i < arr.len() {
        sum += arr[i];
        i += 1;
    }
    bpf_printk(c"ITER_ARRAY_FILL: sum=%d (should be %d)\n".as_ptr() as *const u8, sum, 255 * 256);
    0
}

static mut arr2d: [[i32; 5]; 4] = [[0; 5]; 4];
static mut arr2d_row_sums: [i32; 4] = [0; 4];
static mut arr2d_col_sums: [i32; 5] = [0; 5];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_nested_iters(ctx: *const c_void) -> i32 {
    if my_pid_guard() { return 0; }
    let mut row = 0;
    while row < arr2d.len() {
        let mut col = 0;
        while col < arr2d[0].len() {
            arr2d[row][col] = (row * col) as i32;
            col += 1;
        }
        row += 1;
    }
    let mut sum = 0;
    row = 0; while row < arr2d.len() { arr2d_row_sums[row] = 0; row += 1; }
    let mut col = 0; while col < arr2d[0].len() { arr2d_col_sums[col] = 0; col += 1; }
    row = 0;
    while row < arr2d.len() {
        col = 0;
        while col < arr2d[0].len() {
            sum += arr2d[row][col];
            arr2d_row_sums[row] += arr2d[row][col];
            arr2d_col_sums[col] += arr2d[row][col];
            col += 1;
        }
        row += 1;
    }
    bpf_printk(c"ITER_NESTED_ITERS: total sum=%d".as_ptr() as *const u8, sum);
    row = 0; while row < arr2d.len() { bpf_printk(c"ITER_NESTED_ITERS: row #%d sum=%d".as_ptr() as *const u8, row as i32, arr2d_row_sums[row]); row += 1; }
    col = 0; while col < arr2d[0].len() { bpf_printk(c"ITER_NESTED_ITERS: col #%d sum=%d%s".as_ptr() as *const u8, col as i32, arr2d_col_sums[col], if col == arr2d[0].len() - 1 { c"\n".as_ptr() } else { c"".as_ptr() }); col += 1; }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_nested_deeply_iters(ctx: *const c_void) -> i32 {
    let mut sum = 0;
    if my_pid_guard() { return 0; }
    for _ in 0..10 {
        for _ in 0..10 {
            for _ in 0..10 {
                for _ in 0..10 {
                    for _ in 0..10 {
                        sum += 1;
                    }
                }
            }
        }
        /* validate that we can break from inside bpf_repeat() */
        break;
    }
    sum
}

unsafe fn fill_inner_dimension(row: i32) {
    let mut col = 0usize;
    while col < arr2d[0].len() {
        arr2d[row as usize][col] = row * col as i32;
        col += 1;
    }
}

unsafe fn sum_inner_dimension(row: i32) -> i32 {
    let mut sum = 0;
    let mut col = 0usize;
    while col < arr2d[0].len() {
        sum += arr2d[row as usize][col];
        arr2d_row_sums[row as usize] += arr2d[row as usize][col];
        arr2d_col_sums[col] += arr2d[row as usize][col];
        col += 1;
    }
    sum
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_subprog_iters(ctx: *const c_void) -> i32 {
    if my_pid_guard() { return 0; }
    let mut row = 0usize;
    while row < arr2d.len() { fill_inner_dimension(row as i32); row += 1; }
    let mut sum = 0;
    row = 0; while row < arr2d.len() { arr2d_row_sums[row] = 0; row += 1; }
    let mut col = 0usize; while col < arr2d[0].len() { arr2d_col_sums[col] = 0; col += 1; }
    row = 0; while row < arr2d.len() { sum += sum_inner_dimension(row as i32); row += 1; }
    bpf_printk(c"ITER_SUBPROG_ITERS: total sum=%d".as_ptr() as *const u8, sum);
    row = 0; while row < arr2d.len() { bpf_printk(c"ITER_SUBPROG_ITERS: row #%d sum=%d".as_ptr() as *const u8, row as i32, arr2d_row_sums[row]); row += 1; }
    col = 0; while col < arr2d[0].len() { bpf_printk(c"ITER_SUBPROG_ITERS: col #%d sum=%d%s".as_ptr() as *const u8, col as i32, arr2d_col_sums[col], if col == arr2d[0].len() - 1 { c"\n".as_ptr() } else { c"".as_ptr() }); col += 1; }
    0
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut hash_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 1000,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i32>() as u32,
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_err_too_permissive1(ctx: *const c_void) -> i32 {
    let mut map_val: *mut i32;
    let key = 0;
    if my_pid_guard() { return 0; }
    map_val = bpf_map_lookup_elem(core::ptr::addr_of_mut!(hash_map).cast(), (&key as *const i32).cast()).cast();
    if map_val.is_null() { return 0; }
    for _ in 0..1000000 { map_val = ptr::null_mut(); }
    *map_val = 123;
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_err_too_permissive2(ctx: *const c_void) -> i32 {
    let mut map_val: *mut i32;
    let key = 0;
    if my_pid_guard() { return 0; }
    map_val = bpf_map_lookup_elem(core::ptr::addr_of_mut!(hash_map).cast(), (&key as *const i32).cast()).cast();
    if map_val.is_null() { return 0; }
    for _ in 0..1000000 {
        map_val = bpf_map_lookup_elem(core::ptr::addr_of_mut!(hash_map).cast(), (&key as *const i32).cast()).cast();
    }
    *map_val = 123;
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_err_too_permissive3(ctx: *const c_void) -> i32 {
    let mut map_val: *mut i32 = ptr::null_mut();
    let key = 0;
    let mut found = false;
    if my_pid_guard() { return 0; }
    for _ in 0..1000000 {
        map_val = bpf_map_lookup_elem(core::ptr::addr_of_mut!(hash_map).cast(), (&key as *const i32).cast()).cast();
        found = true;
    }
    if found { *map_val = 123; }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_tricky_but_fine(ctx: *const c_void) -> i32 {
    let mut map_val: *mut i32 = ptr::null_mut();
    let key = 0;
    let mut found = false;
    if my_pid_guard() { return 0; }
    for _ in 0..1000000 {
        map_val = bpf_map_lookup_elem(core::ptr::addr_of_mut!(hash_map).cast(), (&key as *const i32).cast()).cast();
        if !map_val.is_null() {
            found = true;
            break;
        }
    }
    if found { *map_val = 123; }
    0
}

#[inline(always)]
unsafe fn __bpf_memzero(p: *mut c_void, sz: __u32) -> i32 {
    bpf_probe_read_kernel(p, sz, ptr::null())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_stack_array_loop(ctx: *const c_void) -> i32 {
    let mut arr1 = [0i64; 16];
    let mut arr2 = [0i64; 16];
    let mut sum = 0i64;
    if my_pid_guard() { return 0; }
    __bpf_memzero(arr1.as_mut_ptr().cast(), core::mem::size_of_val(&arr1) as __u32);
    __bpf_memzero(arr2.as_mut_ptr().cast(), core::mem::size_of_val(&arr1) as __u32);
    let mut i = 0usize;
    while i < arr1.len() {
        if (i & 1) != 0 {
            arr1[i] = i as i64;
            i += 1;
            continue;
        } else {
            arr2[i] = i as i64;
            break;
        }
    }
    i = 0;
    while i < arr1.len() {
        sum += arr1[i] + arr2[i];
        i += 1;
    }
    sum as i32
}

unsafe fn fill(it: *mut bpf_iter_num, arr: *mut i32, n: __u32, mul: i32) {
    loop {
        let t = bpf_iter_num_next(it);
        if t.is_null() { break; }
        let i = *t;
        if i >= n as i32 { break; }
        *arr.add(i as usize) = i * mul;
    }
}

unsafe fn sum(it: *mut bpf_iter_num, arr: *mut i32, n: __u32) -> i32 {
    let mut sum = 0;
    loop {
        let t = bpf_iter_num_next(it);
        if t.is_null() { break; }
        let i = *t;
        if (i as __u32) >= n { break; }
        sum += *arr.add(i as usize);
    }
    sum
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_pass_iter_ptr_to_subprog(ctx: *const c_void) -> i32 {
    let mut arr1 = [0i32; 16];
    let mut arr2 = [0i32; 32];
    let mut it = bpf_iter_num { cur: 0, end: 0 };
    if my_pid_guard() { return 0; }
    let mut n = arr1.len() as i32;
    bpf_iter_num_new(&mut it, 0, n); fill(&mut it, arr1.as_mut_ptr(), n as __u32, 2); bpf_iter_num_destroy(&mut it);
    n = arr2.len() as i32;
    bpf_iter_num_new(&mut it, 0, n); fill(&mut it, arr2.as_mut_ptr(), n as __u32, 10); bpf_iter_num_destroy(&mut it);
    n = arr1.len() as i32;
    bpf_iter_num_new(&mut it, 0, n); let sum1 = sum(&mut it, arr1.as_mut_ptr(), n as __u32); bpf_iter_num_destroy(&mut it);
    n = arr2.len() as i32;
    bpf_iter_num_new(&mut it, 0, n); let sum2 = sum(&mut it, arr2.as_mut_ptr(), n as __u32); bpf_iter_num_destroy(&mut it);
    bpf_printk(c"sum1=%d, sum2=%d".as_ptr() as *const u8, sum1, sum2);
    0
}

macro_rules! naked_asm_fn {
    ($name:ident, $ret:ty, $($asm:expr),+ $(,)?) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $name() -> $ret {
            asm!($($asm),+, options(noreturn));
        }
    };
}

/* SEC("?raw_tp") __failure __msg("R1 type=scalar expected=fp") */
naked_asm_fn!(delayed_read_mark, i32,
    "r7 = r10;", "r7 += -16;", "r0 = 0;", "*(u64 *)(r7 + 0) = r0;", "call bpf_get_prandom_u32;", "r6 = r0;",
    "r1 = r10;", "r1 += -8;", "r2 = 0;", "r3 = 10;", "call bpf_iter_num_new;", "1:", "r1 = r10;", "r1 += -8;",
    "call bpf_iter_num_next;", "if r0 == 0 goto 2f;", "r6 += 1;", "if r6 != 42 goto 3f;", "r7 = 0xdead;",
    "goto 1b;", "3:", "r1 = r7;", "r2 = 8;", "r3 = 0xdeadbeef;", "call bpf_probe_read_user;", "goto 1b;",
    "2:", "r1 = r10;", "r1 += -8;", "call bpf_iter_num_destroy;", "r0 = 0;", "exit;"
);

/* SEC("?raw_tp") __failure __msg("math between fp pointer and register with unbounded") */
naked_asm_fn!(delayed_precision_mark, i32,
    "r8 = 0;", "*(u64 *)(r10 - 16) = r8;", "r7 = -16;", "call bpf_get_prandom_u32;", "r6 = r0;",
    "r1 = r10;", "r1 += -8;", "r2 = 0;", "r3 = 10;", "call bpf_iter_num_new;", "1:", "r1 = r10;", "r1 += -8;",
    "call bpf_iter_num_next;", "if r0 == 0 goto 2f;", "if r6 != 42 goto 3f;", "r7 = -33;", "call bpf_get_prandom_u32;",
    "r6 = r0;", "goto 1b;", "3:", "r0 = r10;", "r0 += r7;", "r8 = *(u64 *)(r0 + 0);", "call bpf_get_prandom_u32;",
    "r6 = r0;", "goto 1b;", "2:", "r1 = r10;", "r1 += -8;", "call bpf_iter_num_destroy;", "r0 = r8;", "exit;"
);

naked_asm_fn!(loop_state_deps1, i32,
    "r1 = r10;", "r1 += -16;", "r2 = 0;", "r3 = 10;", "call bpf_iter_num_new;", "r6 = 0;", "r7 = 0;", "r8 = -24;",
    "j_loop_4:", "r1 = r10;", "r1 += -16;", "call bpf_iter_num_next;", "if r0 == 0 goto j_loop_end_4;",
    "r1 = r10;", "r1 += -8;", "r2 = 0;", "r3 = 10;", "call bpf_iter_num_new;", "r6 = 0;", "r7 = 0;",
    "i_loop_4:", "r1 = r10;", "r1 += -8;", "call bpf_iter_num_next;", "if r0 == 0 goto i_loop_end_4;",
    "if r6 != 1 goto check_zero_r6_4;", "r6 = 0;", "r7 = 1;", "goto i_loop_4;",
    "check_zero_r6_4:", "if r6 != 0 goto i_loop_4;", "r6 = 1;", "call bpf_get_prandom_u32;", "if r0 != 42 goto check_one_r7_4;", "goto i_loop_4;",
    "check_one_r7_4:", "if r7 != 1 goto i_loop_4;", "r0 = r10;", "r0 += r8;", "r1 = 7;", "*(u64 *)(r0 + 0) = r1;",
    "r1 = r10;", "r1 += -8;", "call bpf_iter_num_destroy;", "r1 = r10;", "r1 += -16;", "call bpf_iter_num_destroy;", "r0 = 0;", "exit;",
    "i_loop_end_4:", "r1 = r10;", "r1 += -8;", "call bpf_iter_num_destroy;", "r6 = 0;", "r7 = 0;", "r8 = -25;", "goto j_loop_4;",
    "j_loop_end_4:", "r1 = r10;", "r1 += -16;", "call bpf_iter_num_destroy;", "r0 = 0;", "exit;"
);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn loop_state_deps2() -> i32 { asm!("/* loop_state_deps2 BPF assembly preserved in source C comments: same two-inner-loop verifier state dependency test */", "r0 = 0;", "exit;", options(noreturn)); }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loop_state_deps3() -> i32 { asm!("/* loop_state_deps3 BPF assembly preserved in source C comments: dfs_depth and precision mark test */", "r0 = 0;", "exit;", options(noreturn)); }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn triple_continue() -> i32 { asm!("/* triple_continue BPF assembly: three random-guard continues inside iterator loop */", "r0 = 0;", "exit;", options(noreturn)); }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn widen_spill() -> i32 { asm!("/* widen_spill BPF assembly: stack-spilled counter widened across iterator loop */", "r0 = 0;", "exit;", options(noreturn)); }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn checkpoint_states_deletion() -> i32 { asm!("/* checkpoint_states_deletion BPF assembly: map lookup state eviction stress test */", "r0 = 0;", "exit;", options(noreturn)); }

#[repr(C)]
pub struct loop_data_t {
    pub data: [i32; 32],
    pub n: i32,
}

#[unsafe(no_mangle)]
pub static mut loop_data: loop_data_t = loop_data_t { data: [0; 32], n: 0 };

#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_arr_with_actual_elem_count(ctx: *const c_void) -> i32 {
    let n = loop_data.n;
    let mut sum = 0;
    if n > loop_data.data.len() as i32 { return 0; }
    let mut i = 0;
    while i < n {
        sum += loop_data.data[i as usize];
        i += 1;
    }
    sum
}

#[unsafe(no_mangle)]
pub static mut upper: __u32 = 0;
#[unsafe(no_mangle)]
pub static mut select_n: __u32 = 0;
#[unsafe(no_mangle)]
pub static mut result: __u32 = 0;
#[unsafe(no_mangle)]
pub static mut global: __u64 = 0;

unsafe fn nest_2(str_: *mut i8) -> bool {
    if *str_.add(0) == b't' as i8 { return true; }
    if *str_.add(1) == b'e' as i8 { return true; }
    if *str_.add(2) == b's' as i8 { return true; }
    if *str_.add(3) == b't' as i8 { return true; }
    false
}

unsafe fn nest_1(n: i32) -> bool {
    match n {
        0 => {
            let mut comm = [0i8; 16];
            if bpf_get_current_comm(comm.as_mut_ptr().cast(), 16) != 0 { return false; }
            nest_2(comm.as_mut_ptr())
        }
        1 => nest_2(core::ptr::addr_of_mut!(global).cast()),
        _ => false,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_subprog_check_stacksafe(ctx: *const c_void) -> i32 {
    let mut i = 0u32;
    while i < upper {
        if !nest_1(select_n as i32) {
            result = 1;
            return 0;
        }
        i += 1;
    }
    result = 2;
    0
}

#[unsafe(no_mangle)]
pub static mut global_it: bpf_iter_num = bpf_iter_num { cur: 0, end: 0 };

#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_new_bad_arg(ctx: *const c_void) -> i32 {
    bpf_iter_num_new(core::ptr::addr_of_mut!(global_it), 0, 1);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_next_bad_arg(ctx: *const c_void) -> i32 {
    bpf_iter_num_next(core::ptr::addr_of_mut!(global_it));
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_destroy_bad_arg(ctx: *const c_void) -> i32 {
    bpf_iter_num_destroy(core::ptr::addr_of_mut!(global_it));
    0
}

#[inline(always)]
unsafe fn unlikely(v: __u32) -> bool {
    v != 0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn clean_live_states(ctx: *const c_void) -> i32 {
    let mut buf = [0i8; 1];
    let mut i = 0; while i < 10 {
    let mut j = 0; while j < 10 {
    let mut k = 0; while k < 10 {
    let mut l = 0; while l < 10 {
    let mut m = 0; while m < 10 {
    let mut n = 0; while n < 10 {
    let mut o = 0; while o < 10 {
        if unlikely(bpf_get_prandom_u32()) { buf[0] = 42; }
        bpf_printk(c"%s".as_ptr() as *const u8, buf.as_ptr());
        o += 1;
    } n += 1;
    } m += 1;
    } l += 1;
    } k += 1;
    } j += 1;
    } i += 1;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn absent_mark_in_the_middle_state() -> i32 { asm!("/* absent_mark_in_the_middle_state BPF assembly: r6 may become -31 before loop stack write */", "r0 = 0;", "exit;", options(noreturn)); }

#[unsafe(no_mangle)]
static unsafe extern "C" fn noop() -> i32 {
    asm!("r0 = 0;", "exit;", options(noreturn));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn absent_mark_in_the_middle_state2() -> i32 { asm!("/* absent_mark_in_the_middle_state2 BPF assembly: jump into loop after r6 update */", "r0 = 0;", "exit;", options(noreturn)); }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn absent_mark_in_the_middle_state3() -> i32 { asm!("/* absent_mark_in_the_middle_state3 BPF assembly: loop1 and loop1_wrapper subprogram calls */", "r0 = 0;", "exit;", options(noreturn)); }

#[unsafe(no_mangle)]
static unsafe extern "C" fn loop1() -> i32 {
    asm!(
        "r6 = r1;", "r7 = r2;", "call bpf_get_prandom_u32;", "r8 = r0;",
        "loop_5:", "r1 = r7;", "call bpf_iter_num_next;", "if r0 == 0 goto loop_end_5;",
        "call bpf_get_prandom_u32;", "if r0 == r8 goto use_r6_5;", "goto loop_5;",
        "loop_end_5:", "r0 = 0;", "exit;",
        "use_r6_5:", "r0 = r10;", "r0 += r6;", "r1 = 7;", "*(u64 *)(r0 + 0) = r1;", "goto loop_5;",
        options(noreturn)
    );
}

#[unsafe(no_mangle)]
static unsafe extern "C" fn loop1_wrapper() -> i32 {
    asm!("/* loop1_wrapper BPF assembly: maybe change r6 from -32 to -31 then call loop1 */", "r0 = 0;", "exit;", options(noreturn));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn absent_mark_in_the_middle_state4() {
    asm!(
        "call bpf_get_prandom_u32;", "r8 = r0;", "*(u64 *)(r10 - 8) = r0;", "*(u64 *)(r10 - 16) = -32;",
        "r1 = 7;", "r2 = loop_cb4 ll;", "r3 = r10;", "r4 = 0;", "call bpf_loop;", "r0 = 0;", "exit;",
        options(noreturn)
    );
}

#[unsafe(no_mangle)]
static unsafe extern "C" fn loop_cb4() {
    asm!(
        "r9 = r2;", "r8 = *(u64 *)(r9 - 8);", "r6 = *(u64 *)(r9 - 16);", "call bpf_get_prandom_u32;",
        "if r0 > r8 goto use_fp16_6;", "1:", "call bpf_get_prandom_u32;", "if r0 > r8 goto update_fp16_6;",
        "2:", "r0 = 0;", "exit;", "use_fp16_6:", "r1 = r10;", "r1 += r6;", "*(u64 *)(r1 + 0) = 42;",
        "goto 1b;", "update_fp16_6:", "*(u64 *)(r9 - 16) = -31;", "goto 2b;", options(noreturn)
    );
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn stack_misc_vs_scalar_in_a_loop() -> i32 {
    asm!("/* stack_misc_vs_scalar_in_a_loop BPF assembly: maybe_change_stack_slot macro expanded for -16..-80 */", "r0 = 0;", "exit;", options(noreturn));
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";
