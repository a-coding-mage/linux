// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/bpf/progs/exceptions_fail.c.
// C includes removed; external BPF/kernel symbols are declared below.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::c_void;
use core::ptr;

type u32 = u32;
type u64 = u64;

#[repr(C)]
pub struct bpf_rb_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_timer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_spin_lock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_rb_root {
    _private: [u8; 0],
}

#[repr(C)]
pub struct __sk_buff {
    pub len: u32,
    pub protocol: u32,
    pub tstamp: u64,
}

#[repr(C)]
pub struct foo {
    pub node: bpf_rb_node,
}

#[repr(C)]
pub struct hmap_elem {
    pub timer: bpf_timer,
}

// struct { __uint(type, BPF_MAP_TYPE_HASH); __uint(max_entries, 64);
//          __type(key, int); __type(value, struct hmap_elem); } hmap SEC(".maps");
#[repr(C)]
pub struct hmap_map_def {
    _private: [u8; 0],
}

unsafe extern "C" {
    #[link_name = "hmap"]
    static mut hmap: hmap_map_def;

    fn bpf_rcu_read_lock();
    fn bpf_rcu_read_unlock();
    fn bpf_preempt_disable();
    fn bpf_preempt_enable();
    fn bpf_local_irq_save(flags: *mut u64);
    fn bpf_local_irq_restore(flags: *mut u64);

    fn bpf_throw(cookie: u64) -> !;
    fn bpf_map_lookup_elem(map: *mut hmap_map_def, key: *const i32) -> *mut hmap_elem;
    fn bpf_timer_set_callback(
        timer: *mut bpf_timer,
        cb: unsafe extern "C" fn(*mut c_void, *mut i32, *mut bpf_timer) -> i32,
    ) -> i32;
    fn bpf_spin_lock(lock: *mut bpf_spin_lock);
    fn bpf_spin_unlock(lock: *mut bpf_spin_lock);
    fn bpf_rbtree_add(
        root: *mut bpf_rb_root,
        node: *mut bpf_rb_node,
        less: unsafe extern "C" fn(*mut bpf_rb_node, *const bpf_rb_node) -> bool,
    );
    fn bpf_obj_new_foo() -> *mut foo;
    fn bpf_obj_drop(f: *mut foo);
    fn bpf_loop(
        nr_loops: u32,
        cb: unsafe extern "C" fn(u32, *mut c_void) -> i32,
        ctx: *mut c_void,
        flags: u64,
    ) -> i32;
}

// private(A) struct bpf_spin_lock lock;
#[unsafe(no_mangle)]
#[link_section = ".bss.A"]
pub static mut lock: bpf_spin_lock = bpf_spin_lock { _private: [] };

// private(A) struct bpf_rb_root rbtree __contains(foo, node);
#[unsafe(no_mangle)]
#[link_section = ".bss.A"]
pub static mut rbtree: bpf_rb_root = bpf_rb_root { _private: [] };

