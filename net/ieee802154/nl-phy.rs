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

// Linux kernel and local header dependencies are supplied by the surrounding translation.

unsafe fn ieee802154_nl_fill_phy(
    msg: *mut sk_buff,
    _portid: u32,
    seq: u32,
    flags: i32,
    phy: *mut wpan_phy,
) -> i32 {
    let mut hdr: *mut core::ffi::c_void;
    let mut pages: i32 = 0;
    let buf = kcalloc(
        (IEEE802154_MAX_PAGE + 1) as usize,
        core::mem::size_of::<u32>(),
        GFP_KERNEL,
    ) as *mut u32;

    pr_debug!("%s\n", "ieee802154_nl_fill_phy");

    if buf.is_null() {
        return -EMSGSIZE;
    }

    hdr = genlmsg_put(msg, 0, seq, &nl802154_family, flags, IEEE802154_LIST_PHY);
    if hdr.is_null() {
        kfree(buf as *mut core::ffi::c_void);
        return -EMSGSIZE;
    }

    rtnl_lock();
    if nla_put_string(msg, IEEE802154_ATTR_PHY_NAME, wpan_phy_name(phy)) != 0
        || nla_put_u8(msg, IEEE802154_ATTR_PAGE, (*phy).current_page) != 0
        || nla_put_u8(msg, IEEE802154_ATTR_CHANNEL, (*phy).current_channel) != 0
    {
        rtnl_unlock();
        genlmsg_cancel(msg, hdr);
        kfree(buf as *mut core::ffi::c_void);
        return -EMSGSIZE;
    }
    for i in 0..=(IEEE802154_MAX_PAGE as usize) {
        if (*phy).supported.channels[i] != 0 {
            *buf.add(pages as usize) = (*phy).supported.channels[i] | ((i as u32) << 27);
            pages += 1;
        }
    }
    if pages != 0
        && nla_put(
            msg,
            IEEE802154_ATTR_CHANNEL_PAGE_LIST,
            pages as usize * core::mem::size_of::<u32>(),
            buf as *const core::ffi::c_void,
        ) != 0
    {
        rtnl_unlock();
        genlmsg_cancel(msg, hdr);
        kfree(buf as *mut core::ffi::c_void);
        return -EMSGSIZE;
    }
    rtnl_unlock();
    kfree(buf as *mut core::ffi::c_void);
    genlmsg_end(msg, hdr);
    0
}

pub unsafe fn ieee802154_list_phy(_skb: *mut sk_buff, info: *mut genl_info) -> i32 {
    let mut msg: *mut sk_buff;
    let phy: *mut wpan_phy;
    let name: *const core::ffi::c_char;
    let mut rc: i32 = -ENOBUFS;

    pr_debug!("%s\n", "ieee802154_list_phy");
    if (*info).attrs[IEEE802154_ATTR_PHY_NAME].is_null() {
        return -EINVAL;
    }
    name = nla_data((*info).attrs[IEEE802154_ATTR_PHY_NAME]) as *const core::ffi::c_char;
    if *name.add(nla_len((*info).attrs[IEEE802154_ATTR_PHY_NAME]) - 1) as u8 != 0 {
        return -EINVAL;
    }
    phy = wpan_phy_find(name);
    if phy.is_null() {
        return -ENODEV;
    }
    msg = nlmsg_new(NLMSG_DEFAULT_SIZE, GFP_KERNEL);
    if msg.is_null() {
        wpan_phy_put(phy);
        return rc;
    }
    rc = ieee802154_nl_fill_phy(msg, (*info).snd_portid, (*info).snd_seq, 0, phy);
    if rc < 0 {
        nlmsg_free(msg);
        wpan_phy_put(phy);
        return rc;
    }
    wpan_phy_put(phy);
    genlmsg_reply(msg, info)
}

#[repr(C)]
pub struct dump_phy_data {
    pub skb: *mut sk_buff,
    pub cb: *mut netlink_callback,
    pub idx: i32,
    pub s_idx: i32,
}

unsafe fn ieee802154_dump_phy_iter(phy: *mut wpan_phy, data: *mut core::ffi::c_void) -> i32 {
    let data = data as *mut dump_phy_data;
    pr_debug!("%s\n", "ieee802154_dump_phy_iter");
    (*data).idx += 1;
    if (*data).idx - 1 < (*data).s_idx {
        return 0;
    }
    let rc = ieee802154_nl_fill_phy(
        (*data).skb,
        NETLINK_CB((*(*data).cb).skb).portid,
        (*(*data).cb).nlh.nlmsg_seq,
        NLM_F_MULTI,
        phy,
    );
    if rc < 0 {
        (*data).idx -= 1;
        return rc;
    }
    0
}

pub unsafe fn ieee802154_dump_phy(skb: *mut sk_buff, cb: *mut netlink_callback) -> i32 {
    let mut data = dump_phy_data { cb, skb, s_idx: (*cb).args[0], idx: 0 };
    pr_debug!("%s\n", "ieee802154_dump_phy");
    wpan_phy_for_each(ieee802154_dump_phy_iter, &mut data as *mut _ as *mut core::ffi::c_void);
    (*cb).args[0] = data.idx;
    (*skb).len as i32
}

