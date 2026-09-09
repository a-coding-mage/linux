/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by other translated headers:
// linux/atomic.h, linux/lockdep_types.h, linux/timer_types.h, linux/types.h

#[repr(C)]
pub struct workqueue_struct {
    _private: [u8; 0],
}

pub type work_func_t = Option<unsafe extern "C" fn(work: *mut work_struct)>;

extern "C" {
    pub fn delayed_work_timer_fn(t: *mut timer_list);
}

pub struct work_struct {
    pub data: atomic_long_t,
    pub entry: list_head,
    pub func: work_func_t,
    // Preserved from: #ifdef CONFIG_LOCKDEP
    #[cfg(CONFIG_LOCKDEP)]
    pub lockdep_map: lockdep_map,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
