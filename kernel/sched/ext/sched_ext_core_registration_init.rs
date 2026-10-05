// SPDX-License-Identifier: GPL-2.0
// F17 source addition to the retained ext.rs baseline, not a retranslation.
// Authority: 126a30fae3bba11420ec2fcbde51a0a01bab1b5b ext.c.
// Native records, macros, iterators, guards and primitive leaves are explicit
// unqualified C runtime. No old ext.c body is an algorithm fallback.
compile_error!("SOURCE ONLY HOLD: sched_ext registration/init ABI, metadata, sections and protection remain unqualified");

use super::*;
use kernel::ffi::{c_int, c_ulong, c_void};

/// Forward the native class-table callback to F01's sole existing algorithm.
///
/// # Safety
/// Only the native update_curr_scx class adapter may call this continuation.
/// rq is live and locked, preemption is disabled, and its current ext task and
/// accounting/server state meet F01 update_curr_scx's original contract.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_ri_update_curr_body(rq: *mut rq) {
    // SAFETY: The native class adapter preserves the caller's rq protection.
    // The root reexport selects the existing F01 Rust body, not a C fallback.
    unsafe { super::update_curr_scx(rq); }
}

/// SysRq-S retains its original caller-owned RCU context, with no new guard.
///
/// # Safety
/// The native SysRq callback supplies its original context and pins the root
/// through the existing caller protocol. Native rcu_dereference is preserved.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_ri_sysrq_reset_body(_key: u8) {
    // SAFETY: The SysRq caller's RCU context covers both dereference and exit
    // queueing. The null path never passes a scheduler pointer to scx_disable.
    unsafe {
        let sch = lupos_scx_ri_reset_root_rcu();
        if lupos_scx_ri_likely_sched(sch) {
            scx_disable(sch, SCX_EXIT_SYSRQ);
        } else {
            lupos_scx_ri_info_not_loaded();
        }
    }
}

/// SysRq-D delegates only its exact native RCU traversal/stack-object envelope.
///
/// # Safety
/// ei is the initialized native stack object and lives for this synchronous
/// traversal. The caller's existing RCU protocol pins the scheduler list.
/// The SysRq context must permit scx_dump_state's IRQ-save and nested rq locks.
/// The iterator and per-entry continuation must not retain ei or unwind.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_ri_sysrq_dump_body(
    _key: u8, ei: *mut scx_exit_info,
) {
    // SAFETY: Native list_for_each_entry_rcu preserves the caller's protected
    // traversal and synchronously forwards the same live stack object.
    unsafe { lupos_scx_ri_for_each_sched_rcu(ei); }
}

/// Preserve the original per-scheduler dump action and its fixed arguments.
///
/// # Safety
/// Invoked synchronously inside the native list_for_each_entry_rcu envelope;
/// sch is the current live entry and ei is the native SysRq stack object.
/// ei has the original SCX_EXIT_NONE/-1/SysRq-D initialization; omitted fields
/// are zero, and this context permits the original dump and nested rq locks.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_ri_dump_one_body(
    sch: *mut scx_sched, ei: *mut scx_exit_info,
) {
    // SAFETY: The native aggregate is valid for this zero-length dump, and
    // both scheduler and stack object remain pinned until dump_state returns.
    unsafe { scx_dump_state(sch, ei, 0, false); }
}

/// Apply the original PM bypass transition while the native RCU guard lives.
///
/// # Safety
/// The native notifier shell holds its exact guard(rcu) across this call,
/// including every early return and the existing scx_bypass operation.
/// The PM context permits scx_bypass's raw IRQ-save locking and task cycling;
/// no Rust unwinding is permitted through the native guard scope.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_ri_pm_body(
    _nb: *mut notifier_block, event: c_ulong, _ptr: *mut c_void,
) -> c_int {
    // SAFETY: The native RCU guard pins a nonnull root through the whole
    // bypass transition. nb and ptr are never dereferenced or retained here.
    unsafe {
        let sch = lupos_scx_ri_pm_root_rcu();
        if sch.is_null() {
            return NOTIFY_OK as c_int;
        }
        if event == PM_HIBERNATION_PREPARE as c_ulong
            || event == PM_SUSPEND_PREPARE as c_ulong
            || event == PM_RESTORE_PREPARE as c_ulong
        {
            scx_bypass(sch, true);
        } else if event == PM_POST_HIBERNATION as c_ulong
            || event == PM_POST_SUSPEND as c_ulong
            || event == PM_POST_RESTORE as c_ulong
        {
            scx_bypass(sch, false);
        }
        NOTIFY_OK as c_int
    }
}

/// Continue the early class initialization after native enum retention.
///
/// # Safety
/// Called once from the original native __init entry during sched_init, before
/// the late kfunc initcall. CPU iteration uses the real possible-CPU macro.
#[no_mangle]
#[link_section = ".init.text"]
#[cold]
pub unsafe extern "C" fn lupos_scx_ri_init_class_body() {
    // SAFETY: The original one-time boot entry runs before this init text is
    // discarded. Native masks and possible-CPU iteration precede registration
    // of the original callbacks and initialization of their shared work slots.
    unsafe {
        lupos_scx_ri_idle_init_masks();
        lupos_scx_ri_for_each_possible_cpu();
        // Original return values are intentionally ignored at both call sites.
        lupos_scx_ri_register_reset_key();
        lupos_scx_ri_register_dump_key();
        lupos_scx_ri_init_watchdog();
        #[cfg(CONFIG_EXT_SUB_SCHED)]
        lupos_scx_ri_init_sched_hash();
    }
}

