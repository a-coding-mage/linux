// SPDX-License-Identifier: GPL-2.0
// Rust translation of verifier_iterating_callbacks.c.
// C includes removed: "bpf_misc.h", "bpf_experimental.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::arch::asm;
use core::ffi::c_void;

type __u8 = u8;
type __u32 = u32;
type __u64 = u64;

const BPF_MAP_TYPE_ARRAY: u32 = 0;
const BPF_MAP_TYPE_USER_RINGBUF: u32 = 0;
const BPF_F_TEST_STATE_FREQ: u32 = 0;
const ARR_SZ: usize = 1000000;
const ARR2_SZ: usize = 1000;
const ARR_LONG_SZ: usize = 1000;

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_dynptr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_iter_num {
    _private: [u8; 0],
}

#[repr(C)]
pub struct buf_context {
    pub buf: *mut i8,
}

#[repr(C)]
pub struct num_context {
    pub i: __u64,
    pub j: __u64,
}

#[repr(C)]
pub struct iter_limit_bug_ctx {
    pub a: __u64,
    pub b: __u64,
    pub c: __u64,
}

#[repr(C)]
pub struct arr_foo_elem {
    pub a: i32,
    pub b: i32,
}

// SEC(".maps") map definition:
// __uint(type, BPF_MAP_TYPE_ARRAY); __uint(max_entries, 8);
// __type(key, __u32); __type(value, __u64);
#[no_mangle]
pub static mut map: bpf_map = bpf_map { _private: [] };

// SEC(".maps") ringbuf definition:
// __uint(type, BPF_MAP_TYPE_USER_RINGBUF); __uint(max_entries, 8);
#[no_mangle]
pub static mut ringbuf: bpf_map = bpf_map { _private: [] };

#[no_mangle]
pub static mut choice_arr: [__u8; 2] = [0, 1];

#[no_mangle]
pub static mut tmp_var: i32 = 0;

#[no_mangle]
pub static mut zero: i32 = 0;

#[no_mangle]
pub static mut arr: [i8; ARR_SZ] = [0; ARR_SZ];

// SEC(".data.arr2")
#[no_mangle]
pub static mut arr2: [i8; ARR2_SZ] = [0; ARR2_SZ];

pub static limit: i32 = ARR2_SZ as i32;

// SEC(".data.arr_long")
#[no_mangle]
pub static mut arr_long: [isize; ARR_LONG_SZ] = [0; ARR_LONG_SZ];

// SEC(".data.arr_foo")
#[no_mangle]
pub static mut arr_foo: [arr_foo_elem; ARR_LONG_SZ] = [arr_foo_elem { a: 0, b: 0 }; ARR_LONG_SZ];

// SEC(".data.buf")
#[no_mangle]
pub static mut buf: [i8; 10] = [0; 10];

// SEC("license")
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

extern "C" {
    fn bpf_probe_read_user(dst: *mut i8, size: u32, unsafe_ptr: *mut c_void) -> i32;
    fn bpf_loop(nr_loops: u32, callback_fn: *const c_void, callback_ctx: *mut c_void, flags: u64) -> i32;
    fn bpf_for_each_map_elem(map: *mut bpf_map, callback_fn: *const c_void, callback_ctx: *mut c_void, flags: u64) -> i32;
    fn bpf_user_ringbuf_drain(map: *mut bpf_map, callback_fn: *const c_void, callback_ctx: *mut c_void, flags: u64) -> i32;
    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn bpf_find_vma(task: *mut task_struct, addr: __u64, callback_fn: *const c_void, callback_ctx: *mut c_void, flags: u64) -> i32;
    fn bpf_get_prandom_u32() -> u32;
    fn bpf_jiffies64() -> u64;
    fn bpf_ktime_get_ns() -> u64;
    fn bpf_iter_num_new(it: *mut bpf_iter_num, start: i32, end: i32);
    fn bpf_iter_num_next(it: *mut bpf_iter_num) -> *mut i32;
    fn bpf_iter_num_destroy(it: *mut bpf_iter_num);
}

#[inline(always)]
unsafe fn barrier_var_u64(_v: __u64) {
    asm!("", inout("r") _v => _, options(nostack, preserves_flags));
}

