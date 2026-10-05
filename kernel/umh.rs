// SPDX-License-Identifier: GPL-2.0-only
/*
 * umh - the kernel usermode helper
 */
// Inactive source repair. Native generation, ABI/CFI and build integration
// remain unqualified; see umh_native_bindings.h and umh_native_bindgen_parameters.
#![no_std]
compile_error!("SOURCE ONLY HOLD: usermode-helper ownership and native behavior are unqualified");

// Generated declarations inherit only the existing binding crate policy.
use kernel::bindings::umh_native as bindings;
use bindings::*;
use core::mem::MaybeUninit;
use core::ptr;
use kernel::ffi::{c_char, c_int, c_long, c_void};
#[cfg(CONFIG_SYSCTL)]
use kernel::ffi::c_ulong;
#[cfg(CONFIG_SYSCTL)]
use bindings::{
    LUPOS_UMH_CAP_SETPCAP as CAP_SETPCAP,
    LUPOS_UMH_CAP_SYS_MODULE as CAP_SYS_MODULE,
    LUPOS_UMH_EPERM as EPERM,
};
use bindings::{
    LUPOS_UMH_CLONE_PARENT as CLONE_PARENT,
    LUPOS_UMH_DISABLED as UMH_DISABLED,
    LUPOS_UMH_EAGAIN as EAGAIN,
    LUPOS_UMH_EBUSY as EBUSY,
    LUPOS_UMH_EINVAL as EINVAL,
    LUPOS_UMH_ENABLED as UMH_ENABLED,
    LUPOS_UMH_ENOMEM as ENOMEM,
    LUPOS_UMH_FREEZABLE as UMH_FREEZABLE,
    LUPOS_UMH_GFP_ATOMIC as GFP_ATOMIC,
    LUPOS_UMH_GFP_KERNEL as GFP_KERNEL,
    LUPOS_UMH_KILLABLE as UMH_KILLABLE,
    LUPOS_UMH_NO_WAIT as UMH_NO_WAIT,
    LUPOS_UMH_RUNNING_HELPERS_TIMEOUT as RUNNING_HELPERS_TIMEOUT,
    LUPOS_UMH_SIGCHLD as SIGCHLD,
    LUPOS_UMH_TASK_FREEZABLE as TASK_FREEZABLE,
    LUPOS_UMH_TASK_INTERRUPTIBLE as TASK_INTERRUPTIBLE,
    LUPOS_UMH_TASK_KILLABLE as TASK_KILLABLE,
    LUPOS_UMH_TASK_UNINTERRUPTIBLE as TASK_UNINTERRUPTIBLE,
    LUPOS_UMH_WAIT_PROC as UMH_WAIT_PROC,
};

unsafe fn call_usermodehelper_freeinfo(info: *mut subprocess_info) {
    // SAFETY: The caller exclusively owns the live native allocation and its
    // optional cleanup callback. Cleanup runs once before the allocation ends.
    unsafe {
        if let Some(cleanup) = lupos_umh_cleanup(info) {
            cleanup(info);
        }
        kfree(info as *mut c_void);
    }
}

unsafe fn umh_complete(sub_info: *mut subprocess_info) {
    // SAFETY: The producer owns the live info until this handoff. A nonnull
    // completion remains on the waiting caller's stack until complete finishes;
    // a null exchange transfers exclusive info ownership to this producer.
    unsafe {
        let comp = lupos_umh_xchg_complete(sub_info, ptr::null_mut());
        /*
         * See call_usermodehelper_exec(). If xchg() returns NULL
         * we own sub_info, the UMH_KILLABLE caller has gone away
         * or the caller used UMH_NO_WAIT.
         */
        if !comp.is_null() {
            complete(comp);
        } else {
            call_usermodehelper_freeinfo(sub_info);
        }
    }
}

/*
 * This is the task which runs the usermode application
 */
