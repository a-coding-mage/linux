/* SPDX-License-Identifier: GPL-2.0 */

/*
 * RCU synchronization types and methods.
 *
 * C dependencies supplied by the surrounding kernel translation unit:
 * linux/rcupdate.h, linux/completion.h, and linux/sched.h.
 */

/* Opaque/kernel-provided types and values. */
#[repr(C)]
pub struct rcu_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct completion {
    _private: [u8; 0],
}

pub type rcu_gp_seq = usize;
pub type call_rcu_func_t = unsafe extern "C" fn(*mut rcu_head);

/*
 * Structure allowing asynchronous waiting on RCU.
 */
#[repr(C)]
pub struct rcu_synchronize {
    pub head: rcu_head,
    pub completion: completion,

    /* This is for debugging. */
    pub oldstate: rcu_gp_seq,
}

unsafe extern "C" {
    pub fn wakeme_after_rcu(head: *mut rcu_head);

    pub fn __wait_rcu_gp(
        checktiny: bool,
        state: ::core::ffi::c_uint,
        n: ::core::ffi::c_int,
        crcu_array: *mut call_rcu_func_t,
        rs_array: *mut rcu_synchronize,
    );
}

/*
 * Translation of the C _wait_rcu_gp() macro.  The state constants and
 * CONFIG-dependent expressions are supplied by the surrounding kernel.
 */
#[macro_export]
macro_rules! _wait_rcu_gp {
    ($checktiny:expr, $state:expr, $($crcu:expr),+ $(,)?) => {{
        let mut __crcu_array: [call_rcu_func_t; _] = [$($crcu),+];
        let mut __rs_array: [rcu_synchronize; __crcu_array.len()] =
            [unsafe { ::core::mem::zeroed() }; __crcu_array.len()];
        unsafe {
            __wait_rcu_gp(
                $checktiny,
                $state,
                __crcu_array.len() as ::core::ffi::c_int,
                __crcu_array.as_mut_ptr(),
                __rs_array.as_mut_ptr(),
            );
        }
    }};
}

#[macro_export]
macro_rules! wait_rcu_gp {
    ($($crcu:expr),+ $(,)?) => {
        $crate::_wait_rcu_gp!(false, TASK_UNINTERRUPTIBLE, $($crcu),+)
    };
}

#[macro_export]
macro_rules! wait_rcu_gp_state {
    ($state:expr, $($crcu:expr),+ $(,)?) => {
        $crate::_wait_rcu_gp!(false, $state, $($crcu),+)
    };
}

/**
 * synchronize_rcu_mult - Wait concurrently for multiple grace periods
 * @...: List of call_rcu() functions for different grace periods to wait on
 *
 * This macro waits concurrently for multiple types of RCU grace periods.
 * The detailed usage and CONFIG_RCU_LAZY caveat are retained from the C
 * interface; CONFIG_TINY_RCU is evaluated by the surrounding build.
 */
#[macro_export]
macro_rules! synchronize_rcu_mult {
    ($($crcu:expr),+ $(,)?) => {
        $crate::_wait_rcu_gp!(IS_ENABLED(CONFIG_TINY_RCU), TASK_UNINTERRUPTIBLE, $($crcu),+)
    };
}

#[inline]
pub unsafe fn cond_resched_rcu() {
    /* CONFIG_DEBUG_ATOMIC_SLEEP || !CONFIG_PREEMPT_RCU */
    #[cfg(any(feature = "CONFIG_DEBUG_ATOMIC_SLEEP", not(feature = "CONFIG_PREEMPT_RCU")))]
    {
        rcu_read_unlock();
        cond_resched();
        rcu_read_lock();
    }
}

// Has the current task blocked within its current RCU read-side
// critical section?
#[inline]
pub unsafe fn has_rcu_reader_blocked() -> bool {
    /* CONFIG_PREEMPT_RCU */
    #[cfg(feature = "CONFIG_PREEMPT_RCU")]
    {
        return !list_empty(&(*current).rcu_node_entry);
    }
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
