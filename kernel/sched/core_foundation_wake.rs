// SPDX-License-Identifier: GPL-2.0-only
// The cmpxchg leaf must expand the native relaxed macro, not a C owner body.
unsafe fn __wake_q_add(head: *mut wake_q_head, task: *mut task_struct) -> bool {
    // SAFETY: The caller exclusively owns the initialized queue head and keeps
    // task live. The native compare-exchange arbitrates ownership of its node.
    unsafe {
        let node = addr_of_mut!((*task).wake_q);
        lupos_core_smp_mb_before_atomic();
        if !lupos_core_cmpxchg_wake_relaxed(
            addr_of_mut!((*node).next),
            null_mut(),
            lupos_core_wake_q_tail(),
        )
        .is_null()
        {
            return false;
        }
        *(*head).lastp = node;
        (*head).lastp = addr_of_mut!((*node).next);
        true
    }
}
#[no_mangle]
pub unsafe extern "C" fn wake_q_add(head: *mut wake_q_head, task: *mut task_struct) {
    // SAFETY: The caller keeps task live and ready to wake, and exclusively
    // owns head; a successful enqueue takes the reference used by wake_up_q.
    unsafe {
        if __wake_q_add(head, task) {
            lupos_core_get_task_struct(task);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wake_q_add_safe(head: *mut wake_q_head, task: *mut task_struct) {
    // SAFETY: The caller transfers a task reference and exclusively owns head.
    // A duplicate drops that reference; a successful enqueue retains it.
    unsafe {
        if !__wake_q_add(head, task) {
            lupos_core_put_task_struct(task);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wake_up_q(head: *mut wake_q_head) {
    // SAFETY: The caller exclusively owns the populated wake queue. Each node
    // has a task reference; its next link is saved before waking and releasing it.
    unsafe {
        let mut node = (*head).first;
        while node != lupos_core_wake_q_tail() {
            let task = node
                .cast::<u8>()
                .sub(offset_of!(task_struct, wake_q))
                .cast::<task_struct>();
            node = (*node).next;
            lupos_core_write_once_wake(addr_of_mut!((*task).wake_q.next), null_mut());
            wake_up_process(task);
            lupos_core_put_task_struct(task);
        }
    }
}
