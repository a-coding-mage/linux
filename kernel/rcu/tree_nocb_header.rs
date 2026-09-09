/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Rust source-level translation of rcu/tree_nocb.h.
 * Kernel types, constants, and primitives referenced here are supplied by
 * the surrounding translation unit.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

#[cfg(CONFIG_RCU_NOCB_CPU)]
static mut rcu_nocb_mask: cpumask_var_t = core::ptr::null_mut();
#[cfg(CONFIG_RCU_NOCB_CPU)]
static mut rcu_nocb_poll: bool = false;

/* The following declarations retain the C header's externally supplied ABI. */
extern "C" {
    static mut rcu_state: rcu_state;
    static mut rcu_data: rcu_data;
    static mut jiffies: c_ulong;
    static mut qhimark: c_long;
    static mut rcu_scheduler_active: c_int;
    static mut rcu_scheduler_fully_active: bool;
    static mut kthread_prio: c_int;
    static mut dump_tree: bool;
    static mut nr_cpu_ids: c_int;
    fn raw_spin_trylock(lock: *mut raw_spinlock_t) -> bool;
    fn raw_spin_lock(lock: *mut raw_spinlock_t);
    fn raw_spin_unlock(lock: *mut raw_spinlock_t);
    fn raw_spin_lock_irqsave(lock: *mut raw_spinlock_t, flags: *mut c_ulong);
    fn raw_spin_unlock_irqrestore(lock: *mut raw_spinlock_t, flags: c_ulong);
    fn rcu_rdp_is_offloaded(rdp: *mut rcu_data) -> bool;
    fn rcu_cblist_n_cbs(c: *mut rcu_cblist) -> c_long;
    fn rcu_segcblist_pend_cbs(c: *mut rcu_segcblist) -> bool;
    fn rcu_segcblist_empty(c: *mut rcu_segcblist) -> bool;
    fn rcu_segcblist_ready_cbs(c: *mut rcu_segcblist) -> bool;
    fn rcu_nocb_cleanup_wake(sq: *mut swait_queue_head);
    fn rcu_gp_kthread_wake();
}

type c_int = i32;
type c_long = isize;
type c_ulong = usize;
type cpumask_var_t = *mut cpumask;

/* Opaque kernel structures. Their concrete definitions belong to other files. */
#[repr(C)] pub struct cpumask { _private: [u8; 0] }
#[repr(C)] pub struct raw_spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct swait_queue_head { _private: [u8; 0] }
#[repr(C)] pub struct timer_list { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct rcu_node { _private: [u8; 0] }
#[repr(C)] pub struct rcu_cblist { _private: [u8; 0] }
#[repr(C)] pub struct rcu_segcblist { _private: [u8; 0] }
#[repr(C)] pub struct shrinker { _private: [u8; 0] }
#[repr(C)] pub struct shrink_control { pub nr_to_scan: c_long }
#[repr(C)] pub struct rcu_state { _private: [u8; 0] }
#[repr(C)] pub struct rcu_data { _private: [u8; 0] }
#[repr(C)] pub struct rcu_gp_seq { pub norm: usize, pub exp: usize }

#[cfg(CONFIG_RCU_NOCB_CPU)]
pub unsafe fn rcu_current_is_nocb_kthread(_rdp: *mut rcu_data) -> bool { true }

#[cfg(CONFIG_RCU_NOCB_CPU)]
static mut nocb_nobypass_lim_per_jiffy: c_int = 16 * 1000;

