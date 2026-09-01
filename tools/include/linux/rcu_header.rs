/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_int;

pub static mut rcu_scheduler_active: c_int = 0;

#[inline]
pub fn rcu_lockdep_current_cpu_online() -> c_int {
    1
}

#[inline]
pub fn rcu_is_cpu_idle() -> c_int {
    1
}

#[inline]
pub fn rcu_is_watching() -> bool {
    false
}

macro_rules! rcu_assign_pointer {
    ($p:expr, $v:expr) => {{
        $p = $v;
    }};
}

macro_rules! RCU_INIT_POINTER {
    ($p:expr, $v:expr) => {{
        $p = $v;
    }};
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