unsafe extern "C" fn call_usermodehelper_exec_async(data: *mut c_void) -> c_int {
    // SAFETY: The native thread receives live setup-owned info and runs in the
    // helper task context. Credential ownership is aborted or committed once;
    // capability fields are accessed under the native sysctl lock. Completing
    // can free info, so only the local retval is used after that call.
    unsafe {
        let sub_info = data as *mut subprocess_info;
        let new: *mut cred;
        let mut retval: c_int;
        'out: {
            let current = lupos_umh_current();
            lupos_umh_lock_siglock_irq(current);
            flush_signal_handlers(current, 1);
            lupos_umh_unlock_siglock_irq(current);

            /* Usermodehelper threads get a copy of userspace init's fs_struct. */
            lupos_umh_set_umask(current, 0o022);
            /* Avoid propagating elevated workqueue scheduling priority. */
            set_user_nice(current, 0);

            retval = -ENOMEM;
            new = prepare_kernel_cred(current);
            if new.is_null() { break 'out; }

            lupos_umh_sysctl_lock();
            let bset = lupos_umh_cred_bset(new);
            lupos_umh_cap_intersect(bset, lupos_umh_bset(), bset);
            let inheritable = lupos_umh_cred_inheritable(new);
            lupos_umh_cap_intersect(inheritable, lupos_umh_inheritable(), inheritable);
            lupos_umh_sysctl_unlock();

            if let Some(init) = lupos_umh_init(sub_info) {
                retval = init(sub_info, new);
                if retval != 0 {
                    abort_creds(new);
                    break 'out;
                }
            }

            commit_creds(new);
            lupos_umh_wait_for_initramfs();
            retval = kernel_execve(lupos_umh_path(sub_info),
                                   lupos_umh_argv(sub_info) as *const *const c_char,
                                   lupos_umh_envp(sub_info) as *const *const c_char);
        }

        lupos_umh_set_retval(sub_info, retval);
        /* call_usermodehelper_exec_sync() calls umh_complete for UMH_WAIT_PROC. */
        if lupos_umh_wait(sub_info) & UMH_WAIT_PROC == 0 { umh_complete(sub_info); }
        if retval == 0 { return 0; }
        do_exit(0);
    }
}

/* Handles UMH_WAIT_PROC. */
unsafe fn call_usermodehelper_exec_sync(sub_info: *mut subprocess_info) {
    // SAFETY: The work callback supplies live WAIT_PROC info. Its child defers
    // completion to this waiter; native kernel_wait synchronizes result access.
    unsafe {
        let pid: pid_t;
        lupos_umh_sigchld_default();
        pid = user_mode_thread(Some(call_usermodehelper_exec_async), sub_info as *mut c_void, SIGCHLD);
        if pid < 0 { lupos_umh_set_retval(sub_info, pid); }
        else { kernel_wait(pid, lupos_umh_retval_ptr(sub_info)); }
        lupos_umh_sigchld_ignore();
        umh_complete(sub_info);
    }
}

unsafe extern "C" fn call_usermodehelper_exec_work(work: *mut work_struct) {
    // SAFETY: Native INIT_WORK installs this callback in live subprocess_info;
    // queue_work publishes its initialized fields. Successful child creation
    // hands info to the child; failure is completed here without further use.
    unsafe {
        let sub_info = lupos_umh_from_work(work);
        if lupos_umh_wait(sub_info) & UMH_WAIT_PROC != 0 {
            call_usermodehelper_exec_sync(sub_info);
        } else {
            let pid = user_mode_thread(Some(call_usermodehelper_exec_async), sub_info as *mut c_void,
                                       CLONE_PARENT | SIGCHLD);
            if pid < 0 { lupos_umh_set_retval(sub_info, pid); umh_complete(sub_info); }
        }
    }
}

/// Acquires the helper read lock, waiting through the freezing state.
///
/// # Safety
/// The caller must be in sleepable task context and release a successful lock.
#[no_mangle]
pub unsafe extern "C" fn usermodehelper_read_trylock() -> c_int {
    // SAFETY: The native globals are initialized; the caller can sleep. The
    // waiter is initialized at its final address and unpublished by finish_wait
    // before its storage ends. Semaphore ownership follows each return path.
    unsafe {
        let mut wait_storage = MaybeUninit::<wait_queue_entry>::uninit();
        let wait = wait_storage.as_mut_ptr();
        // Initialize at its final address; the native list must never be moved.
        lupos_umh_init_wait(wait);
        let mut ret = 0;
        lupos_umh_down_read();
        loop {
            prepare_to_wait(lupos_umh_disabled_waitq(), wait, TASK_INTERRUPTIBLE as c_int);
            if lupos_umh_disabled() == UMH_ENABLED { break; }
            if lupos_umh_disabled() == UMH_DISABLED { ret = -EAGAIN; }
            lupos_umh_up_read();
            if ret != 0 { break; }
            schedule();
            lupos_umh_try_to_freeze();
            lupos_umh_down_read();
        }
        finish_wait(lupos_umh_disabled_waitq(), wait);
        ret
    }
}

