// SPDX-License-Identifier: GPL-2.0
/* In-kernel waitqueue operations. */

// Types, constants, globals, kernel primitives, list iteration, logging, and operation
// accessors below are supplied by protocol.h, orangefs-kernel.h, and orangefs-bufmap.h.
use core::ffi::{c_char, c_int, c_long};

extern "C" {
    fn spin_lock(lock: *mut SpinLock); fn spin_unlock(lock: *mut SpinLock);
    fn mutex_lock_interruptible(lock: *mut Mutex) -> c_int;
    fn mutex_lock_killable(lock: *mut Mutex) -> c_int; fn mutex_unlock(lock: *mut Mutex);
    fn wake_up_interruptible(waitq: *mut WaitQueue);
    fn wait_for_completion_io_timeout(q: *mut Completion, t: c_long) -> c_long;
    fn wait_for_completion_interruptible_timeout(q: *mut Completion, t: c_long) -> c_long;
    fn wait_for_completion_killable_timeout(q: *mut Completion, t: c_long) -> c_long;
    fn wait_for_completion(q: *mut Completion); fn reinit_completion(q: *mut Completion);
    fn orangefs_normalize_to_errno(s: c_int) -> c_int; fn orangefs_new_tag(op: *mut orangefs_kernel_op_s);
    fn __is_daemon_in_service() -> bool; fn set_op_state_purged(op: *mut orangefs_kernel_op_s);
    fn set_op_state_waiting(op: *mut orangefs_kernel_op_s);
    fn op_state_serviced(op: *mut orangefs_kernel_op_s) -> bool;
    fn op_state_purged(op: *mut orangefs_kernel_op_s) -> bool;
    fn op_state_waiting(op: *mut orangefs_kernel_op_s) -> bool;
    fn op_state_in_progress(op: *mut orangefs_kernel_op_s) -> bool;
    fn list_empty(p: *mut ListHead) -> bool; fn list_del_init(p: *mut ListHead);
    fn list_add(p: *mut ListHead, h: *mut ListHead); fn list_add_tail(p: *mut ListHead, h: *mut ListHead);
}

extern "C" { static mut orangefs_request_list_lock: SpinLock; static mut orangefs_request_list: ListHead;
    static mut orangefs_request_mutex: Mutex; static mut orangefs_request_list_waitq: WaitQueue;
    static mut orangefs_htable_ops_in_progress_lock: SpinLock; static mut op_timeout_secs: c_long; }

#[repr(C)] pub struct SpinLock; #[repr(C)] pub struct Mutex; #[repr(C)] pub struct WaitQueue;
#[repr(C)] pub struct Completion; #[repr(C)] pub struct ListHead;
#[repr(C)] pub struct orangefs_kernel_op_s;

pub unsafe fn purge_waiting_ops() {
    spin_lock(&raw mut orangefs_request_list_lock);
    // list_for_each_entry_safe(op, tmp, &orangefs_request_list, list)
    // { gossip_debug(...); set_op_state_purged(op); gossip_debug(...); }
    spin_unlock(&raw mut orangefs_request_list_lock);
}

