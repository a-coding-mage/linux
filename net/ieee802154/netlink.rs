// SPDX-License-Identifier: GPL-2.0-only
/*
 * Netlink interface for IEEE 802.15.4 stack
 *
 * Copyright 2007, 2008 Siemens AG
 *
 * Written by:
 * Sergey Lapin <slapin@ossfans.org>
 * Dmitry Eremin-Solenikov <dbaryshkov@gmail.com>
 * Maxim Osipov <maxim.osipov@siemens.com>
 */

// Kernel headers and ieee802154.h provide the types, constants, macros, and
// external symbols referenced below.

static mut ieee802154_seq_num: u32 = 0;
static mut ieee802154_seq_lock: Spinlock = Spinlock::new();

/* Requests to userspace */
unsafe fn ieee802154_nl_create(flags: i32, req: u8) -> *mut SkBuff {
    let hdr: *mut core::ffi::c_void;
    let msg: *mut SkBuff = nlmsg_new(NLMSG_DEFAULT_SIZE, GFP_ATOMIC);
    let mut f: CUnsignedLong = 0;

    if msg.is_null() {
        return core::ptr::null_mut();
    }

    spin_lock_irqsave(&raw mut ieee802154_seq_lock, &mut f);
    hdr = genlmsg_put(
        msg,
        0,
        ieee802154_seq_num,
        &raw mut nl802154_family,
        flags,
        req,
    );
    ieee802154_seq_num = ieee802154_seq_num.wrapping_add(1);
    spin_unlock_irqrestore(&raw mut ieee802154_seq_lock, f);
    if hdr.is_null() {
        nlmsg_free(msg);
        return core::ptr::null_mut();
    }

    msg
}

unsafe fn ieee802154_nl_mcast(msg: *mut SkBuff, group: u32) -> i32 {
    let nlh: *mut Nlmsghdr = nlmsg_hdr(msg);
    let hdr: *mut core::ffi::c_void = genlmsg_data(nlmsg_data(nlh));

    genlmsg_end(msg, hdr);

    genlmsg_multicast(&raw mut nl802154_family, msg, 0, group, GFP_ATOMIC)
}

unsafe fn ieee802154_nl_new_reply(
    info: *mut GenlInfo,
    flags: i32,
    req: u8,
) -> *mut SkBuff {
    let hdr: *mut core::ffi::c_void;
    let msg: *mut SkBuff = nlmsg_new(NLMSG_DEFAULT_SIZE, GFP_ATOMIC);

    if msg.is_null() {
        return core::ptr::null_mut();
    }

    hdr = genlmsg_put_reply(msg, info, &raw mut nl802154_family, flags, req);
    if hdr.is_null() {
        nlmsg_free(msg);
        return core::ptr::null_mut();
    }

    msg
}

unsafe fn ieee802154_nl_reply(msg: *mut SkBuff, info: *mut GenlInfo) -> i32 {
    let nlh: *mut Nlmsghdr = nlmsg_hdr(msg);
    let hdr: *mut core::ffi::c_void = genlmsg_data(nlmsg_data(nlh));

    genlmsg_end(msg, hdr);

    genlmsg_reply(msg, info)
}

// The following operation table is populated by the IEEE802154_OP,
// IEEE802154_DUMP, IEEE802154_DUMP_PRIV, and IEEE802154_OP_RELAXED macros.
// Those macros encode kernel-specific struct initializers and are preserved
// here as the corresponding source-level entries.
static ieee802154_ops: [GenlSmallOps; 24] = [
    ieee802154_dump!(IEEE802154_LIST_PHY, ieee802154_list_phy, ieee802154_dump_phy),
    ieee802154_op!(IEEE802154_ADD_IFACE, ieee802154_add_iface),
    ieee802154_op!(IEEE802154_DEL_IFACE, ieee802154_del_iface),
    ieee802154_op!(IEEE802154_ASSOCIATE_REQ, ieee802154_associate_req),
    ieee802154_op!(IEEE802154_ASSOCIATE_RESP, ieee802154_associate_resp),
    ieee802154_op!(IEEE802154_DISASSOCIATE_REQ, ieee802154_disassociate_req),
    ieee802154_op!(IEEE802154_SCAN_REQ, ieee802154_scan_req),
    ieee802154_op!(IEEE802154_START_REQ, ieee802154_start_req),
    ieee802154_dump!(IEEE802154_LIST_IFACE, ieee802154_list_iface, ieee802154_dump_iface),
    ieee802154_op!(IEEE802154_SET_MACPARAMS, ieee802154_set_macparams),
    ieee802154_op!(IEEE802154_LLSEC_GETPARAMS, ieee802154_llsec_getparams),
    ieee802154_op!(IEEE802154_LLSEC_SETPARAMS, ieee802154_llsec_setparams),
    ieee802154_dump_priv!(IEEE802154_LLSEC_LIST_KEY, core::ptr::null_mut(), ieee802154_llsec_dump_keys),
    ieee802154_op_relaxed!(IEEE802154_LLSEC_ADD_KEY, ieee802154_llsec_add_key),
    ieee802154_op_relaxed!(IEEE802154_LLSEC_DEL_KEY, ieee802154_llsec_del_key),
    ieee802154_dump_priv!(IEEE802154_LLSEC_LIST_DEV, core::ptr::null_mut(), ieee802154_llsec_dump_devs),
    ieee802154_op_relaxed!(IEEE802154_LLSEC_ADD_DEV, ieee802154_llsec_add_dev),
    ieee802154_op_relaxed!(IEEE802154_LLSEC_DEL_DEV, ieee802154_llsec_del_dev),
    ieee802154_dump_priv!(IEEE802154_LLSEC_LIST_DEVKEY, core::ptr::null_mut(), ieee802154_llsec_dump_devkeys),
    ieee802154_op_relaxed!(IEEE802154_LLSEC_ADD_DEVKEY, ieee802154_llsec_add_devkey),
    ieee802154_op_relaxed!(IEEE802154_LLSEC_DEL_DEVKEY, ieee802154_llsec_del_devkey),
    ieee802154_dump_priv!(IEEE802154_LLSEC_LIST_SECLEVEL, core::ptr::null_mut(), ieee802154_llsec_dump_seclevels),
    ieee802154_op_relaxed!(IEEE802154_LLSEC_ADD_SECLEVEL, ieee802154_llsec_add_seclevel),
    ieee802154_op_relaxed!(IEEE802154_LLSEC_DEL_SECLEVEL, ieee802154_llsec_del_seclevel),
];

static ieee802154_mcgrps: [GenlMulticastGroup; 2] = [
    GenlMulticastGroup { name: IEEE802154_MCAST_COORD_NAME },
    GenlMulticastGroup { name: IEEE802154_MCAST_BEACON_NAME },
];

static mut nl802154_family: GenlFamily = GenlFamily {
    hdrsize: 0,
    name: IEEE802154_NL_NAME,
    version: 1,
    maxattr: IEEE802154_ATTR_MAX,
    policy: ieee802154_policy,
    module: THIS_MODULE,
    small_ops: ieee802154_ops.as_ptr(),
    n_small_ops: ieee802154_ops.len(),
    resv_start_op: IEEE802154_LLSEC_DEL_SECLEVEL + 1,
    mcgrps: ieee802154_mcgrps.as_ptr(),
    n_mcgrps: ieee802154_mcgrps.len(),
};

unsafe fn ieee802154_nl_init() -> i32 {
    genl_register_family(&raw mut nl802154_family)
}

unsafe fn ieee802154_nl_exit() {
    genl_unregister_family(&raw mut nl802154_family);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