#[unsafe(no_mangle)]
pub unsafe extern "C" fn exception_cb_bad_ret_type1(_cookie: u64) -> *mut c_void {
    ptr::null_mut()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn exception_cb_bad_ret_type2(_cookie: u64) {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn exception_cb_bad_arg_0() -> i32 {
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn exception_cb_bad_arg_2(_a: i32, _b: i32) -> i32 {
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn exception_cb_ok_arg_small(_a: i32) -> i32 {
    0
}

// SEC("?tc") __exception_cb(exception_cb_bad_ret_type1)
// __failure __msg("Global function exception_cb_bad_ret_type1() return value not void or scalar.")
#[unsafe(no_mangle)]
#[link_section = "?tc"]
pub unsafe extern "C" fn reject_exception_cb_type_1(_ctx: *mut __sk_buff) -> i32 {
    bpf_throw(0);
}

// SEC("?tc") __exception_cb(exception_cb_bad_arg_0)
// __failure __msg("exception cb only supports single integer argument")
#[unsafe(no_mangle)]
#[link_section = "?tc"]
pub unsafe extern "C" fn reject_exception_cb_type_2(_ctx: *mut __sk_buff) -> i32 {
    bpf_throw(0);
}

// SEC("?tc") __exception_cb(exception_cb_bad_arg_2)
// __failure __msg("exception cb only supports single integer argument")
#[unsafe(no_mangle)]
#[link_section = "?tc"]
pub unsafe extern "C" fn reject_exception_cb_type_3(_ctx: *mut __sk_buff) -> i32 {
    bpf_throw(0);
}

// SEC("?tc") __exception_cb(exception_cb_ok_arg_small) __success
#[unsafe(no_mangle)]
#[link_section = "?tc"]
pub unsafe extern "C" fn reject_exception_cb_type_4(_ctx: *mut __sk_buff) -> i32 {
    bpf_throw(0);
}

// SEC("?tc") __exception_cb(exception_cb_bad_ret_type2)
// __failure __msg("exception cb cannot return void")
#[unsafe(no_mangle)]
#[link_section = "?tc"]
pub unsafe extern "C" fn reject_exception_cb_type_5(_ctx: *mut __sk_buff) -> i32 {
    bpf_throw(0);
}

#[inline(never)]
unsafe extern "C" fn timer_cb(
    _map: *mut c_void,
    _key: *mut i32,
    _timer: *mut bpf_timer,
) -> i32 {
    bpf_throw(0);
}

// SEC("?tc") __failure __msg("cannot be called from callback subprog")
#[unsafe(no_mangle)]
#[link_section = "?tc"]
pub unsafe extern "C" fn reject_async_callback_throw(_ctx: *mut __sk_buff) -> i32 {
    let elem: *mut hmap_elem;

    elem = bpf_map_lookup_elem(&raw mut hmap, &0i32 as *const i32);
    if elem.is_null() {
        return 0;
    }
    bpf_timer_set_callback(&mut (*elem).timer, timer_cb)
}

#[inline(never)]
unsafe extern "C" fn subprog_lock(ctx: *mut __sk_buff) -> i32 {
    let ret: i32 = 0;

    bpf_spin_lock(&raw mut lock);
    if (*ctx).len != 0 {
        bpf_throw(0);
    }
    core::ptr::read_volatile(&ret)
}

// SEC("?tc") __failure __msg("function calls are not allowed while holding a lock")
#[unsafe(no_mangle)]
#[link_section = "?tc"]
pub unsafe extern "C" fn reject_with_lock(_ctx: *mut c_void) -> i32 {
    bpf_spin_lock(&raw mut lock);
    bpf_throw(0);
}

// SEC("?tc") __failure __msg("function calls are not allowed while holding a lock")
#[unsafe(no_mangle)]
#[link_section = "?tc"]
pub unsafe extern "C" fn reject_subprog_with_lock(ctx: *mut c_void) -> i32 {
    subprog_lock(ctx as *mut __sk_buff)
}

// SEC("?tc") __failure __msg("bpf_throw cannot be used inside bpf_rcu_read_lock-ed region")
#[unsafe(no_mangle)]
#[link_section = "?tc"]
pub unsafe extern "C" fn reject_with_rcu_read_lock(_ctx: *mut c_void) -> i32 {
    bpf_rcu_read_lock();
    bpf_throw(0);
}

#[inline(never)]
unsafe extern "C" fn throwing_subprog(ctx: *mut __sk_buff) -> i32 {
    if (*ctx).len != 0 {
        bpf_throw(0);
    }
    0
}

// SEC("?tc") __failure __msg("bpf_throw cannot be used inside bpf_rcu_read_lock-ed region")
#[unsafe(no_mangle)]
#[link_section = "?tc"]
pub unsafe extern "C" fn reject_subprog_with_rcu_read_lock(ctx: *mut c_void) -> i32 {
    bpf_rcu_read_lock();
    throwing_subprog(ctx as *mut __sk_buff);
    bpf_rcu_read_unlock();
    0
}

unsafe extern "C" fn rbless(_n1: *mut bpf_rb_node, _n2: *const bpf_rb_node) -> bool {
    bpf_throw(0);
}

// SEC("?tc") __failure __msg("function calls are not allowed while holding a lock")
#[unsafe(no_mangle)]
#[link_section = "?tc"]
pub unsafe extern "C" fn reject_with_rbtree_add_throw(_ctx: *mut c_void) -> i32 {
    let f: *mut foo;

    f = bpf_obj_new_foo();
    if f.is_null() {
        return 0;
    }
    bpf_spin_lock(&raw mut lock);
    bpf_rbtree_add(&raw mut rbtree, &mut (*f).node, rbless);
    bpf_spin_unlock(&raw mut lock);
    0
}

// SEC("?tc") __failure __msg("Unreleased reference")
#[unsafe(no_mangle)]
#[link_section = "?tc"]
pub unsafe extern "C" fn reject_with_reference(_ctx: *mut c_void) -> i32 {
    let f: *mut foo;

    f = bpf_obj_new_foo();
    if f.is_null() {
        return 0;
    }
    bpf_throw(0);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn global_subprog_may_throw(ctx: *mut __sk_buff) -> i32 {
    if (*ctx).len != 0 {
        bpf_throw(0);
    }
    0
}

// SEC("?tc") __failure __msg("Unreleased reference")
#[unsafe(no_mangle)]
#[link_section = "?tc"]
pub unsafe extern "C" fn reject_global_subprog_throw_with_reference(ctx: *mut __sk_buff) -> i32 {
    let f: *mut foo;

    f = bpf_obj_new_foo();
    if f.is_null() {
        return 0;
    }
    if (*ctx).protocol != 0 {
        global_subprog_may_throw(ctx);
    }
    bpf_obj_drop(f);
    0
}

#[inline(never)]
unsafe extern "C" fn subprog_ref(_ctx: *mut __sk_buff) -> i32 {
    let f: *mut foo;

    f = bpf_obj_new_foo();
    if f.is_null() {
        return 0;
    }
    bpf_throw(0);
}

#[inline(never)]
unsafe extern "C" fn subprog_cb_ref(_i: u32, _ctx: *mut c_void) -> i32 {
    bpf_throw(0);
}

// SEC("?tc") __failure __msg("Unreleased reference")
#[unsafe(no_mangle)]
#[link_section = "?tc"]
pub unsafe extern "C" fn reject_with_cb_reference(_ctx: *mut c_void) -> i32 {
    let f: *mut foo;

    f = bpf_obj_new_foo();
    if f.is_null() {
        return 0;
    }
    bpf_loop(5, subprog_cb_ref, ptr::null_mut(), 0);
    bpf_obj_drop(f);
    0
}

// SEC("?tc") __failure __msg("cannot be called from callback")
#[unsafe(no_mangle)]
#[link_section = "?tc"]
pub unsafe extern "C" fn reject_with_cb(_ctx: *mut c_void) -> i32 {
    bpf_loop(5, subprog_cb_ref, ptr::null_mut(), 0);
    0
}

// SEC("?tc") __failure __msg("Unreleased reference")
#[unsafe(no_mangle)]
#[link_section = "?tc"]
pub unsafe extern "C" fn reject_with_subprog_reference(ctx: *mut c_void) -> i32 {
    subprog_ref(ctx as *mut __sk_buff) + 1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn throwing_exception_cb(c: u64) -> i32 {
    bpf_throw(0);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn exception_cb1(c: u64) -> i32 {
    c as i32
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn exception_cb2(c: u64) -> i32 {
    c as i32
}

#[inline(never)]
unsafe extern "C" fn static_func(ctx: *mut __sk_buff) -> i32 {
    exception_cb1((*ctx).tstamp)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn global_func(ctx: *mut __sk_buff) -> i32 {
    exception_cb1((*ctx).tstamp)
}

// SEC("?tc") __exception_cb(throwing_exception_cb)
// __failure __msg("cannot be called from callback subprog")
#[unsafe(no_mangle)]
#[link_section = "?tc"]
pub unsafe extern "C" fn reject_throwing_exception_cb(_ctx: *mut __sk_buff) -> i32 {
    0
}

// SEC("?tc") __exception_cb(exception_cb1)
// __failure __msg("cannot call exception cb directly")
#[unsafe(no_mangle)]
#[link_section = "?tc"]
pub unsafe extern "C" fn reject_exception_cb_call_global_func(ctx: *mut __sk_buff) -> i32 {
    global_func(ctx)
}

// SEC("?tc") __exception_cb(exception_cb1)
// __failure __msg("cannot call exception cb directly")
#[unsafe(no_mangle)]
#[link_section = "?tc"]
pub unsafe extern "C" fn reject_exception_cb_call_static_func(ctx: *mut __sk_buff) -> i32 {
    static_func(ctx)
}

// SEC("?tc") __exception_cb(exception_cb1) __exception_cb(exception_cb2)
// __failure __msg("multiple exception callback tags for main subprog")
#[unsafe(no_mangle)]
#[link_section = "?tc"]
pub unsafe extern "C" fn reject_multiple_exception_cb(_ctx: *mut __sk_buff) -> i32 {
    bpf_throw(0);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn exception_cb_bad_ret(c: u64) -> i32 {
    c as i32
}

// SEC("?fentry/bpf_check") __exception_cb(exception_cb_bad_ret)
// __failure __msg("At program exit the register R0 has unknown scalar value should")
#[unsafe(no_mangle)]
#[link_section = "?fentry/bpf_check"]
pub unsafe extern "C" fn reject_set_exception_cb_bad_ret1(_ctx: *mut c_void) -> i32 {
    0
}

// SEC("?fentry/bpf_check")
// __failure __msg("At program exit the register R1 has smin=64 smax=64 should")
#[unsafe(no_mangle)]
#[link_section = "?fentry/bpf_check"]
pub unsafe extern "C" fn reject_set_exception_cb_bad_ret2(_ctx: *mut c_void) -> i32 {
    bpf_throw(64);
}

#[inline(never)]
unsafe extern "C" fn loop_cb1(_index: u32, _ctx: *mut c_void) -> i32 {
    bpf_throw(0);
}

#[inline(never)]
unsafe extern "C" fn loop_cb2(_index: u32, _ctx: *mut c_void) -> i32 {
    bpf_throw(0);
}

// SEC("?tc") __failure __msg("cannot be called from callback")
#[unsafe(no_mangle)]
#[link_section = "?tc"]
pub unsafe extern "C" fn reject_exception_throw_cb(_ctx: *mut __sk_buff) -> i32 {
    bpf_loop(5, loop_cb1, ptr::null_mut(), 0);
    0
}

// SEC("?tc") __failure __msg("cannot be called from callback")
#[unsafe(no_mangle)]
#[link_section = "?tc"]
pub unsafe extern "C" fn reject_exception_throw_cb_diff(ctx: *mut __sk_buff) -> i32 {
    if (*ctx).protocol != 0 {
        bpf_loop(5, loop_cb1, ptr::null_mut(), 0);
    } else {
        bpf_loop(5, loop_cb2, ptr::null_mut(), 0);
    }
    0
}

// __weak
#[unsafe(no_mangle)]
pub unsafe extern "C" fn foo() {
    bpf_throw(1);
}

// SEC("?fentry/bpf_check")
// __failure __msg("At program exit the register R1 has smin=1 smax=1 should")
#[unsafe(no_mangle)]
#[link_section = "?fentry/bpf_check"]
pub unsafe extern "C" fn reject_out_of_range_global_throw(_skb: *mut __sk_buff) -> i32 {
    foo();

    0
}

#[inline(never)]
unsafe extern "C" fn always_throws() -> i32 {
    bpf_throw(0);
}

#[inline(never)]
unsafe extern "C" fn rcu_lock_then_throw() -> i32 {
    bpf_rcu_read_lock();
    bpf_throw(0);
}

// SEC("?tc") __failure __msg("bpf_throw cannot be used inside bpf_rcu_read_lock-ed region")
#[unsafe(no_mangle)]
#[link_section = "?tc"]
pub unsafe extern "C" fn reject_subprog_rcu_lock_throw(_ctx: *mut c_void) -> i32 {
    rcu_lock_then_throw();
    0
}

// SEC("?tc") __failure __msg("bpf_throw cannot be used inside bpf_preempt_disable-ed region")
#[unsafe(no_mangle)]
#[link_section = "?tc"]
pub unsafe extern "C" fn reject_subprog_throw_preempt_lock(_ctx: *mut c_void) -> i32 {
    bpf_preempt_disable();
    always_throws();
    bpf_preempt_enable();
    0
}

// SEC("?tc") __failure __msg("bpf_throw cannot be used inside bpf_local_irq_save-ed region")
#[unsafe(no_mangle)]
#[link_section = "?tc"]
pub unsafe extern "C" fn reject_subprog_throw_irq_lock(_ctx: *mut c_void) -> i32 {
    let mut flags: u64 = 0;

    bpf_local_irq_save(&mut flags);
    always_throws();
    bpf_local_irq_restore(&mut flags);
    0
}

#[unsafe(no_mangle)]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