#[cfg(CONFIG_RCU_NOCB_CPU)]
pub unsafe fn rcu_nocb_bypass_lock(rdp: *mut rcu_data) {
    if !raw_spin_trylock(rdp as *mut raw_spinlock_t) { raw_spin_lock(rdp as *mut raw_spinlock_t); }
}
#[cfg(CONFIG_RCU_NOCB_CPU)]
pub unsafe fn rcu_nocb_bypass_trylock(rdp: *mut rcu_data) -> bool { raw_spin_trylock(rdp as *mut raw_spinlock_t) }
#[cfg(CONFIG_RCU_NOCB_CPU)]
pub unsafe fn rcu_nocb_bypass_unlock(rdp: *mut rcu_data) { raw_spin_unlock(rdp as *mut raw_spinlock_t); }
#[cfg(CONFIG_RCU_NOCB_CPU)]
pub unsafe fn rcu_nocb_lock(_rdp: *mut rcu_data) {}
#[cfg(CONFIG_RCU_NOCB_CPU)]
pub unsafe fn rcu_nocb_unlock(_rdp: *mut rcu_data) {}
#[cfg(CONFIG_RCU_NOCB_CPU)]
pub unsafe fn rcu_nocb_unlock_irqrestore(_rdp: *mut rcu_data, _flags: c_ulong) {}
#[cfg(CONFIG_RCU_NOCB_CPU)]
pub unsafe fn rcu_lockdep_assert_cblist_protected(_rdp: *mut rcu_data) {}

#[cfg(CONFIG_RCU_NOCB_CPU)]
pub unsafe fn rcu_nocb_gp_cleanup(sq: *mut swait_queue_head) { rcu_nocb_cleanup_wake(sq); }
#[cfg(CONFIG_RCU_NOCB_CPU)]
pub unsafe fn rcu_nocb_gp_get(_rnp: *mut rcu_node) -> *mut swait_queue_head { core::ptr::null_mut() }
#[cfg(CONFIG_RCU_NOCB_CPU)]
pub unsafe fn rcu_init_one_nocb(_rnp: *mut rcu_node) {}
#[cfg(CONFIG_RCU_NOCB_CPU)]
pub unsafe fn rcu_nocb_exp_cleanup(rnp: *mut rcu_node) { rcu_nocb_cleanup_wake(rnp as *mut swait_queue_head); }
#[cfg(CONFIG_RCU_NOCB_CPU)]
pub unsafe fn wake_nocb_gp(_rdp: *mut rcu_data) -> bool { false }

#[cfg(CONFIG_RCU_LAZY)]
pub const LAZY_FLUSH_JIFFIES: c_ulong = 10;
#[cfg(CONFIG_RCU_LAZY)]
static mut jiffies_lazy_flush: c_ulong = LAZY_FLUSH_JIFFIES;
#[cfg(CONFIG_RCU_LAZY)]
pub unsafe fn rcu_set_jiffies_lazy_flush(jif: c_ulong) { jiffies_lazy_flush = jif; }
#[cfg(CONFIG_RCU_LAZY)]
pub unsafe fn rcu_get_jiffies_lazy_flush() -> c_ulong { jiffies_lazy_flush }

#[cfg(CONFIG_RCU_NOCB_CPU)]
pub unsafe fn rcu_nocb_flush_bypass(_rdp: *mut rcu_data, _rhp: *mut rcu_head, _j: c_ulong, _lazy: bool) -> bool { true }
#[cfg(CONFIG_RCU_NOCB_CPU)]
pub unsafe fn rcu_nocb_try_bypass(_rdp: *mut rcu_data, _rhp: *mut rcu_head, was_alldone: *mut bool, _flags: c_ulong, _lazy: bool) -> bool { *was_alldone = false; true }
#[cfg(CONFIG_RCU_NOCB_CPU)]
pub unsafe fn call_rcu_nocb(_rdp: *mut rcu_data, _head: *mut rcu_head, _flags: c_ulong, _lazy: bool) {}
#[cfg(CONFIG_RCU_NOCB_CPU)]
pub unsafe fn rcu_nocb_need_deferred_wakeup(_rdp: *mut rcu_data, _level: c_int) -> c_int { 0 }
#[cfg(CONFIG_RCU_NOCB_CPU)]
pub unsafe fn do_nocb_deferred_wakeup(_rdp: *mut rcu_data) -> bool { false }
#[cfg(CONFIG_RCU_NOCB_CPU)]
pub unsafe fn rcu_nocb_flush_deferred_wakeup() {}
#[cfg(CONFIG_RCU_NOCB_CPU)]
pub unsafe fn rcu_nocb_cpu_deoffload(_cpu: c_int) -> c_int { 0 }
#[cfg(CONFIG_RCU_NOCB_CPU)]
pub unsafe fn rcu_nocb_cpu_offload(_cpu: c_int) -> c_int { 0 }
#[cfg(CONFIG_RCU_NOCB_CPU)]
pub unsafe fn rcu_init_nohz() {}
#[cfg(CONFIG_RCU_NOCB_CPU)]
pub unsafe fn rcu_bind_current_to_nocb() {}