#[inline(always)]
unsafe fn barrier_var_isize(_v: isize) {
    asm!("", inout("r") _v => _, options(nostack, preserves_flags));
}

#[inline(always)]
unsafe fn can_loop() -> bool {
    true
}

#[inline(always)]
unsafe fn cond_break() {
    asm!(
        ".byte 0xe5",
        ".byte 0",
        ".short 0",
        ".long 0",
        options(nostack)
    );
}

unsafe extern "C" fn unsafe_on_2nd_iter_cb(idx: __u32, ctx: *mut buf_context) -> i32 {
    if idx == 0 {
        (*ctx).buf = 0xDEADusize as *mut i8;
        return 0;
    }

    if bpf_probe_read_user((*ctx).buf, 8, 0xBADC0FFEEusize as *mut c_void) != 0 {
        return 1;
    }

    0
}

// SEC("?raw_tp") __failure __msg("R1 type=scalar expected=fp")
#[no_mangle]
pub unsafe extern "C" fn unsafe_on_2nd_iter(_unused: *mut c_void) -> i32 {
    let mut local_buf = [0i8; 4];
    let mut loop_ctx = buf_context { buf: local_buf.as_mut_ptr() };

    bpf_loop(100, unsafe_on_2nd_iter_cb as *const c_void, &mut loop_ctx as *mut _ as *mut c_void, 0);
    0
}

unsafe extern "C" fn unsafe_on_zero_iter_cb(_idx: __u32, ctx: *mut num_context) -> i32 {
    (*ctx).i = 0;
    0
}

// SEC("?raw_tp") __failure __msg("invalid access to map value, value_size=2 off=32 size=1")
#[no_mangle]
pub unsafe extern "C" fn unsafe_on_zero_iter(_unused: *mut c_void) -> i32 {
    let mut loop_ctx = num_context { i: 32, j: 0 };

    bpf_loop(100, unsafe_on_zero_iter_cb as *const c_void, &mut loop_ctx as *mut _ as *mut c_void, 0);
    choice_arr[loop_ctx.i as usize] as i32
}

unsafe extern "C" fn widening_cb(_idx: __u32, ctx: *mut num_context) -> i32 {
    (*ctx).i = (*ctx).i.wrapping_add(1);
    0
}

// SEC("?raw_tp") __success
#[no_mangle]
pub unsafe extern "C" fn widening(_unused: *mut c_void) -> i32 {
    let mut loop_ctx = num_context { i: 0, j: 1 };

    bpf_loop(100, widening_cb as *const c_void, &mut loop_ctx as *mut _ as *mut c_void, 0);
    /* loop_ctx.j is not changed during callback iteration,
     * verifier should not apply widening to it.
     */
    choice_arr[loop_ctx.j as usize] as i32
}

unsafe extern "C" fn loop_detection_cb(_idx: __u32, _ctx: *mut num_context) -> i32 {
    loop {}
}

// SEC("?raw_tp") __failure __msg("infinite loop detected")
#[no_mangle]
pub unsafe extern "C" fn loop_detection(_unused: *mut c_void) -> i32 {
    let mut loop_ctx = num_context { i: 0, j: 0 };

    bpf_loop(100, loop_detection_cb as *const c_void, &mut loop_ctx as *mut _ as *mut c_void, 0);
    0
}

