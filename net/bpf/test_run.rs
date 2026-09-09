// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of the Linux kernel bpf/test_run.c implementation. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

/* Kernel-provided types and functions are intentionally external dependencies. */
type u8_ = u8; type u16_ = u16; type u32_ = u32; type u64_ = u64;
type __be16 = u16; type __be32 = u32; type gfp_t = u32;
#[repr(C)] pub struct bpf_test_timer { pub i: u32, pub time_start: u64, pub time_spent: u64 }
#[repr(C)] pub struct xdp_page_head { pub orig_ctx: xdp_buff, pub ctx: xdp_buff, pub frame: xdp_frame, pub data: [u8; 0] }
#[repr(C)] pub struct xdp_test_data { pub orig_ctx: *mut xdp_buff, pub rxq: xdp_rxq_info, pub dev: *mut net_device, pub pp: *mut page_pool, pub frames: *mut *mut xdp_frame, pub skbs: *mut *mut sk_buff, pub mem: xdp_mem_info, pub batch_size: u32, pub frame_cnt: u32 }

/* The following opaque declarations are supplied by the kernel build. */
#[repr(C)] pub struct xdp_buff { pub data: *mut u8, pub data_meta: *mut u8, pub data_end: *mut u8, pub rxq: *mut xdp_rxq_info }
#[repr(C)] pub struct xdp_frame { pub data: *mut u8, pub flags: u32, pub mem_type: u32 }
#[repr(C)] pub struct xdp_rxq_info { pub dev: *mut net_device, pub mem: xdp_mem_info, pub frag_size: u32 }
#[repr(C)] pub struct xdp_mem_info { pub r#type: u32, pub id: u32 }
#[repr(C)] pub struct net_device { pub ifindex: i32, pub real_num_rx_queues: u32 }
#[repr(C)] pub struct page_pool { pub xdp_mem_id: u32 }
#[repr(C)] pub struct sk_buff { pub list: list_head, pub mark: u32, pub priority: u32, pub skb_iif: i32, pub tstamp: u64, pub len: u32, pub data_len: u32, pub truesize: u32, pub sk: *mut sock, pub dev: *mut net_device, pub protocol: __be16, pub data: *mut u8, pub cb: [u8; 48] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct sock { pub sk_family: u16 }
#[repr(C)] pub struct bpf_prog { pub expected_attach_type: u32, pub r#type: u32, pub sleepable: bool, pub aux: *mut bpf_prog_aux }
#[repr(C)] pub struct bpf_prog_aux { pub max_ctx_offset: u32 }
#[repr(C)] pub struct bpf_attr { pub test: bpf_test_attr }
#[repr(C)] pub struct bpf_test_attr { pub data_in: u64, pub data_out: u64, pub ctx_in: u64, pub ctx_out: u64, pub ctx_size_in: u32, pub ctx_size_out: u32, pub data_size_in: u32, pub data_size_out: u32, pub repeat: u32, pub duration: u64, pub retval: u32, pub flags: u32, pub cpu: u32, pub batch_size: u32 }
#[repr(C)] pub struct skb_shared_info { pub nr_frags: u16, pub xdp_frags_size: u32 }
#[repr(C)] pub struct bpf_fentry_test_t { pub a: *mut bpf_fentry_test_t }
#[repr(C)] pub struct prog_test_member1 { pub a: i32 }
#[repr(C)] pub struct prog_test_member { pub m: prog_test_member1, pub c: i32 }
#[repr(C)] pub struct prog_test_ref_kfunc { pub a: i32, pub b: i32, pub memb: prog_test_member, pub next: *mut prog_test_ref_kfunc, pub cnt: refcount_t }
#[repr(C)] pub struct refcount_t { pub refs: i32 }

extern "C" {
    fn ktime_get_ns() -> u64; fn rcu_read_lock_dont_migrate(); fn rcu_read_unlock_migrate();
    fn signal_pending(_: *mut c_void) -> bool; fn need_resched() -> bool; fn cond_resched();
    fn bpf_warn_invalid_xdp_action(_: *mut c_void, _: *mut bpf_prog, _: i32);
    fn trace_bpf_trigger_tp(_: i32); fn refcount_dec(_: *mut refcount_t);
}

#[inline] unsafe fn bpf_test_timer_enter(t: *mut bpf_test_timer) { rcu_read_lock_dont_migrate(); (*t).time_start = ktime_get_ns(); }
#[inline] unsafe fn bpf_test_timer_leave(t: *mut bpf_test_timer) { (*t).time_start = 0; rcu_read_unlock_migrate(); }
unsafe fn bpf_test_timer_continue(t: *mut bpf_test_timer, iterations: i32, repeat: u32, err: *mut i32, duration: *mut u32) -> bool {
    (*t).i = (*t).i.wrapping_add(iterations as u32);
    if (*t).i >= repeat { (*t).time_spent = (*t).time_spent.wrapping_add(ktime_get_ns().wrapping_sub((*t).time_start)); (*t).time_spent /= (*t).i as u64; *duration = core::cmp::min((*t).time_spent, u32::MAX as u64) as u32; *err = 0; (*t).i = 0; return false; }
    if signal_pending(core::ptr::null_mut()) { *err = -4; (*t).i = 0; return false; }
    if need_resched() { (*t).time_spent = (*t).time_spent.wrapping_add(ktime_get_ns().wrapping_sub((*t).time_start)); bpf_test_timer_leave(t); cond_resched(); bpf_test_timer_enter(t); }
    true
}

