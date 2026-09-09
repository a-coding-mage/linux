/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: <linux/mutex.h>, <linux/types.h>
// The referenced types are supplied by other translated dependencies.

#[repr(C)]
pub struct netns_xdp {
    pub lock: mutex,
    pub list: hlist_head,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
