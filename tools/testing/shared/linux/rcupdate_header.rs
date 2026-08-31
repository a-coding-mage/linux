/* SPDX-License-Identifier: GPL-2.0 */

// C dependency intent: #include <urcu.h>
// The rcu_dereference symbol is supplied by that dependency in the original C header.

#[macro_export]
macro_rules! rcu_dereference_raw {
    ($p:expr) => {
        rcu_dereference!($p)
    };
}

#[macro_export]
macro_rules! rcu_dereference_protected {
    ($p:expr, $cond:expr) => {
        rcu_dereference!($p)
    };
}

#[macro_export]
macro_rules! rcu_dereference_check {
    ($p:expr, $cond:expr) => {
        rcu_dereference!($p)
    };
}

#[macro_export]
macro_rules! RCU_INIT_POINTER {
    ($p:expr, $v:expr) => {{
        $p = $v;
    }};
}
