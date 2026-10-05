// SPDX-License-Identifier: GPL-2.0

/*
 * Auto-group scheduling implementation, continued from the existing Rust.
 * Native reference: 126a30fae3bba11420ec2fcbde51a0a01bab1b5b.
 * Configured native headers own all layouts and constants. Opaque objects are
 * never allocated, dereferenced, or borrowed as Rust references here.
 */
#[cfg(CONFIG_RUST_SCHED_AUTOGROUP)]
compile_error!("SOURCE ONLY HOLD: scheduler autogroup is not admitted");

use kernel::bindings::sched_autogroup_native::*;
use kernel::ffi::{c_char, c_int, c_ulong};

/// Initialize the default group and the initial task's signal group.
///
/// # Safety
/// Called once during native scheduler initialization with its live init task.
#[export_name = "lupos_autogroup_init"]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn autogroup_init(init_task: *mut task_struct) {
    // SAFETY: Boot serialization owns the default group's initialization.
    unsafe {
        let ag = lupos_autogroup_default();
        lupos_autogroup_set_tg(ag, lupos_autogroup_root());
        lupos_autogroup_ref_init(ag);
        lupos_autogroup_default_lock_init();
        lupos_autogroup_signal_set(lupos_autogroup_task_signal(init_task), ag);
        #[cfg(CONFIG_SYSCTL)]
        lupos_autogroup_sysctl_init();
    }
}

/// Free autogroup storage on native task-group cleanup.
///
/// # Safety
/// Caller owns native sched_free_group cleanup: either an unpublished group's
/// creation failure or teardown after the required RCU grace periods, with all
/// remaining users excluded. The group's autogroup pointer may be null.
#[no_mangle]
pub unsafe extern "C" fn autogroup_free(tg: *mut task_group) {
    // SAFETY: Caller owns a native cleanup path; kfree(NULL) remains valid.
    unsafe { lupos_autogroup_free(lupos_autogroup_tg_autogroup(tg)) }
}

/// Final-reference callback reached through the native kref release adapter.
///
/// # Safety
/// `ag` owns the last reference, surrendered by native kref_put.
#[export_name = "lupos_autogroup_destroy"]
pub unsafe extern "C" fn autogroup_destroy(ag: *mut autogroup) {
    // SAFETY: kref_put dispatches once for the last reference. Destruction keeps
    // native deferred RCU cleanup; do not free ag or touch tg after destruction.
    unsafe {
        let tg = lupos_autogroup_tg(ag);
        #[cfg(CONFIG_RT_GROUP_SCHED)]
        lupos_autogroup_clear_rt(tg);
        sched_release_group(tg);
        sched_destroy_group(tg);
    }
}

unsafe fn autogroup_kref_put(ag: *mut autogroup) {
    // SAFETY: Caller transfers one live native reference to kref_put.
    unsafe { lupos_autogroup_ref_put(ag) }
}

unsafe fn autogroup_kref_get(ag: *mut autogroup) -> *mut autogroup {
    // SAFETY: Caller holds a reference, siglock, or the default's boot reference.
    unsafe { lupos_autogroup_ref_get(ag) };
    ag
}

unsafe fn autogroup_task_get(p: *mut task_struct) -> *mut autogroup {
    // SAFETY: Caller pins p. Acquire the reference while holding siglock; failed
    // lock acquisition uses the default's retained boot reference.
    unsafe {
        let mut flags: c_ulong = 0;
        if !lupos_autogroup_lock_sighand(p, &mut flags) {
            return autogroup_kref_get(lupos_autogroup_default());
        }
        let ag = autogroup_kref_get(lupos_autogroup_signal_get(
            lupos_autogroup_task_signal(p),
        ));
        lupos_autogroup_unlock_sighand(p, &mut flags);
        ag
    }
}