/// Initialize one possible CPU in the original field/callback order.
///
/// # Safety
/// The native boot iterator supplies a possible CPU and initialized native rq
/// storage. Each BUG_ON remains native and fatal; no unwind is introduced.
/// The canonical configured rq layout must match the native iterator's storage;
/// this is the one-time initialization of its list, lock and work fields.
#[no_mangle]
#[link_section = ".init.text"]
#[cold]
pub unsafe extern "C" fn lupos_scx_ri_init_cpu_body(cpu: i32) {
    // SAFETY: A possible CPU has live native rq storage throughout boot init.
    // addr_of_mut! passes raw addresses of its real list fields without
    // constructing Rust references; each native leaf initializes its own slot.
    unsafe {
        let rq = lupos_scx_ri_cpu_rq(cpu);
        let n = lupos_scx_core_cpu_to_node(cpu);
        lupos_scx_ri_init_local_dsq(rq);
        #[cfg(CONFIG_EXT_SUB_SCHED)]
        {
            lupos_scx_ri_init_reject_dsq(rq);
            lupos_scx_ri_rescue_init(rq);
        }
        lupos_scx_ri_init_list(ptr::addr_of_mut!((*rq).scx.runnable_list));
        lupos_scx_ri_init_list(ptr::addr_of_mut!((*rq).scx.ddsp_deferred_locals));
        lupos_scx_ri_alloc_cpus_to_sync(rq, n);
        lupos_scx_ri_init_list(ptr::addr_of_mut!((*rq).scx.sched_pcpus_to_kick));
        lupos_scx_ri_init_reenq_lock(rq);
        lupos_scx_ri_init_list(ptr::addr_of_mut!((*rq).scx.deferred_reenq_locals));
        lupos_scx_ri_init_list(ptr::addr_of_mut!((*rq).scx.deferred_reenq_users));
        lupos_scx_ri_init_deferred_irq(rq);
        lupos_scx_ri_init_kick_irq(rq);
        if lupos_scx_ri_cpu_online(cpu) {
            lupos_scx_ri_mark_online(cpu);
        }
    }
}

/// Register late metadata after the exact native CPU/CID offset assertions.
///
/// # Safety
/// Called only by scx_init's original device-level __initcall, with the
/// canonical native metadata/owner definitions present in the same unit.
/// Failed registrations return the original error without added cleanup.
/// The system has reached the original registration/allocation context and
/// this one-time call completes before its init text can be discarded.
#[no_mangle]
#[link_section = ".init.text"]
#[cold]
pub unsafe extern "C" fn lupos_scx_ri_init_body() -> c_int {
    // SAFETY: Native static sets, struct_ops records, notifier and sysfs data
    // retain their original lifetimes. This ordered boot continuation is the
    // sole setter of F00's kset pointer; a null result never reaches sysfs.
    unsafe {
        // Preserve the original eight-call || chain's left-to-right ordering
        // and first-error short circuit. A failing call skips every later one.
        let mut ret = lupos_scx_ri_register_enqueue_dispatch();
        if ret == 0 { ret = lupos_scx_ri_register_dispatch(); }
        if ret == 0 { ret = lupos_scx_ri_register_cpu_release(); }
        if ret == 0 { ret = lupos_scx_ri_register_unlocked_struct_ops(); }
        if ret == 0 { ret = lupos_scx_ri_register_unlocked_syscall(); }
        if ret == 0 { ret = lupos_scx_ri_register_any_struct_ops(); }
        if ret == 0 { ret = lupos_scx_ri_register_any_tracing(); }
        if ret == 0 { ret = lupos_scx_ri_register_any_syscall(); }
        if ret != 0 {
            lupos_scx_ri_error_kfunc_sets(ret);
            return ret;
        }
        ret = lupos_scx_ri_idle_init();
        if ret != 0 {
            lupos_scx_ri_error_idle_init(ret);
            return ret;
        }
        ret = lupos_scx_ri_cid_kfunc_init();
        if ret != 0 {
            lupos_scx_ri_error_cid_kfuncs(ret);
            return ret;
        }
        ret = lupos_scx_ri_register_cpu_ops();
        if ret != 0 {
            lupos_scx_ri_error_cpu_ops(ret);
            return ret;
        }
        ret = lupos_scx_ri_register_cid_ops();
        if ret != 0 {
            lupos_scx_ri_error_cid_ops(ret);
            return ret;
        }
        ret = lupos_scx_ri_register_pm();
        if ret != 0 {
            lupos_scx_ri_error_pm(ret);
            return ret;
        }
        let kset = lupos_scx_ri_create_kset();
        lupos_scx_core_kset_set(kset);
        if lupos_scx_core_kset().is_null() {
            lupos_scx_ri_error_kset();
            return -(ENOMEM as c_int);
        }
        ret = lupos_scx_ri_create_global_group();
        // This last original branch is < 0, not != 0. Positive returns fall
        // through to success, and no kset put or notifier rollback is added.
        if ret < 0 {
            lupos_scx_ri_error_global_group();
            return ret;
        }
        0
    }
}