#[inline(always)]
unsafe fn oob_state_machine(ctx: *mut num_context) -> __u64 {
    match (*ctx).i {
        0 => {
            (*ctx).i = 1;
        }
        1 => {
            (*ctx).i = 32;
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn for_each_map_elem_cb(
    _map: *mut bpf_map,
    _key: *mut __u32,
    _val: *mut __u64,
    data: *mut c_void,
) -> __u64 {
    oob_state_machine(data as *mut num_context)
}

// SEC("?raw_tp") __failure __msg("invalid access to map value, value_size=2 off=32 size=1")
#[no_mangle]
pub unsafe extern "C" fn unsafe_for_each_map_elem(_unused: *mut c_void) -> i32 {
    let mut loop_ctx = num_context { i: 0, j: 0 };

    bpf_for_each_map_elem(&mut map, for_each_map_elem_cb as *const c_void, &mut loop_ctx as *mut _ as *mut c_void, 0);
    choice_arr[loop_ctx.i as usize] as i32
}

unsafe extern "C" fn ringbuf_drain_cb(_dynptr: *mut bpf_dynptr, data: *mut c_void) -> __u64 {
    oob_state_machine(data as *mut num_context)
}

// SEC("?raw_tp") __failure __msg("invalid access to map value, value_size=2 off=32 size=1")
#[no_mangle]
pub unsafe extern "C" fn unsafe_ringbuf_drain(_unused: *mut c_void) -> i32 {
    let mut loop_ctx = num_context { i: 0, j: 0 };

    bpf_user_ringbuf_drain(&mut ringbuf, ringbuf_drain_cb as *const c_void, &mut loop_ctx as *mut _ as *mut c_void, 0);
    choice_arr[loop_ctx.i as usize] as i32
}

unsafe extern "C" fn find_vma_cb(
    _task: *mut task_struct,
    _vma: *mut vm_area_struct,
    data: *mut c_void,
) -> __u64 {
    oob_state_machine(data as *mut num_context)
}

// SEC("?raw_tp") __failure __msg("invalid access to map value, value_size=2 off=32 size=1")
#[no_mangle]
pub unsafe extern "C" fn unsafe_find_vma(_unused: *mut c_void) -> i32 {
    let task = bpf_get_current_task_btf();
    let mut loop_ctx = num_context { i: 0, j: 0 };

    bpf_find_vma(task, 0, find_vma_cb as *const c_void, &mut loop_ctx as *mut _ as *mut c_void, 0);
    choice_arr[loop_ctx.i as usize] as i32
}

unsafe extern "C" fn iter_limit_cb(_idx: __u32, ctx: *mut num_context) -> i32 {
    (*ctx).i = (*ctx).i.wrapping_add(1);
    0
}

// SEC("?raw_tp") __success
#[no_mangle]
pub unsafe extern "C" fn bpf_loop_iter_limit_ok(_unused: *mut c_void) -> i32 {
    let mut ctx = num_context { i: 0, j: 0 };

    bpf_loop(1, iter_limit_cb as *const c_void, &mut ctx as *mut _ as *mut c_void, 0);
    choice_arr[ctx.i as usize] as i32
}

// SEC("?raw_tp") __failure __msg("invalid access to map value, value_size=2 off=2 size=1")
#[no_mangle]
pub unsafe extern "C" fn bpf_loop_iter_limit_overflow(_unused: *mut c_void) -> i32 {
    let mut ctx = num_context { i: 0, j: 0 };

    bpf_loop(2, iter_limit_cb as *const c_void, &mut ctx as *mut _ as *mut c_void, 0);
    choice_arr[ctx.i as usize] as i32
}

unsafe extern "C" fn iter_limit_level2a_cb(_idx: __u32, ctx: *mut num_context) -> i32 {
    (*ctx).i = (*ctx).i.wrapping_add(100);
    0
}

unsafe extern "C" fn iter_limit_level2b_cb(_idx: __u32, ctx: *mut num_context) -> i32 {
    (*ctx).i = (*ctx).i.wrapping_add(10);
    0
}

unsafe extern "C" fn iter_limit_level1_cb(_idx: __u32, ctx: *mut num_context) -> i32 {
    (*ctx).i = (*ctx).i.wrapping_add(1);
    bpf_loop(1, iter_limit_level2a_cb as *const c_void, ctx as *mut c_void, 0);
    bpf_loop(1, iter_limit_level2b_cb as *const c_void, ctx as *mut c_void, 0);
    0
}

/* Check that path visiting every callback function once had been
 * reached by verifier. Variables 'ctx{1,2}i' below serve as flags,
 * with each decimal digit corresponding to a callback visit marker.
 */
// SEC("socket") __success __retval(111111)
#[no_mangle]
pub unsafe extern "C" fn bpf_loop_iter_limit_nested(_unused: *mut c_void) -> i32 {
    let mut ctx1 = num_context { i: 0, j: 0 };
    let mut ctx2 = num_context { i: 0, j: 0 };
    let a: __u64;
    let b: __u64;
    let mut c: __u64;

    bpf_loop(1, iter_limit_level1_cb as *const c_void, &mut ctx1 as *mut _ as *mut c_void, 0);
    bpf_loop(1, iter_limit_level1_cb as *const c_void, &mut ctx2 as *mut _ as *mut c_void, 0);
    a = ctx1.i;
    b = ctx2.i;
    /* Force 'ctx1.i' and 'ctx2.i' precise. */
    c = choice_arr[((a + b) % 2) as usize] as __u64;
    /* This makes 'c' zero, but neither clang nor verifier know it. */
    c /= 10;
    /* Make sure that verifier does not visit 'impossible' states:
     * enumerate all possible callback visit masks.
     */
    if a != 0 && a != 1 && a != 11 && a != 101 && a != 111 &&
        b != 0 && b != 1 && b != 11 && b != 101 && b != 111 {
        asm!("r0 /= 0", options(nostack));
    }
    (1000u64.wrapping_mul(a).wrapping_add(b).wrapping_add(c)) as i32
}

#[naked]
unsafe extern "C" fn iter_limit_bug_cb() {
    /* This is the same as C code below, but written
     * in assembly to control which branches are fall-through.
     *
     *   switch (bpf_get_prandom_u32()) {
     *   case 1:  ctx->a = 42; break;
     *   case 2:  ctx->b = 42; break;
     *   default: ctx->c = 42; break;
     *   }
     */
    asm!(
        "r9 = r2",
        "call {bpf_get_prandom_u32}",
        "r1 = r0",
        "r2 = 42",
        "r0 = 0",
        "if r1 == 0x1 goto 1f",
        "if r1 == 0x2 goto 2f",
        "*(u64 *)(r9 + 16) = r2",
        "exit",
        "1: *(u64 *)(r9 + 0) = r2",
        "exit",
        "2: *(u64 *)(r9 + 8) = r2",
        "exit",
        bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
        options(noreturn)
    );
}

// SEC("socket") __failure __msg("infinite loop detected at insn 2")
#[naked]
#[no_mangle]
pub unsafe extern "C" fn jgt_imm64_and_may_goto() {
    asm!(
        "r0 = {tmp_var} ll",
        "0:",
        ".byte 0xe5",
        ".byte 0",
        ".short -3",
        ".long 0",
        "if r0 > 10 goto 0b",
        "r0 = 0",
        "exit",
        tmp_var = sym tmp_var,
        options(noreturn)
    );
}

// SEC("socket") __failure __msg("infinite loop detected at insn 1")
#[naked]
#[no_mangle]
pub unsafe extern "C" fn may_goto_self() {
    asm!(
        "r0 = *(u32 *)(r10 - 4)",
        "0:",
        ".byte 0xe5",
        ".byte 0",
        ".short -1",
        ".long 0",
        "if r0 > 10 goto 0b",
        "r0 = 0",
        "exit",
        options(noreturn)
    );
}

// SEC("socket") __success __retval(0)
#[naked]
#[no_mangle]
pub unsafe extern "C" fn may_goto_neg_off() {
    asm!(
        "r0 = *(u32 *)(r10 - 4)",
        "goto 0f",
        "goto 1f",
        "0:",
        ".byte 0xe5",
        ".byte 0",
        ".short -2",
        ".long 0",
        "if r0 > 10 goto 0b",
        "1:",
        "r0 = 0",
        "exit",
        options(noreturn)
    );
}

// SEC("tc") __failure __flag(BPF_F_TEST_STATE_FREQ)
#[no_mangle]
pub unsafe extern "C" fn iter_limit_bug(_skb: *mut __sk_buff) -> i32 {
    let mut ctx = iter_limit_bug_ctx { a: 7, b: 7, c: 7 };

    bpf_loop(2, iter_limit_bug_cb as *const c_void, &mut ctx as *mut _ as *mut c_void, 0);

    /* This is the same as C code below,
     * written in assembly to guarantee checks order.
     *
     *   if (ctx.a == 42 && ctx.b == 42 && ctx.c == 7)
     *     asm volatile("r1 /= 0;":::"r1");
     */
    asm!(
        "r1 = *(u64 *){ctx_a}",
        "if r1 != 42 goto 1f",
        "r1 = *(u64 *){ctx_b}",
        "if r1 != 42 goto 1f",
        "r1 = *(u64 *){ctx_c}",
        "if r1 != 7 goto 1f",
        "r1 /= 0",
        "1:",
        ctx_a = in(reg) &ctx.a,
        ctx_b = in(reg) &ctx.b,
        ctx_c = in(reg) &ctx.c,
        out("r1") _,
    );
    0
}

// SEC("socket") __success __retval(0)
#[naked]
#[no_mangle]
pub unsafe extern "C" fn ja_and_may_goto() {
    asm!(
        "0:",
        ".byte 0xe5",
        ".byte 0",
        ".short 1",
        ".long 0",
        "goto 0b",
        "r0 = 0",
        "exit",
        options(noreturn)
    );
}

// SEC("socket") __success __retval(0)
#[naked]
#[no_mangle]
pub unsafe extern "C" fn ja_and_may_goto2() {
    asm!(
        "0:",
        "r0 = 0",
        ".byte 0xe5",
        ".byte 0",
        ".short 1",
        ".long 0",
        "goto 0b",
        "r0 = 0",
        "exit",
        options(noreturn)
    );
}

// SEC("socket") __success __retval(0)
#[naked]
#[no_mangle]
pub unsafe extern "C" fn jlt_and_may_goto() {
    asm!(
        "0:",
        "call {bpf_jiffies64}",
        ".byte 0xe5",
        ".byte 0",
        ".short 1",
        ".long 0",
        "if r0 < 10 goto 0b",
        "r0 = 0",
        "exit",
        bpf_jiffies64 = sym bpf_jiffies64,
        options(noreturn)
    );
}

// Original C condition: #ifdef CAN_USE_GOTOL
// SEC("socket") __success __retval(0)
#[cfg(CAN_USE_GOTOL)]
#[naked]
#[no_mangle]
pub unsafe extern "C" fn gotol_and_may_goto() {
    asm!(
        "0:",
        "r0 = 0",
        ".byte 0xe5",
        ".byte 0",
        ".short 1",
        ".long 0",
        "gotol 0b",
        "r0 = 0",
        "exit",
        options(noreturn)
    );
}

// SEC("socket") __success __retval(0)
#[naked]
#[no_mangle]
pub unsafe extern "C" fn ja_and_may_goto_subprog() {
    asm!(
        "call subprog_with_may_goto",
        "exit",
        options(noreturn)
    );
}

#[naked]
#[no_mangle]
unsafe extern "C" fn subprog_with_may_goto() {
    asm!(
        "0:",
        ".byte 0xe5",
        ".byte 0",
        ".short 1",
        ".long 0",
        "goto 0b",
        "r0 = 0",
        "exit",
        options(noreturn)
    );
}

// SEC("socket") __success __retval(0xd495cdc0)
#[no_mangle]
pub unsafe extern "C" fn cond_break1(_ctx: *const c_void) -> i32 {
    let mut i: usize;
    let mut sum: u32 = 0;

    i = zero as usize;
    while i < ARR_SZ && can_loop() {
        sum = sum.wrapping_add(i as u32);
        i = i.wrapping_add(1);
    }
    i = zero as usize;
    while i < ARR_SZ {
        barrier_var_u64(i as __u64);
        sum = sum.wrapping_add((i as u32).wrapping_add(arr[i] as u32));
        cond_break();
        i = i.wrapping_add(1);
    }

    sum as i32
}

// SEC("socket") __success __retval(999000000)
#[no_mangle]
pub unsafe extern "C" fn cond_break2(_ctx: *const c_void) -> i32 {
    let mut i: i32;
    let mut j: i32;
    let mut sum: i32 = 0;

    i = zero;
    while i < 1000 && can_loop() {
        j = zero;
        while j < 1000 {
            sum = sum.wrapping_add(i.wrapping_add(j));
            cond_break();
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    sum
}

#[inline(never)]
unsafe fn loop_() -> i32 {
    let mut i: i32;
    let mut sum: i32 = 0;

    i = zero;
    while i <= 1000000 && can_loop() {
        sum = sum.wrapping_add(i);
        i = i.wrapping_add(1);
    }

    sum
}

// SEC("socket") __success __retval(0x6a5a2920)
#[no_mangle]
pub unsafe extern "C" fn cond_break3(_ctx: *const c_void) -> i32 {
    loop_()
}

// SEC("socket") __success __retval(1)
#[no_mangle]
pub unsafe extern "C" fn cond_break4(_ctx: *const c_void) -> i32 {
    let mut cnt = zero;

    loop {
        /* should eventually break out of the loop */
        cond_break();
        cnt = cnt.wrapping_add(1);
    }
    #[allow(unreachable_code)]
    {
        /* if we looped a bit, it's a success */
        if cnt > 1 { 1 } else { 0 }
    }
}

#[inline(never)]
unsafe fn static_subprog() -> i32 {
    let mut cnt = zero;

    loop {
        cond_break();
        cnt = cnt.wrapping_add(1);
    }

    #[allow(unreachable_code)]
    cnt
}

// SEC("socket") __success __retval(1)
#[no_mangle]
pub unsafe extern "C" fn cond_break5(_ctx: *const c_void) -> i32 {
    let mut cnt1 = zero;
    let cnt2: i32;

    loop {
        cond_break();
        cnt1 = cnt1.wrapping_add(1);
    }

    #[allow(unreachable_code)]
    {
        cnt2 = static_subprog();

        /* main and subprog have to loop a bit */
        if cnt1 > 1 && cnt2 > 1 { 1 } else { 0 }
    }
}

// SEC("socket") __success __flag(BPF_F_TEST_STATE_FREQ)
#[no_mangle]
pub unsafe extern "C" fn loop_inside_iter(_ctx: *const c_void) -> i32 {
    let mut it: bpf_iter_num = core::mem::zeroed();
    let mut v: *mut i32;
    let mut sum: i32 = 0;
    let mut i: __u64 = 0;

    bpf_iter_num_new(&mut it, 0, ARR2_SZ as i32);
    loop {
        v = bpf_iter_num_next(&mut it);
        if v.is_null() {
            break;
        }
        if i < ARR2_SZ as __u64 {
            sum = sum.wrapping_add(arr2[i as usize] as i32);
            i = i.wrapping_add(1);
        }
    }
    bpf_iter_num_destroy(&mut it);
    sum
}

// SEC("socket") __success __flag(BPF_F_TEST_STATE_FREQ)
#[no_mangle]
pub unsafe extern "C" fn loop_inside_iter_signed(_ctx: *const c_void) -> i32 {
    let mut it: bpf_iter_num = core::mem::zeroed();
    let mut v: *mut i32;
    let mut sum: i32 = 0;
    let mut i: isize = 0;

    bpf_iter_num_new(&mut it, 0, ARR2_SZ as i32);
    loop {
        v = bpf_iter_num_next(&mut it);
        if v.is_null() {
            break;
        }
        if i < ARR2_SZ as isize && i >= 0 {
            sum = sum.wrapping_add(arr2[i as usize] as i32);
            i = i.wrapping_add(1);
        }
    }
    bpf_iter_num_destroy(&mut it);
    sum
}

// SEC("socket") __success __flag(BPF_F_TEST_STATE_FREQ)
#[no_mangle]
pub unsafe extern "C" fn loop_inside_iter_volatile_limit(_ctx: *const c_void) -> i32 {
    let mut it: bpf_iter_num = core::mem::zeroed();
    let mut v: *mut i32;
    let mut sum: i32 = 0;
    let mut i: __u64 = 0;

    bpf_iter_num_new(&mut it, 0, ARR2_SZ as i32);
    loop {
        v = bpf_iter_num_next(&mut it);
        if v.is_null() {
            break;
        }
        if i < core::ptr::read_volatile(&limit) as __u64 {
            sum = sum.wrapping_add(arr2[i as usize] as i32);
            i = i.wrapping_add(1);
        }
    }
    bpf_iter_num_destroy(&mut it);
    sum
}

// SEC("socket") __success
#[no_mangle]
pub unsafe extern "C" fn test1(_ctx: *const c_void) -> i32 {
    let mut i: isize = 0;

    while i < ARR_LONG_SZ as isize && can_loop() {
        arr_long[i as usize] = i;
        i = i.wrapping_add(1);
    }
    0
}

// SEC("socket") __success
#[no_mangle]
pub unsafe extern "C" fn test2(_ctx: *const c_void) -> i32 {
    let mut i: __u64 = zero as __u64;

    while i < ARR_LONG_SZ as __u64 && can_loop() {
        barrier_var_u64(i);
        arr_long[i as usize] = i as isize;
        i = i.wrapping_add(1);
    }
    0
}

// SEC("socket") __success
#[no_mangle]
pub unsafe extern "C" fn test3(_ctx: *const c_void) -> i32 {
    let mut i: __u64 = zero as __u64;

    while i < ARR_LONG_SZ as __u64 && can_loop() {
        barrier_var_u64(i);
        arr_foo[i as usize].a = i as i32;
        arr_foo[i as usize].b = i as i32;
        i = i.wrapping_add(1);
    }
    0
}

// SEC("socket") __success
#[no_mangle]
pub unsafe extern "C" fn test4(_ctx: *const c_void) -> i32 {
    let mut i: isize = zero as isize + ARR_LONG_SZ as isize - 1;

    while i < ARR_LONG_SZ as isize && i >= 0 && can_loop() {
        barrier_var_isize(i);
        arr_foo[i as usize].a = i as i32;
        arr_foo[i as usize].b = i as i32;
        i = i.wrapping_sub(1);
    }
    0
}

// SEC("socket") __description("check add const") __success
#[naked]
#[no_mangle]
pub unsafe extern "C" fn check_add_const() {
    /* typical LLVM generated loop with may_goto */
    asm!(
        "call {bpf_ktime_get_ns}",
        "if r0 > 9 goto 1f",
        "0:",
        "r1 = {buf}",
        "r2 = r0",
        "r1 += r2",
        "r3 = *(u8 *)(r1 +0)",
        ".byte 0xe5",
        ".byte 0",
        ".short 4",
        ".long 0",
        "r0 = r2",
        "r0 += 1",
        "if r2 < 9 goto 0b",
        "exit",
        "1:",
        "r0 = 0",
        "exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
        buf = sym buf,
        options(noreturn)
    );
}

// SEC("socket") __failure
// __msg("*(u8 *)(r7 +0) = r0")
// __msg("invalid access to map value, value_size=10 off=10 size=1")
#[naked]
#[no_mangle]
pub unsafe extern "C" fn check_add_const_3regs() {
    asm!(
        "r6 = {buf}",
        "r7 = {buf}",
        "call {bpf_ktime_get_ns}",
        "r1 = r0",
        "r2 = r0",
        "r1 += 1",
        "r2 += 2",
        "if r0 > 8 goto 1f",
        "r6 += r1",
        "r7 += r2",
        "*(u8 *)(r6 +0) = r0",
        "*(u8 *)(r7 +0) = r0",
        "1: exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
        buf = sym buf,
        options(noreturn)
    );
}

// SEC("socket") __failure
// __msg("*(u8 *)(r8 -1) = r0")
// __msg("invalid access to map value, value_size=10 off=10 size=1")
#[naked]
#[no_mangle]
pub unsafe extern "C" fn check_add_const_3regs_2if() {
    asm!(
        "r6 = {buf}",
        "r7 = {buf}",
        "r8 = {buf}",
        "call {bpf_ktime_get_ns}",
        "if r0 < 2 goto 1f",
        "r1 = r0",
        "r2 = r0",
        "r1 += 1",
        "r2 += 2",
        "if r2 > 11 goto 1f",
        "if r0 s< 0 goto 1f",
        "r6 += r0",
        "r7 += r1",
        "r8 += r2",
        "*(u8 *)(r6 +0) = r0",
        "*(u8 *)(r7 -1) = r0",
        "*(u8 *)(r8 -1) = r0",
        "1: exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
        buf = sym buf,
        options(noreturn)
    );
}

// SEC("socket") __failure __flag(BPF_F_TEST_STATE_FREQ)
#[naked]
#[no_mangle]
pub unsafe extern "C" fn check_add_const_regsafe_off() {
    asm!(
        "r8 = {buf}",
        "call {bpf_ktime_get_ns}",
        "r6 = r0",
        "call {bpf_ktime_get_ns}",
        "r7 = r0",
        "call {bpf_ktime_get_ns}",
        "r1 = r0",
        "if r6 > r7 goto 1f",
        "r1 += 1",
        "goto 2f",
        "1: r1 += 100",
        "goto +0",
        "2: if r0 > 8 goto 3f",
        "r8 += r1",
        "*(u8 *)(r8 +0) = r0",
        "3: exit",
        bpf_ktime_get_ns = sym bpf_ktime_get_ns,
        buf = sym buf,
        options(noreturn)
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
