/* SPDX-License-Identifier: GPL-2.0+ */

// C includes removed: "shared.h", <stdlib.h>, <time.h>, "linux/init.h".
// Expects external definitions for rcu_head, call_rcu, and MAPLE_NODE_SLOTS.

pub const CONFIG_DEBUG_MAPLE_TREE: bool = true;
pub const CONFIG_MAPLE_SEARCH: bool = true;
pub const MAPLE_32BIT: bool = MAPLE_NODE_SLOTS > 31;

unsafe extern "C" {
    pub fn maple_rcu_cb(head: *mut rcu_head);
}

pub use maple_rcu_cb as rcu_cb;

macro_rules! kfree_rcu {
    ($_struct:expr, $_memb:ident) => {{
        let _p_struct = $_struct;
        call_rcu(
            unsafe { &mut (*_p_struct).$_memb as *mut _ },
            rcu_cb,
        );
    }};
}


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