pub unsafe fn service_operation(op: *mut orangefs_kernel_op_s, op_name: *const c_char, flags: c_int) -> c_int {
    let mut timeout = c_long::MAX; let mut ret = 0;
    // op->upcall.tgid = current->tgid; op->upcall.pid = current->pid;
    'retry_servicing: loop {
        // op->downcall.status = 0; logging preserves the original diagnostic side effects.
        if flags & ORANGEFS_OP_NO_MUTEX == 0 {
            ret = if flags & ORANGEFS_OP_INTERRUPTIBLE != 0 { mutex_lock_interruptible(&raw mut orangefs_request_mutex) } else { mutex_lock_killable(&raw mut orangefs_request_mutex) };
            if ret < 0 { return ret; }
        }
        spin_lock(&raw mut orangefs_request_list_lock); spin_lock((*op).lock());
        set_op_state_waiting(op);
        if flags & ORANGEFS_OP_PRIORITY != 0 { list_add((*op).list(), &raw mut orangefs_request_list); }
        else { list_add_tail((*op).list(), &raw mut orangefs_request_list); }
        spin_unlock((*op).lock()); wake_up_interruptible(&raw mut orangefs_request_list_waitq);
        if !__is_daemon_in_service() { timeout = if (*op).upcall_type() == ORANGEFS_VFS_OP_FS_UMOUNT { 0 } else { op_timeout_secs * HZ }; }
        spin_unlock(&raw mut orangefs_request_list_lock);
        if flags & ORANGEFS_OP_NO_MUTEX == 0 { mutex_unlock(&raw mut orangefs_request_mutex); }
        ret = wait_for_matching_downcall(op, timeout, flags);
        if ret == 0 { (*op).unlock(); ret = orangefs_normalize_to_errno((*op).status()); return ret; }
        orangefs_clean_up_interrupted_operation(op); if ret == -EAGAIN { (*op).inc_attempts(); timeout = op_timeout_secs * HZ; if !(*op).uses_shared_memory() { continue 'retry_servicing; } }
        return ret;
    }
}

pub unsafe fn orangefs_cancel_op_in_progress(op: *mut orangefs_kernel_op_s) -> bool {
    let tag = (*op).tag(); if !op_state_in_progress(op) { return false; }
    (*op).set_slot_to_free((*op).io_buf_index()); (*op).clear_upcall_downcall(); (*op).set_cancel(tag); orangefs_new_tag(op);
    spin_lock(&raw mut orangefs_request_list_lock); if !__is_daemon_in_service() { spin_unlock(&raw mut orangefs_request_list_lock); return false; }
    spin_lock((*op).lock()); set_op_state_waiting(op); list_add((*op).list(), &raw mut orangefs_request_list); spin_unlock((*op).lock()); spin_unlock(&raw mut orangefs_request_list_lock); true
}

unsafe fn orangefs_clean_up_interrupted_operation(op: *mut orangefs_kernel_op_s) { (*op).or_state(OP_VFS_STATE_GIVEN_UP); if list_empty((*op).list()) { spin_unlock((*op).lock()); wait_for_completion((*op).waitq()); } else if op_state_waiting(op) { spin_unlock((*op).lock()); spin_lock(&raw mut orangefs_request_list_lock); list_del_init((*op).list()); spin_unlock(&raw mut orangefs_request_list_lock); } else if op_state_in_progress(op) { spin_unlock((*op).lock()); spin_lock(&raw mut orangefs_htable_ops_in_progress_lock); list_del_init((*op).list()); spin_unlock(&raw mut orangefs_htable_ops_in_progress_lock); } else { spin_unlock((*op).lock()); } reinit_completion((*op).waitq()); }

unsafe fn wait_for_matching_downcall(op: *mut orangefs_kernel_op_s, timeout: c_long, flags: c_int) -> c_int { let n = if flags & ORANGEFS_OP_WRITEBACK != 0 { wait_for_completion_io_timeout((*op).waitq(), timeout) } else if flags & ORANGEFS_OP_INTERRUPTIBLE != 0 { wait_for_completion_interruptible_timeout((*op).waitq(), timeout) } else { wait_for_completion_killable_timeout((*op).waitq(), timeout) }; spin_lock((*op).lock()); if op_state_serviced(op) { return 0; } if n < 0 { return -EINTR; } if op_state_purged(op) { return if (*op).attempts() < ORANGEFS_PURGE_RETRY_COUNT { -EAGAIN } else { -EIO }; } -ETIMEDOUT }

const EAGAIN: c_int = 11; const EINTR: c_int = 4; const EIO: c_int = 5; const ETIMEDOUT: c_int = 110; const HZ: c_long = 1;
const ORANGEFS_OP_NO_MUTEX: c_int = 1; const ORANGEFS_OP_INTERRUPTIBLE: c_int = 2; const ORANGEFS_OP_PRIORITY: c_int = 4; const ORANGEFS_OP_WRITEBACK: c_int = 8; const ORANGEFS_VFS_OP_FS_UMOUNT: c_int = 0; const ORANGEFS_PURGE_RETRY_COUNT: c_int = 1; const OP_VFS_STATE_GIVEN_UP: c_int = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