/// Waits up to the supplied timeout for the helper read lock.
///
/// # Safety
/// The caller must be in sleepable task context and release any acquired lock.
/// As in the native API, zero input while already enabled still acquires it.
#[no_mangle]
pub unsafe extern "C" fn usermodehelper_read_lock_wait(mut timeout: c_long) -> c_long {
    if timeout < 0 { return -EINVAL as c_long; }
    // SAFETY: Task context and native semaphore ownership meet the caller's
    // contract. The in-place waiter never moves and finish_wait removes its
    // published link before returning, including the timeout path.
    unsafe {
        let mut wait_storage = MaybeUninit::<wait_queue_entry>::uninit();
        let wait = wait_storage.as_mut_ptr();
        lupos_umh_init_wait(wait);
        lupos_umh_down_read();
        loop {
            prepare_to_wait(lupos_umh_disabled_waitq(), wait, TASK_UNINTERRUPTIBLE as c_int);
            if lupos_umh_disabled() == UMH_ENABLED { break; }
            lupos_umh_up_read();
            timeout = schedule_timeout(timeout);
            if timeout == 0 { break; }
            lupos_umh_down_read();
        }
        finish_wait(lupos_umh_disabled_waitq(), wait);
        timeout
    }
}

/// Releases the helper read lock.
///
/// # Safety
/// The caller must own a read lock acquired by a helper read-lock operation.
#[no_mangle]
pub unsafe extern "C" fn usermodehelper_read_unlock() {
    // SAFETY: The caller supplies the matching native read-lock ownership.
    unsafe { lupos_umh_up_read(); }
}

/// Sets the native helper disable depth and wakes readers.
///
/// # Safety
/// The caller must be in sleepable context and obey the native disable policy.
#[no_mangle]
pub unsafe extern "C" fn __usermodehelper_set_disable_depth(depth: umh_disable_depth) {
    // SAFETY: Native state is initialized; the write semaphore serializes the
    // depth change, and the native waitqueue is permanent storage.
    unsafe {
        lupos_umh_down_write();
        lupos_umh_set_disabled(depth);
        lupos_umh_wake_up(lupos_umh_disabled_waitq());
        lupos_umh_up_write();
    }
}

/// Disables helpers and waits for the native running-helper count to drain.
///
/// # Safety
/// The caller must be in sleepable context and obey the native disable policy.
#[no_mangle]
pub unsafe extern "C" fn __usermodehelper_disable(depth: umh_disable_depth) -> c_int {
    if depth == UMH_ENABLED { return -EINVAL; }
    // SAFETY: The depth write is semaphore-protected and count operations are
    // native atomics. The caller can sleep; the in-place wait entry stays live
    // and unmoved until finish_wait removes it on every initialized path.
    unsafe {
        lupos_umh_down_write();
        lupos_umh_set_disabled(depth);
        lupos_umh_up_write();
        // Expand only the original wait_event_timeout primitive here, retaining
        // both condition evaluations and the success-at-timeout return of one.
        let mut retval = RUNNING_HELPERS_TIMEOUT;
        lupos_umh_might_sleep();
        let done = lupos_umh_running_read() == 0;
        if done && retval == 0 { retval = 1; }
        if !done && retval != 0 {
            let mut wait_storage = MaybeUninit::<wait_queue_entry>::uninit();
            let wait = wait_storage.as_mut_ptr();
            init_wait_entry(wait, 0);
            loop {
                prepare_to_wait_event(lupos_umh_running_waitq(), wait,
                                      TASK_UNINTERRUPTIBLE as c_int);
                let done = lupos_umh_running_read() == 0;
                if done && retval == 0 { retval = 1; }
                if done || retval == 0 { break; }
                retval = schedule_timeout(retval);
                let done = lupos_umh_running_read() == 0;
                if done && retval == 0 { retval = 1; }
                if done || retval == 0 { break; }
            }
            finish_wait(lupos_umh_running_waitq(), wait);
        }
        if retval != 0 { return 0; }
        __usermodehelper_set_disable_depth(UMH_ENABLED);
        -EAGAIN
    }
}