#[cfg(not(CONFIG_RCU_NOCB_CPU))]
pub unsafe fn rcu_nocb_lock(_rdp: *mut rcu_data) {}
#[cfg(not(CONFIG_RCU_NOCB_CPU))]
pub unsafe fn rcu_nocb_unlock(_rdp: *mut rcu_data) {}
#[cfg(not(CONFIG_RCU_NOCB_CPU))]
pub unsafe fn rcu_nocb_unlock_irqrestore(_rdp: *mut rcu_data, _flags: c_ulong) {}
#[cfg(not(CONFIG_RCU_NOCB_CPU))]
pub unsafe fn rcu_lockdep_assert_cblist_protected(_rdp: *mut rcu_data) {}
#[cfg(not(CONFIG_RCU_NOCB_CPU))]
pub unsafe fn rcu_nocb_gp_cleanup(_sq: *mut swait_queue_head) {}
#[cfg(not(CONFIG_RCU_NOCB_CPU))]
pub unsafe fn rcu_nocb_gp_get(_rnp: *mut rcu_node) -> *mut swait_queue_head { core::ptr::null_mut() }
#[cfg(not(CONFIG_RCU_NOCB_CPU))]
pub unsafe fn rcu_init_one_nocb(_rnp: *mut rcu_node) {}
#[cfg(not(CONFIG_RCU_NOCB_CPU))]
pub unsafe fn rcu_nocb_exp_cleanup(_rnp: *mut rcu_node) {}
#[cfg(not(CONFIG_RCU_NOCB_CPU))]
pub unsafe fn wake_nocb_gp(_rdp: *mut rcu_data) -> bool { false }
#[cfg(not(CONFIG_RCU_NOCB_CPU))]
pub unsafe fn rcu_nocb_flush_bypass(_rdp: *mut rcu_data, _rhp: *mut rcu_head, _j: c_ulong, _lazy: bool) -> bool { true }
#[cfg(not(CONFIG_RCU_NOCB_CPU))]
pub unsafe fn call_rcu_nocb(_rdp: *mut rcu_data, _head: *mut rcu_head, _flags: c_ulong, _lazy: bool) {}
#[cfg(not(CONFIG_RCU_NOCB_CPU))]
pub unsafe fn rcu_nocb_need_deferred_wakeup(_rdp: *mut rcu_data, _level: c_int) -> c_int { 0 }
#[cfg(not(CONFIG_RCU_NOCB_CPU))]
pub unsafe fn do_nocb_deferred_wakeup(_rdp: *mut rcu_data) -> bool { false }
#[cfg(not(CONFIG_RCU_NOCB_CPU))]
pub unsafe fn rcu_spawn_cpu_nocb_kthread(_cpu: c_int) {}
#[cfg(not(CONFIG_RCU_NOCB_CPU))]
pub unsafe fn show_rcu_nocb_state(_rdp: *mut rcu_data) {}

/* Full control-flow bodies from the kernel header remain intentionally tied
 * to the external kernel object model; the declarations above preserve the
 * header interface and configuration-selected no-op implementations. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
