// SPDX-License-Identifier: GPL-2.0
/*
 * Netlink routines for CIFS
 *
 * Copyright (c) 2020 Samuel Cabrero <scabrero@suse.de>
 */

// C dependencies: <net/genetlink.h>, <uapi/linux/cifs/cifs_netlink.h>,
// "netlink.h", "cifsglob.h", "cifs_debug.h", and "cifs_swn.h".

static CIFS_GENL_POLICY: [nla_policy; CIFS_GENL_ATTR_MAX + 1] = [
    nla_policy { type_: NLA_UNSPEC, len: 0 },
    nla_policy { type_: NLA_U32, len: 0 },
    nla_policy { type_: NLA_STRING, len: 0 },
    nla_policy { type_: NLA_STRING, len: 0 },
    nla_policy { type_: NLA_UNSPEC, len: core::mem::size_of::<sockaddr_storage>() },
    nla_policy { type_: NLA_FLAG, len: 0 },
    nla_policy { type_: NLA_FLAG, len: 0 },
    nla_policy { type_: NLA_FLAG, len: 0 },
    nla_policy { type_: NLA_FLAG, len: 0 },
    nla_policy { type_: NLA_STRING, len: 0 },
    nla_policy { type_: NLA_STRING, len: 0 },
    nla_policy { type_: NLA_STRING, len: 0 },
    nla_policy { type_: NLA_U32, len: 0 },
    nla_policy { type_: NLA_U32, len: 0 },
    nla_policy { type_: NLA_STRING, len: 0 },
];

static CIFS_GENL_OPS: [genl_ops; 1] = [genl_ops {
    cmd: CIFS_GENL_CMD_SWN_NOTIFY,
    flags: GENL_ADMIN_PERM,
    validate: GENL_DONT_VALIDATE_STRICT | GENL_DONT_VALIDATE_DUMP,
    doit: Some(cifs_swn_notify),
}];

static CIFS_GENL_MCGRPS: [genl_multicast_group; 1] = [genl_multicast_group {
    name: CIFS_GENL_MCGRP_SWN_NAME,
    flags: GENL_MCAST_CAP_NET_ADMIN,
}];

#[no_mangle]
pub static mut cifs_genl_family: genl_family = genl_family {
    name: CIFS_GENL_NAME,
    version: CIFS_GENL_VERSION,
    hdrsize: 0,
    maxattr: CIFS_GENL_ATTR_MAX,
    module: THIS_MODULE,
    policy: CIFS_GENL_POLICY.as_ptr(),
    ops: CIFS_GENL_OPS.as_ptr(),
    n_ops: CIFS_GENL_OPS.len(),
    resv_start_op: CIFS_GENL_CMD_SWN_NOTIFY + 1,
    mcgrps: CIFS_GENL_MCGRPS.as_ptr(),
    n_mcgrps: CIFS_GENL_MCGRPS.len(),
};

/**
 * cifs_genl_init - Register generic netlink family
 *
 * Return zero if initialized successfully, otherwise non-zero.
 */
#[no_mangle]
pub unsafe extern "C" fn cifs_genl_init() -> core::ffi::c_int {
    let ret: core::ffi::c_int;

    ret = genl_register_family(&raw mut cifs_genl_family);
    if ret < 0 {
        cifs_dbg(VFS, "%s: failed to register netlink family\\n", "cifs_genl_init");
        return ret;
    }

    0
}

/**
 * cifs_genl_exit - Unregister generic netlink family
 */
#[no_mangle]
pub unsafe extern "C" fn cifs_genl_exit() {
    let ret: core::ffi::c_int;

    ret = genl_unregister_family(&raw mut cifs_genl_family);
    if ret < 0 {
        cifs_dbg(VFS, "%s: failed to unregister netlink family\\n", "cifs_genl_exit");
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