unsafe fn helper_lock() {
    // SAFETY: This starts one live execution's native counter ownership. The
    // barrier orders its increment before observing the helper disable depth.
    unsafe { lupos_umh_running_inc(); lupos_umh_mb_after_atomic(); }
}
unsafe fn helper_unlock() {
    // SAFETY: The caller owns exactly one running-counter increment. Native
    // decrement/wakeup touches only initialized permanent storage.
    unsafe {
        if lupos_umh_running_dec_and_test() { lupos_umh_wake_up(lupos_umh_running_waitq()); }
    }
}

// The public native thunk preserves the nullable callback parameters' CFI
// identity; it passes a borrowed native carrier only for this setup call.
/// Implements setup behind the native nullable-callback signature adapter.
///
/// # Safety
/// The callback carrier must be valid for this call, with native nullable
/// function pointers. The caller must satisfy the native allocation/context
/// rules and keep path, vectors, callback targets and data valid as required
/// until execution/cleanup ends. A null path retains the native error policy.
#[export_name = "lupos_umh_setup_body"]
pub unsafe extern "C" fn call_usermodehelper_setup(_path: *const c_char, argv: *mut *mut c_char,
    envp: *mut *mut c_char, gfp_mask: gfp_t,
    callbacks: *const lupos_umh_callbacks, data: *mut c_void) -> *mut subprocess_info {
    // SAFETY: Successful native allocation is exclusively owned until returned.
    // All field accesses use configured C layout. The valid callback carrier is
    // read synchronously and never stored; callback values retain caller-owned
    // lifetimes, and the configured static path has permanent native storage.
    unsafe {
        let sub_info = lupos_umh_alloc(gfp_mask);
        if sub_info.is_null() { return ptr::null_mut(); }
        lupos_umh_init_work(lupos_umh_work(sub_info), Some(call_usermodehelper_exec_work));
        #[cfg(CONFIG_STATIC_USERMODEHELPER)]
        { lupos_umh_set_path(sub_info, lupos_umh_static_path()); }
        #[cfg(not(CONFIG_STATIC_USERMODEHELPER))]
        { lupos_umh_set_path(sub_info, _path); }
        lupos_umh_set_argv(sub_info, argv);
        lupos_umh_set_envp(sub_info, envp);
        lupos_umh_set_cleanup(sub_info, lupos_umh_setup_cleanup(callbacks));
        lupos_umh_set_init(sub_info, lupos_umh_setup_init(callbacks));
        lupos_umh_set_data(sub_info, data);
        sub_info
    }
}

