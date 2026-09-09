// SPDX-License-Identifier: GPL-2.0-or-later

// C header guard: __DSA_NETLINK_H

// External dependency: `rtnl_link_ops` is supplied by another translation unit.
extern "C" {
    pub static mut dsa_link_ops: crate::rtnl_link_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