unsafe fn autogroup_create() -> *mut autogroup {
    // SAFETY: Caller permits GFP_KERNEL allocation and holds no spinlock.
    unsafe {
        let ag = lupos_autogroup_alloc();
        if ag.is_null() {
            if lupos_autogroup_printk_ratelimit() {
                lupos_autogroup_warn_create(false);
            }
            return autogroup_kref_get(lupos_autogroup_default());
        }
        let tg = sched_create_group(lupos_autogroup_root());
        if lupos_autogroup_is_err(tg) {
            lupos_autogroup_free(ag);
            if lupos_autogroup_printk_ratelimit() {
                lupos_autogroup_warn_create(true);
            }
            return autogroup_kref_get(lupos_autogroup_default());
        }
        lupos_autogroup_ref_init(ag);
        lupos_autogroup_lock_init(ag);
        // Native atomic_inc_return is signed int. Cast directly to unsigned
        // long, preserving sign extension on 64-bit targets after int overflow.
        lupos_autogroup_set_id(ag, lupos_autogroup_next_id() as c_ulong);
        lupos_autogroup_set_tg(ag, tg);
        #[cfg(CONFIG_RT_GROUP_SCHED)]
        {
            free_rt_sched_group(tg);
            lupos_autogroup_redirect_rt(tg);
        }
        lupos_autogroup_tg_set_autogroup(tg, ag);
        sched_online_group(tg, lupos_autogroup_root());
        ag
    }
}

/// Decide whether a task in the root CPU group may use its autogroup.
///
/// # Safety
/// `p` and `tg` must satisfy the native task_group scheduler locking contract.
#[no_mangle]
pub unsafe extern "C" fn task_wants_autogroup(
    p: *mut task_struct,
    tg: *mut task_group,
) -> bool {
    // SAFETY: Caller pins both objects. PF_EXITING excludes threads which the
    // group move can no longer find in the signal's thread list.
    unsafe {
        if tg != lupos_autogroup_root() || lupos_autogroup_task_exiting(p) {
            return false;
        }
        true
    }
}

/// Move an exiting task away from the signal's autogroup before exit_notify.
///
/// # Safety
/// Caller owns native task exit sequencing, including the PF_EXITING state.
#[no_mangle]
pub unsafe extern "C" fn sched_autogroup_exit_task(p: *mut task_struct) {
    // SAFETY: Preserve the forced move before losing thread-list visibility.
    unsafe { sched_move_task(p, true) }
}

unsafe fn autogroup_move_group(p: *mut task_struct, ag: *mut autogroup) {
    // SAFETY: Caller pins p and ag and does not hold siglock. The native cursor
    // preserves for_each_thread's RCU reads and initial lockdep diagnostic.
    unsafe {
        let mut flags: c_ulong = 0;
        let locked = lupos_autogroup_lock_sighand(p, &mut flags);
        if lupos_autogroup_warn_move_unlocked(!locked) {
            return;
        }
        let sig = lupos_autogroup_task_signal(p);
        let prev = lupos_autogroup_signal_get(sig);
        if prev == ag {
            lupos_autogroup_unlock_sighand(p, &mut flags);
            return;
        }
        lupos_autogroup_signal_set(sig, autogroup_kref_get(ag));
        // Move every thread after publication and before releasing the old
        // group. siglock prevents missing a migrating thread. Already-removed
        // exiting threads use sched_autogroup_exit_task instead.
        let mut t = lupos_autogroup_thread_first(p);
        while !t.is_null() {
            sched_move_task(t, true);
            t = lupos_autogroup_thread_next(p, t);
        }
        lupos_autogroup_unlock_sighand(p, &mut flags);
        autogroup_kref_put(prev);
    }
}

/// Allocate an autogroup and attach the task's entire signal group.
///
/// # Safety
/// Caller pins p and permits GFP_KERNEL allocation; no spinlock may be held.
#[no_mangle]
pub unsafe extern "C" fn sched_autogroup_create_attach(p: *mut task_struct) {
    // SAFETY: The temporary creation reference survives the move attempt.
    unsafe {
        let ag = autogroup_create();
        autogroup_move_group(p, ag);
        autogroup_kref_put(ag);
    }
}

/// Reattach the task's signal group to the default autogroup.
///
/// # Safety
/// Caller pins p and must not hold its siglock.
#[no_mangle]
pub unsafe extern "C" fn sched_autogroup_detach(p: *mut task_struct) {
    // SAFETY: The default retains its boot reference.
    unsafe { autogroup_move_group(p, lupos_autogroup_default()) }
}

/// Inherit a reference to the current task's autogroup on signal creation.
///
/// # Safety
/// Caller exclusively initializes the new native signal structure.
#[no_mangle]
pub unsafe extern "C" fn sched_autogroup_fork(sig: *mut signal_struct) {
    // SAFETY: Only the new signal owns the acquired reference after assignment.
    unsafe {
        lupos_autogroup_signal_set(sig, autogroup_task_get(lupos_autogroup_current()));
    }
}

