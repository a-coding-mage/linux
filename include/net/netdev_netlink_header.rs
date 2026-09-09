/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied externally by <linux/list.h> and related declarations.

#[repr(C)]
pub struct netdev_nl_sock {
    pub lock: mutex,
    pub bindings: list_head,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