pub unsafe fn ieee802154_add_iface(skb: *mut sk_buff, info: *mut genl_info) -> i32 {
    let mut rc = -ENOBUFS;
    if (*info).attrs[IEEE802154_ATTR_PHY_NAME].is_null() { return -EINVAL; }
    let name = nla_data((*info).attrs[IEEE802154_ATTR_PHY_NAME]) as *const i8;
    if *(name.add(nla_len((*info).attrs[IEEE802154_ATTR_PHY_NAME]) - 1) as *const u8) != 0 { return -EINVAL; }
    let (devname, name_assign_type): (*const i8, u8) = if !(*info).attrs[IEEE802154_ATTR_DEV_NAME].is_null() {
        let n = nla_data((*info).attrs[IEEE802154_ATTR_DEV_NAME]) as *const i8;
        if *(n.add(nla_len((*info).attrs[IEEE802154_ATTR_DEV_NAME]) - 1) as *const u8) != 0 { return -EINVAL; }
        (n, NET_NAME_USER)
    } else { (b"wpan%d\0".as_ptr() as *const i8, NET_NAME_ENUM) };
    if strlen(devname) >= IFNAMSIZ { return -ENAMETOOLONG; }
    let phy = wpan_phy_find(name); if phy.is_null() { return -ENODEV; }
    let msg = ieee802154_nl_new_reply(info, 0, IEEE802154_ADD_IFACE);
    if msg.is_null() { wpan_phy_put(phy); return rc; }
    if !(*info).attrs[IEEE802154_ATTR_HW_ADDR].is_null() && nla_len((*info).attrs[IEEE802154_ATTR_HW_ADDR]) != IEEE802154_ADDR_LEN {
        nlmsg_free(msg); wpan_phy_put(phy); return -EINVAL;
    }
    let mut ty = __IEEE802154_DEV_INVALID;
    if !(*info).attrs[IEEE802154_ATTR_DEV_TYPE].is_null() { ty = nla_get_u8((*info).attrs[IEEE802154_ATTR_DEV_TYPE]) as i32; if ty >= __IEEE802154_DEV_MAX { nlmsg_free(msg); wpan_phy_put(phy); return -EINVAL; } }
    let dev = rdev_add_virtual_intf_deprecated(wpan_phy_to_rdev(phy), devname, name_assign_type, ty);
    if IS_ERR(dev) { rc = PTR_ERR(dev); nlmsg_free(msg); wpan_phy_put(phy); return rc; }
    dev_hold(dev);
    if !(*info).attrs[IEEE802154_ATTR_HW_ADDR].is_null() {
        let mut addr: sockaddr_storage = core::mem::zeroed(); addr.ss_family = ARPHRD_IEEE802154;
        nla_memcpy(addr.__data.as_mut_ptr() as *mut core::ffi::c_void, (*info).attrs[IEEE802154_ATTR_HW_ADDR], IEEE802154_ADDR_LEN);
        rtnl_lock(); rc = dev_set_mac_address(dev, &mut addr, core::ptr::null_mut()); rtnl_unlock();
        if rc != 0 { rtnl_lock(); rdev_del_virtual_intf_deprecated(wpan_phy_to_rdev(phy), dev); dev_put(dev); rtnl_unlock(); nlmsg_free(msg); wpan_phy_put(phy); return rc; }
    }
    if nla_put_string(msg, IEEE802154_ATTR_PHY_NAME, wpan_phy_name(phy)) != 0 || nla_put_string(msg, IEEE802154_ATTR_DEV_NAME, (*dev).name.as_ptr()) != 0 { nlmsg_free(msg); dev_put(dev); wpan_phy_put(phy); return -EMSGSIZE; }
    dev_put(dev); wpan_phy_put(phy); ieee802154_nl_reply(msg, info)
}

pub unsafe fn ieee802154_del_iface(skb: *mut sk_buff, info: *mut genl_info) -> i32 {
    if (*info).attrs[IEEE802154_ATTR_DEV_NAME].is_null() { return -EINVAL; }
    let name = nla_data((*info).attrs[IEEE802154_ATTR_DEV_NAME]) as *const i8;
    if *(name.add(nla_len((*info).attrs[IEEE802154_ATTR_DEV_NAME]) - 1) as *const u8) != 0 { return -EINVAL; }
    let dev = dev_get_by_name(genl_info_net(info), name); if dev.is_null() { return -ENODEV; }
    if (*dev).type != ARPHRD_IEEE802154 { dev_put(dev); return -ENODEV; }
    let phy = (*(*dev).ieee802154_ptr).wpan_phy; BUG_ON(phy.is_null()); get_device(&mut (*phy).dev);
    if !(*info).attrs[IEEE802154_ATTR_PHY_NAME].is_null() { let p = nla_data((*info).attrs[IEEE802154_ATTR_PHY_NAME]) as *const i8; if *(p.add(nla_len((*info).attrs[IEEE802154_ATTR_PHY_NAME]) - 1) as *const u8) != 0 { wpan_phy_put(phy); dev_put(dev); return -EINVAL; } let p2 = wpan_phy_find(p); if p2.is_null() || p2 != phy { if !p2.is_null() { wpan_phy_put(p2); } wpan_phy_put(phy); dev_put(dev); return -EINVAL; } }
    let msg = ieee802154_nl_new_reply(info, 0, IEEE802154_DEL_IFACE); if msg.is_null() { wpan_phy_put(phy); dev_put(dev); return -ENOBUFS; }
    rtnl_lock(); rdev_del_virtual_intf_deprecated(wpan_phy_to_rdev(phy), dev); dev_put(dev); rtnl_unlock();
    if nla_put_string(msg, IEEE802154_ATTR_PHY_NAME, wpan_phy_name(phy)) != 0 || nla_put_string(msg, IEEE802154_ATTR_DEV_NAME, name) != 0 { nlmsg_free(msg); wpan_phy_put(phy); return -EMSGSIZE; }
    wpan_phy_put(phy); ieee802154_nl_reply(msg, info)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