/// Drop the signal structure's reference during final native signal teardown.
///
/// # Safety
/// All signal users have exited; its owned reference is released exactly once.
#[no_mangle]
pub unsafe extern "C" fn sched_autogroup_exit(sig: *mut signal_struct) {
    // SAFETY: Caller transfers the signal's final autogroup reference.
    unsafe { autogroup_kref_put(lupos_autogroup_signal_get(sig)) }
}

/// Native __setup callback for noautogroup.
///
/// # Safety
/// Called only by boot argument processing before init memory is released.
#[export_name = "lupos_autogroup_setup"]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn setup_autogroup(_str: *mut c_char) -> c_int {
    // SAFETY: Early boot owns this setting; native sysctl storage has C layout.
    unsafe { lupos_autogroup_disable() };
    1
}

/// Change autogroup shares after the native permission and rate-limit checks.
///
/// # Safety
/// Caller pins p and supplies the native proc scheduler write context.
#[cfg(CONFIG_PROC_FS)]
#[no_mangle]
pub unsafe extern "C" fn proc_sched_autogroup_set_nice(
    p: *mut task_struct,
    nice: c_int,
) -> c_int {
    // SAFETY: Configured native primitives and shared state remain in C. The
    // next deadline deliberately retains native non-atomic rate semantics.
    unsafe {
        if nice < LUPOS_AUTOGROUP_MIN_NICE as c_int
            || nice > LUPOS_AUTOGROUP_MAX_NICE as c_int
        {
            return -(LUPOS_AUTOGROUP_EINVAL as c_int);
        }
        let mut err = lupos_autogroup_security_setnice(lupos_autogroup_current(), nice);
        if err != 0 {
            return err;
        }
        if nice < 0 && can_nice(lupos_autogroup_current(), nice) == 0 {
            return -(LUPOS_AUTOGROUP_EPERM as c_int);
        }
        if !lupos_autogroup_capable_admin()
            && lupos_autogroup_time_before(
                lupos_autogroup_jiffies(),
                lupos_autogroup_next_read(),
            )
        {
            return -(LUPOS_AUTOGROUP_EAGAIN as c_int);
        }
        lupos_autogroup_next_write(
            (LUPOS_AUTOGROUP_HZ as c_ulong / 10).wrapping_add(lupos_autogroup_jiffies()),
        );
        let ag = autogroup_task_get(p);
        let idx = lupos_autogroup_index_nospec(nice + 20);
        let shares = lupos_autogroup_scaled_weight(idx);
        lupos_autogroup_down_write(ag);
        err = sched_group_set_shares(lupos_autogroup_tg(ag), shares);
        if err == 0 {
            lupos_autogroup_set_nice(ag, nice);
        }
        lupos_autogroup_up_write(ag);
        autogroup_kref_put(ag);
        err
    }
}

/// Show a task's non-default autogroup under its native read semaphore.
///
/// # Safety
/// Caller pins p and supplies a writable seq_file in the native proc context.
#[cfg(CONFIG_PROC_FS)]
#[no_mangle]
pub unsafe extern "C" fn proc_sched_autogroup_show_task(
    p: *mut task_struct,
    m: *mut seq_file,
) {
    // SAFETY: The acquired reference pins ag through semaphore use and output.
    unsafe {
        let ag = autogroup_task_get(p);
        if lupos_autogroup_is_autogroup(lupos_autogroup_tg(ag)) {
            lupos_autogroup_down_read(ag);
            lupos_autogroup_show(m, ag);
            lupos_autogroup_up_read(ag);
        }
        autogroup_kref_put(ag);
    }
}

/// Format a non-default task group's autogroup path with native snprintf.
///
/// # Safety
/// Caller pins tg and its autogroup and supplies the native buffer contract.
#[no_mangle]
pub unsafe extern "C" fn autogroup_path(
    tg: *mut task_group,
    buf: *mut c_char,
    buflen: c_int,
) -> c_int {
    // SAFETY: Native formatting retains the original signed-length conversion
    // and original %ld representation of the group's unsigned-long id.
    unsafe {
        if !lupos_autogroup_is_autogroup(tg) {
            return 0;
        }
        lupos_autogroup_format_path(lupos_autogroup_tg_autogroup(tg), buf, buflen)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
