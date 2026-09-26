// SPDX-License-Identifier: GPL-2.0-only
//! Sole owner of the original init-memory kthreadd startup completion.

use super::bindings;

const fn initial_completion() -> bindings::completion {
    // SAFETY: completion, its raw lock and optional lockdep metadata consist of
    // integers, raw pointers and nullable callbacks, all admitting zero. The
    // original initializer's nonzero values and relocations are assigned below.
    let mut value: bindings::completion = unsafe { core::mem::zeroed() };
    #[cfg(CONFIG_SMP)]
    {
        // Both supported boot architectures use the canonical queued spinlock
        // initializer. Naming its actual member also prevents silently treating
        // an unrelated architecture lock representation as this zero state.
        value.wait.lock.raw_lock.__bindgen_anon_1.val.counter = 0;
    }
    #[cfg(all(not(CONFIG_SMP), CONFIG_DEBUG_SPINLOCK))]
    {
        // UP debug locks have the opposite unlocked representation from qspin.
        value.wait.lock.raw_lock.slock = 1;
    }
    #[cfg(CONFIG_DEBUG_SPINLOCK)]
    {
        value.wait.lock.magic = bindings::SPINLOCK_MAGIC;
        value.wait.lock.owner_cpu = kernel::ffi::c_uint::MAX;
        value.wait.lock.owner = kernel::ffi::c_ulong::MAX as *mut kernel::ffi::c_void;
    }
    #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
    {
        value.wait.lock.dep_map.name = c"(kthreadd_done).wait.lock".as_ptr().cast();
        value.wait.lock.dep_map.wait_type_inner = bindings::lockdep_wait_type_LD_WAIT_SPIN as u8;
    }
    // SAFETY: only addresses of the sole static owner's fields are formed.
    // Nothing is read before initialization; the head points to its own list.
    let head = unsafe { core::ptr::addr_of_mut!(kthreadd_done.wait.task_list) };
    value.wait.task_list.next = head;
    value.wait.task_list.prev = head;
    value
}

#[used]
#[link_section = ".init.data"]
#[linkage = "internal"]
pub(super) static mut kthreadd_done: bindings::completion = initial_completion();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