/// Starts the prepared helper and consumes its native setup allocation.
///
/// # Safety
/// The caller must transfer one live setup allocation, keep its referenced
/// inputs/callbacks valid through cleanup, and obey the requested wait mode's
/// task-context requirements. The info must not be used after this call.
#[no_mangle]
pub unsafe extern "C" fn call_usermodehelper_exec(sub_info: *mut subprocess_info, wait: c_int) -> c_int {
    // SAFETY: The caller transfers live info and valid inputs. Native queue_work
    // publishes fields; native completion synchronizes result reads. Stack
    // completion storage is initialized in place and never moved. No-wait and
    // interrupted ownership-transfer paths return without using info again;
    // if the producer already took the completion, the caller waits before its
    // stack storage ends and before it exclusively cleans up info.
    unsafe {
        let mut state = TASK_UNINTERRUPTIBLE;
        let mut done_storage = MaybeUninit::<completion>::uninit();
        let done = done_storage.as_mut_ptr();
        // Native initialization in place; only raw pointers cross the concurrent
        // completion boundary, and done_storage stays here until ownership ends.
        lupos_umh_init_completion(done);
        let mut retval = 0;
        'out: {
            'wait_done: {
                if lupos_umh_path(sub_info).is_null() { call_usermodehelper_freeinfo(sub_info); return -EINVAL; }
                helper_lock();
                if lupos_umh_disabled() != UMH_ENABLED { retval = -EBUSY; break 'out; }
                if lupos_umh_strlen(lupos_umh_path(sub_info)) == 0 { break 'out; }
                lupos_umh_set_complete(sub_info, if wait == UMH_NO_WAIT { ptr::null_mut() } else { done });
                lupos_umh_set_wait(sub_info, wait);
                lupos_umh_queue_work(lupos_umh_work(sub_info));
                if wait == UMH_NO_WAIT { helper_unlock(); return retval; }
                if wait & UMH_FREEZABLE != 0 { state |= TASK_FREEZABLE; }
                if wait & UMH_KILLABLE != 0 {
                    retval = wait_for_completion_state(done, state | TASK_KILLABLE);
                    if retval == 0 { break 'wait_done; }
                    if !lupos_umh_xchg_complete(sub_info, ptr::null_mut()).is_null() { helper_unlock(); return retval; }
                }
                wait_for_completion_state(done, state);
            }

            retval = lupos_umh_retval(sub_info);
        }

        call_usermodehelper_freeinfo(sub_info);
        helper_unlock();
        retval
    }
}

/// Allocates and starts a helper using the native wait-mode allocation policy.
///
/// # Safety
/// The caller must supply native path/vector storage valid for the requested
/// asynchronous or synchronous execution and satisfy its context requirements.
#[no_mangle]
pub unsafe extern "C" fn call_usermodehelper(path: *const c_char, argv: *mut *mut c_char,
    envp: *mut *mut c_char, wait: c_int) -> c_int {
    let gfp_mask = if wait == UMH_NO_WAIT { GFP_ATOMIC } else { GFP_KERNEL };
    // SAFETY: The caller guarantees input lifetime and context. Null callbacks
    // are accepted by native setup; successful allocation is transferred once
    // to exec and is never accessed by this wrapper afterward.
    unsafe {
        let info = bindings::call_usermodehelper_setup(path, argv, envp, gfp_mask, None, None, ptr::null_mut());
        if info.is_null() { return -ENOMEM; }
        call_usermodehelper_exec(info, wait)
    }
}

// Native two-row table and early-init metadata are in the owner-local C files.
#[cfg(CONFIG_SYSCTL)]
#[export_name = "lupos_umh_proc_cap_handler"]
unsafe extern "C" fn proc_cap_handler(table: *const ctl_table, write: c_int, buffer: *mut c_void,
                           lenp: *mut usize, ppos: *mut loff_t) -> c_int {
    // SAFETY: Native sysctl dispatch supplies the registered permanent table
    // and its validated handler arguments. Stack scratch is initialized by
    // native copy/set before use and never retained by registration or parsing.
    // Capability reads/restrictions use the native lock; authorization precedes
    // table access and parser failure returns before any global mutation.
    unsafe {
        let mut t_storage = MaybeUninit::<ctl_table>::uninit();
        let t = t_storage.as_mut_ptr();
        let mut cap_array = [0 as c_ulong; 2];
        let mut new_cap_storage = MaybeUninit::<kernel_cap_t>::uninit();
        let new_cap = new_cap_storage.as_mut_ptr();
        if write != 0 && (!lupos_umh_capable(CAP_SETPCAP) || !lupos_umh_capable(CAP_SYS_MODULE)) { return -EPERM; }
        let cap = lupos_umh_table_cap(table);
        lupos_umh_sysctl_lock();
        cap_array[0] = lupos_umh_cap_value(cap) as u32 as c_ulong;
        cap_array[1] = (lupos_umh_cap_value(cap) >> 32) as c_ulong;
        lupos_umh_sysctl_unlock();
        lupos_umh_copy_table(t, table);
        lupos_umh_set_table_data(t, cap_array.as_mut_ptr() as *mut c_void);
        let err = proc_doulongvec_minmax(t, write, buffer, lenp, ppos);
        if err < 0 { return err; }
        let value = (cap_array[0] as u32 as u64) + ((cap_array[1] as u64) << 32);
        lupos_umh_set_cap_value(new_cap, value);
        if write != 0 {
            lupos_umh_sysctl_lock();
            lupos_umh_cap_intersect(cap, cap, new_cap);
            lupos_umh_sysctl_unlock();
        }
        0
    }
}

#[cfg(CONFIG_SYSCTL)]
#[export_name = "lupos_umh_init_sysctls"]
#[link_section = ".init.text"]
unsafe extern "C" fn init_umh_sysctls() -> c_int {
    // SAFETY: The native early-init entry calls this while init text is live;
    // registration retains only the permanent native table and runtime handler.
    unsafe { lupos_umh_register_sysctl(); }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
