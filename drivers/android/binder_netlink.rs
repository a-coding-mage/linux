// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)
/* Do not edit directly, auto-generated from: */
/* Documentation/netlink/specs/binder.yaml */
/* YNL-GEN kernel source */
/* To regenerate run: tools/net/ynl/ynl-regen.sh */

// Dependencies supplied by the kernel netlink/genetlink headers and
// binder_netlink.h/uapi/linux/android/binder_netlink.h.

/* Ops table for binder */
static BINDER_NL_OPS: [genl_split_ops; 0] = [];

static BINDER_NL_MCGRPS: [genl_multicast_group; 1] = [genl_multicast_group {
    name: b"report\0".as_ptr() as *const _,
}];

// __ro_after_init
pub static mut binder_nl_family: genl_family = genl_family {
    name: BINDER_FAMILY_NAME,
    version: BINDER_FAMILY_VERSION,
    netnsok: true,
    parallel_ops: true,
    module: THIS_MODULE,
    split_ops: BINDER_NL_OPS.as_ptr(),
    n_split_ops: BINDER_NL_OPS.len(),
    mcgrps: BINDER_NL_MCGRPS.as_ptr(),
    n_mcgrps: BINDER_NL_MCGRPS.len(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
