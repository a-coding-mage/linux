// SPDX-License-Identifier: GPL-2.0-only
/*
 * MAC commands interface
 *
 * Copyright 2007-2012 Siemens AG
 *
 * Written by:
 * Sergey Lapin <slapin@ossfans.org>
 * Dmitry Eremin-Solenikov <dbaryshkov@gmail.com>
 * Alexander Smirnov <alex.bluesman.smirnov@gmail.com>
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn mac802154_mlme_start_req(
    dev: *mut net_device,
    addr: *mut ieee802154_addr,
    channel: u8,
    page: u8,
    _bcn_ord: u8,
    _sf_ord: u8,
    _pan_coord: u8,
    _blx: u8,
    _coord_realign: u8,
) -> i32 {
    let mut params: ieee802154_llsec_params = core::mem::zeroed();
    let mut changed: i32 = 0;

    ASSERT_RTNL!();

    BUG_ON!((*addr).mode != IEEE802154_ADDR_SHORT);

    (*(*dev).ieee802154_ptr).pan_id = (*addr).pan_id;
    (*(*dev).ieee802154_ptr).short_addr = (*addr).short_addr;
    mac802154_dev_set_page_channel(dev, page, channel);

    params.pan_id = (*addr).pan_id;
    changed |= IEEE802154_LLSEC_PARAM_PAN_ID;

    params.hwaddr = ieee802154_devaddr_from_raw((*dev).dev_addr);
    changed |= IEEE802154_LLSEC_PARAM_HWADDR;

    params.coord_hwaddr = params.hwaddr;
    changed |= IEEE802154_LLSEC_PARAM_COORD_HWADDR;

    params.coord_shortaddr = (*addr).short_addr;
    changed |= IEEE802154_LLSEC_PARAM_COORD_SHORTADDR;

    mac802154_set_params(dev, &mut params, changed)
}

unsafe fn mac802154_set_mac_params(
    dev: *mut net_device,
    params: *const ieee802154_mac_params,
) -> i32 {
    let sdata: *mut ieee802154_sub_if_data = IEEE802154_DEV_TO_SUB_IF!(dev);
    let local: *mut ieee802154_local = (*sdata).local;
    let wpan_dev: *mut wpan_dev = &mut (*sdata).wpan_dev;
    let ret: i32;

    ASSERT_RTNL!();

    /* PHY */
    (*(*wpan_dev).wpan_phy).transmit_power = (*params).transmit_power;
    (*(*wpan_dev).wpan_phy).cca = (*params).cca;
    (*(*wpan_dev).wpan_phy).cca_ed_level = (*params).cca_ed_level;

    /* MAC */
    (*wpan_dev).min_be = (*params).min_be;
    (*wpan_dev).max_be = (*params).max_be;
    (*wpan_dev).csma_retries = (*params).csma_retries;
    (*wpan_dev).frame_retries = (*params).frame_retries;
    (*wpan_dev).lbt = (*params).lbt;

    if (*(*local).hw.phy).flags & WPAN_PHY_FLAG_TXPOWER != 0 {
        ret = drv_set_tx_power(local, (*params).transmit_power);
        if ret < 0 { return ret; }
    }

    if (*(*local).hw.phy).flags & WPAN_PHY_FLAG_CCA_MODE != 0 {
        ret = drv_set_cca_mode(local, &(*params).cca);
        if ret < 0 { return ret; }
    }

    if (*(*local).hw.phy).flags & WPAN_PHY_FLAG_CCA_ED_LEVEL != 0 {
        ret = drv_set_cca_ed_level(local, (*params).cca_ed_level);
        if ret < 0 { return ret; }
    }

    0
}

unsafe fn mac802154_get_mac_params(
    dev: *mut net_device,
    params: *mut ieee802154_mac_params,
) {
    let sdata: *mut ieee802154_sub_if_data = IEEE802154_DEV_TO_SUB_IF!(dev);
    let wpan_dev: *mut wpan_dev = &mut (*sdata).wpan_dev;

    ASSERT_RTNL!();

    /* PHY */
    (*params).transmit_power = (*(*wpan_dev).wpan_phy).transmit_power;
    (*params).cca = (*(*wpan_dev).wpan_phy).cca;
    (*params).cca_ed_level = (*(*wpan_dev).wpan_phy).cca_ed_level;

    /* MAC */
    (*params).min_be = (*wpan_dev).min_be;
    (*params).max_be = (*wpan_dev).max_be;
    (*params).csma_retries = (*wpan_dev).csma_retries;
    (*params).frame_retries = (*wpan_dev).frame_retries;
    (*params).lbt = (*wpan_dev).lbt;
}

static mac802154_llsec_ops: ieee802154_llsec_ops = ieee802154_llsec_ops {
    get_params: mac802154_get_params,
    set_params: mac802154_set_params,
    add_key: mac802154_add_key,
    del_key: mac802154_del_key,
    add_dev: mac802154_add_dev,
    del_dev: mac802154_del_dev,
    add_devkey: mac802154_add_devkey,
    del_devkey: mac802154_del_devkey,
    add_seclevel: mac802154_add_seclevel,
    del_seclevel: mac802154_del_seclevel,
    lock_table: mac802154_lock_table,
    get_table: mac802154_get_table,
    unlock_table: mac802154_unlock_table,
};

static mut mac802154_mlme_wpan: ieee802154_mlme_ops = ieee802154_mlme_ops {
    start_req: Some(mac802154_mlme_start_req),
    llsec: &mac802154_llsec_ops,
    set_mac_params: Some(mac802154_set_mac_params),
    get_mac_params: Some(mac802154_get_mac_params),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
