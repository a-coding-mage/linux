/* SPDX-License-Identifier: GPL-2.0+ */
/* Read-Copy Update mechanism for mutual exclusion. */

// The following declarations intentionally refer to kernel-provided types and
// operations. Configuration-specific C preprocessor branches are represented
// with Rust cfg attributes where useful.

pub const RCU_SEQ_CTR_SHIFT: i32 = 2;
pub const RCU_SEQ_STATE_MASK: i32 = (1 << RCU_SEQ_CTR_SHIFT) - 1;
pub const NUM_ACTIVE_RCU_POLL_OLDSTATE: usize = 2;

#[repr(C)]
pub struct rcu_gp_seq {
    pub norm: ::core::ffi::c_ulong,
    pub exp: ::core::ffi::c_ulong,
}

pub type rcu_callback_t = Option<unsafe extern "C" fn(*mut rcu_head)>;
#[repr(C)]
pub struct rcu_head { pub func: rcu_callback_t }
#[repr(C)] pub struct kvfree_rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct lockdep_map { _private: [u8; 0] }
#[repr(C)] pub struct __ctx_lock_RCU { _private: [u8; 0] }

extern "C" {
    pub fn call_rcu(head: *mut rcu_head, func: rcu_callback_t);
    pub fn rcu_barrier_tasks();
    pub fn synchronize_rcu();
    pub fn get_completed_synchronize_rcu() -> ::core::ffi::c_ulong;
    pub fn get_completed_synchronize_rcu_full(gsp: *mut rcu_gp_seq);
    pub fn __rcu_read_lock();
    pub fn __rcu_read_unlock();
    pub fn preempt_disable();
    pub fn preempt_enable();
    pub fn preempt_disable_notrace();
    pub fn preempt_enable_notrace();
    pub fn local_bh_disable();
    pub fn local_bh_enable();
    pub fn migrate_disable();
    pub fn migrate_enable();
    pub fn cond_resched();
    pub fn rcu_softirq_qs();
    pub fn rcu_is_watching() -> bool;
    pub fn preemptible() -> bool;
    pub fn rcu_init();
    pub static mut rcu_scheduler_active: ::core::ffi::c_int;
    pub fn rcu_sched_clock_irq(user: ::core::ffi::c_int);
    pub fn kvfree_call_rcu(head: *mut kvfree_rcu_head, ptr: *mut ::core::ffi::c_void);
    pub fn kfree_call_rcu_nolock(head: *mut kvfree_rcu_head, ptr: *mut ::core::ffi::c_void);
    pub static mut rcu_lock_map: lockdep_map;
    pub static mut rcu_bh_lock_map: lockdep_map;
    pub static mut rcu_sched_lock_map: lockdep_map;
    pub static mut rcu_callback_map: lockdep_map;
    pub static mut rcu_expedited: ::core::ffi::c_int;
    pub static mut rcu_normal: ::core::ffi::c_int;
}

#[inline] pub fn same_state_synchronize_rcu(a: ::core::ffi::c_ulong, b: ::core::ffi::c_ulong) -> bool { a == b }

#[inline] pub unsafe fn rcu_read_lock() { __rcu_read_lock(); }
#[inline] pub unsafe fn rcu_read_unlock() { __rcu_read_unlock(); }
#[inline] pub unsafe fn rcu_read_lock_bh() { local_bh_disable(); }
#[inline] pub unsafe fn rcu_read_unlock_bh() { local_bh_enable(); }
#[inline] pub unsafe fn rcu_read_lock_sched() { preempt_disable(); }
#[inline] pub unsafe fn rcu_read_unlock_sched() { preempt_enable(); }
#[inline] pub unsafe fn rcu_read_lock_sched_notrace() { preempt_disable_notrace(); }
#[inline] pub unsafe fn rcu_read_unlock_sched_notrace() { preempt_enable_notrace(); }
#[inline] pub unsafe fn rcu_read_lock_dont_migrate() { migrate_disable(); rcu_read_lock(); }
#[inline] pub unsafe fn rcu_read_unlock_migrate() { rcu_read_unlock(); migrate_enable(); }

#[inline] pub unsafe fn rcu_head_init(rhp: *mut rcu_head) {
    (*rhp).func = Some(::core::mem::transmute(usize::MAX));
}
#[inline] pub unsafe fn rcu_head_after_call_rcu(rhp: *mut rcu_head, f: rcu_callback_t) -> bool {
    let func = ::core::ptr::read_volatile(&(*rhp).func);
    func == f
}

#[macro_export] macro_rules! ULONG_CMP_GE { ($a:expr, $b:expr) => { (usize::MAX / 2 >= ($a).wrapping_sub($b)) }; }
#[macro_export] macro_rules! ULONG_CMP_LT { ($a:expr, $b:expr) => { (usize::MAX / 2 < ($a).wrapping_sub($b)) }; }
#[macro_export] macro_rules! rcu_pointer_handoff { ($p:expr) => { $p }; }
#[macro_export] macro_rules! rcu_dereference { ($p:expr) => { $p }; }
#[macro_export] macro_rules! rcu_dereference_bh { ($p:expr) => { $p }; }
#[macro_export] macro_rules! rcu_dereference_sched { ($p:expr) => { $p }; }
#[macro_export] macro_rules! rcu_dereference_all { ($p:expr) => { $p }; }
#[macro_export] macro_rules! RCU_INITIALIZER { ($v:expr) => { $v }; }
#[macro_export] macro_rules! RCU_INIT_POINTER { ($p:expr, $v:expr) => { unsafe { ::core::ptr::write_volatile($p, $v) } }; }
#[macro_export] macro_rules! kfree_rcu { ($p:expr, $h:ident) => { kvfree_rcu_arg_2!($p, $h) }; }
#[macro_export] macro_rules! kvfree_rcu { ($p:expr, $h:ident) => { kvfree_rcu_arg_2!($p, $h) }; }
#[macro_export] macro_rules! kfree_rcu_mightsleep { ($p:expr) => { kvfree_rcu_arg_1!($p) }; }
#[macro_export] macro_rules! kvfree_rcu_mightsleep { ($p:expr) => { kvfree_rcu_arg_1!($p) }; }

#[inline] pub unsafe fn kvfree_rcu_arg_1(ptr: *mut ::core::ffi::c_void) { if !ptr.is_null() { kvfree_call_rcu(::core::ptr::null_mut(), ptr); } }
#[inline] pub unsafe fn rcu_preempt_depth() -> i32 { 0 }
#[inline] pub unsafe fn rcu_read_lock_held() -> i32 { 1 }
#[inline] pub unsafe fn rcu_read_lock_bh_held() -> i32 { 1 }
#[inline] pub unsafe fn rcu_read_lock_sched_held() -> i32 { (!preemptible()) as i32 }
#[inline] pub unsafe fn rcu_read_lock_any_held() -> i32 { (!preemptible()) as i32 }
#[inline] pub unsafe fn debug_lockdep_rcu_enabled() -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
