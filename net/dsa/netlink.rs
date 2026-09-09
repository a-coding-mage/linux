// SPDX-License-Identifier: GPL-2.0
/* Copyright 2022 NXP
 */

// Translated from netlink.c.  Linux and local header dependencies are supplied
// by the surrounding repository.

static DSA_POLICY: [nla_policy; (IFLA_DSA_MAX + 1) as usize] = {
    let mut policy = [nla_policy { type_: 0 }; (IFLA_DSA_MAX + 1) as usize];
    policy[IFLA_DSA_CONDUIT as usize] = nla_policy { type_: NLA_U32 };
    policy
};

unsafe fn dsa_changelink(
    dev: *mut net_device,
    _tb: *mut *mut nlattr,
    data: *mut *mut nlattr,
    extack: *mut netlink_ext_ack,
) -> c_int {
    let mut err: c_int;

    if data.is_null() {
        return 0;
    }

    if !(*data.add(IFLA_DSA_CONDUIT as usize)).is_null() {
        let ifindex: u32 = nla_get_u32(*data.add(IFLA_DSA_CONDUIT as usize));
        let conduit: *mut net_device;

        conduit = __dev_get_by_index(dev_net(dev), ifindex);
        if conduit.is_null() {
            return -EINVAL;
        }

        err = dsa_user_change_conduit(dev, conduit, extack);
        if err != 0 {
            return err;
        }
    }

    0
}

unsafe fn dsa_get_size(_dev: *const net_device) -> usize {
    nla_total_size(core::mem::size_of::<u32>()) + // IFLA_DSA_CONDUIT
        0
}

unsafe fn dsa_fill_info(skb: *mut sk_buff, dev: *const net_device) -> c_int {
    let conduit: *mut net_device = dsa_user_to_conduit(dev);

    if nla_put_u32(skb, IFLA_DSA_CONDUIT, (*conduit).ifindex) != 0 {
        return -EMSGSIZE;
    }

    0
}

#[no_mangle]
pub static mut dsa_link_ops: rtnl_link_ops = rtnl_link_ops {
    kind: b"dsa\0".as_ptr() as *const c_char,
    priv_size: core::mem::size_of::<dsa_port>(),
    maxtype: IFLA_DSA_MAX,
    policy: DSA_POLICY.as_ptr(),
    changelink: Some(dsa_changelink),
    get_size: Some(dsa_get_size),
    fill_info: Some(dsa_fill_info),
    netns_refund: true,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