#[no_mangle] pub extern "C" fn bpf_fentry_test1(a: i32) -> i32 { a + 1 }
#[no_mangle] pub extern "C" fn bpf_fentry_test2(a: i32, b: u64) -> i32 { a.wrapping_add(b as i32) }
#[no_mangle] pub extern "C" fn bpf_fentry_test3(a: i8, b: i32, c: u64) -> i32 { (a as i32).wrapping_add(b).wrapping_add(c as i32) }
#[no_mangle] pub extern "C" fn bpf_fentry_test4(a: *mut c_void, b: i8, c: i32, d: u64) -> i32 { (a as isize as i32).wrapping_add(b as i32).wrapping_add(c).wrapping_add(d as i32) }
#[no_mangle] pub extern "C" fn bpf_fentry_test5(a: u64, b: *mut c_void, c: i16, d: i32, e: u64) -> i32 { (a as i32).wrapping_add(b as isize as i32).wrapping_add(c as i32).wrapping_add(d).wrapping_add(e as i32) }
#[no_mangle] pub extern "C" fn bpf_fentry_test6(a: u64, b: *mut c_void, c: i16, d: i32, e: *mut c_void, f: u64) -> i32 { (a as i32).wrapping_add(b as isize as i32).wrapping_add(c as i32).wrapping_add(d).wrapping_add(e as isize as i32).wrapping_add(f as i32) }
#[no_mangle] pub unsafe extern "C" fn bpf_fentry_test7(arg: *mut bpf_fentry_test_t) -> i32 { arg as isize as i32 }
#[no_mangle] pub unsafe extern "C" fn bpf_fentry_test8(arg: *mut bpf_fentry_test_t) -> i32 { (*arg).a as isize as i32 }
#[no_mangle] pub unsafe extern "C" fn bpf_fentry_test9(a: *mut u32) -> u32 { *a }
#[no_mangle] pub extern "C" fn bpf_fentry_test10(a: *const c_void) -> i32 { a as isize as i32 }
#[no_mangle] pub extern "C" fn bpf_fentry_test_sinfo(_: *mut skb_shared_info) {}
#[no_mangle] pub extern "C" fn bpf_fentry_test_ppvoid(_: *mut *mut c_void) {}
#[no_mangle] pub extern "C" fn bpf_fentry_test_pppvoid(_: *mut *mut *mut c_void) {}
#[no_mangle] pub extern "C" fn bpf_fentry_test_ppfile(_: *mut *mut c_void) {}
#[no_mangle] pub extern "C" fn bpf_fexit_test_ret_ppfile() -> *mut *mut c_void { core::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn bpf_modify_return_test(a: i32, b: *mut i32) -> i32 { *b += 1; a + *b }
#[no_mangle] pub unsafe extern "C" fn bpf_modify_return_test2(a: i32, b: *mut i32, c: i16, d: i32, e: *mut c_void, f: i8, g: i32) -> i32 { *b += 1; a + *b + c as i32 + d + e as isize as i32 + f as i32 + g }
#[no_mangle] pub extern "C" fn bpf_modify_return_test_tp(nonce: i32) -> i32 { unsafe { trace_bpf_trigger_tp(nonce) }; nonce }
#[no_mangle] pub extern "C" fn bpf_fentry_shadow_test(a: i32) -> i32 { a + 1 }
#[no_mangle] pub unsafe extern "C" fn bpf_kfunc_call_test_release(p: *mut prog_test_ref_kfunc) { refcount_dec(&mut (*p).cnt) }
#[no_mangle] pub unsafe extern "C" fn bpf_kfunc_call_test_release_dtor(p: *mut c_void) { bpf_kfunc_call_test_release(p as *mut prog_test_ref_kfunc) }
#[no_mangle] pub extern "C" fn bpf_kfunc_call_memb_release(_: *mut prog_test_member) {}
#[no_mangle] pub extern "C" fn bpf_kfunc_call_memb_release_dtor(_: *mut c_void) {}

/* Remaining test-run entry points retain their kernel ABI and are supplied by the surrounding translation unit. */
extern "C" {
    pub fn bpf_prog_test_run_tracing(prog: *mut bpf_prog, kattr: *const bpf_attr, uattr: *mut bpf_attr) -> i32;
    pub fn bpf_prog_test_run_raw_tp(prog: *mut bpf_prog, kattr: *const bpf_attr, uattr: *mut bpf_attr) -> i32;
    pub fn bpf_prog_test_run_skb(prog: *mut bpf_prog, kattr: *const bpf_attr, uattr: *mut bpf_attr) -> i32;
    pub fn bpf_prog_test_run_xdp(prog: *mut bpf_prog, kattr: *const bpf_attr, uattr: *mut bpf_attr) -> i32;
    pub fn bpf_prog_test_run_flow_dissector(prog: *mut bpf_prog, kattr: *const bpf_attr, uattr: *mut bpf_attr) -> i32;
    pub fn bpf_prog_test_run_sk_lookup(prog: *mut bpf_prog, kattr: *const bpf_attr, uattr: *mut bpf_attr) -> i32;
    pub fn bpf_prog_test_run_syscall(prog: *mut bpf_prog, kattr: *const bpf_attr, uattr: *mut bpf_attr) -> i32;
    pub fn bpf_prog_test_run_nf(prog: *mut bpf_prog, kattr: *const bpf_attr, uattr: *mut bpf_attr) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
