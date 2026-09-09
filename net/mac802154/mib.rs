// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2007-2012 Siemens AG
 *
 * Written by:
 * Dmitry Eremin-Solenikov <dbaryshkov@gmail.com>
 * Sergey Lapin <slapin@ossfans.org>
 * Maxim Gorbachyov <maxim.gorbachev@siemens.com>
 * Alexander Smirnov <alex.bluesman.smirnov@gmail.com>
 */

// Dependencies supplied by the surrounding kernel translation.

pub unsafe fn mac802154_dev_set_page_channel(
    dev: *mut net_device,
    page: u8,
    chan: u8,
) {
    let sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    let local = (*sdata).local;
    let res: i32;

    ASSERT_RTNL();

    BUG_ON((*dev).type_ != ARPHRD_IEEE802154);

    res = drv_set_channel(local, page, chan);
    if res != 0 {
        pr_debug!("set_channel failed\n");
    } else {
        (*(*local).phy).current_channel = chan;
        (*(*local).phy).current_page = page;
    }
}

pub unsafe fn mac802154_get_params(
    dev: *mut net_device,
    params: *mut ieee802154_llsec_params,
) -> i32 {
    let sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    let res: i32;

    BUG_ON((*dev).type_ != ARPHRD_IEEE802154);

    mutex_lock(&mut (*sdata).sec_mtx);
    res = mac802154_llsec_get_params(&mut (*sdata).sec, params);
    mutex_unlock(&mut (*sdata).sec_mtx);

    res
}

pub unsafe fn mac802154_set_params(
    dev: *mut net_device,
    params: *const ieee802154_llsec_params,
    changed: i32,
) -> i32 {
    let sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    let res: i32;

    BUG_ON((*dev).type_ != ARPHRD_IEEE802154);

    mutex_lock(&mut (*sdata).sec_mtx);
    res = mac802154_llsec_set_params(&mut (*sdata).sec, params, changed);
    mutex_unlock(&mut (*sdata).sec_mtx);

    res
}

pub unsafe fn mac802154_add_key(
    dev: *mut net_device,
    id: *const ieee802154_llsec_key_id,
    key: *const ieee802154_llsec_key,
) -> i32 {
    let sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    let res: i32;

    BUG_ON((*dev).type_ != ARPHRD_IEEE802154);
    mutex_lock(&mut (*sdata).sec_mtx);
    res = mac802154_llsec_key_add(&mut (*sdata).sec, id, key);
    mutex_unlock(&mut (*sdata).sec_mtx);
    res
}

pub unsafe fn mac802154_del_key(dev: *mut net_device, id: *const ieee802154_llsec_key_id) -> i32 {
    let sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    let res: i32;
    BUG_ON((*dev).type_ != ARPHRD_IEEE802154);
    mutex_lock(&mut (*sdata).sec_mtx);
    res = mac802154_llsec_key_del(&mut (*sdata).sec, id);
    mutex_unlock(&mut (*sdata).sec_mtx);
    res
}

pub unsafe fn mac802154_add_dev(dev: *mut net_device, llsec_dev: *const ieee802154_llsec_device) -> i32 {
    let sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    let res: i32;
    BUG_ON((*dev).type_ != ARPHRD_IEEE802154);
    mutex_lock(&mut (*sdata).sec_mtx);
    res = mac802154_llsec_dev_add(&mut (*sdata).sec, llsec_dev);
    mutex_unlock(&mut (*sdata).sec_mtx);
    res
}

pub unsafe fn mac802154_del_dev(dev: *mut net_device, dev_addr: __le64) -> i32 {
    let sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    let res: i32;
    BUG_ON((*dev).type_ != ARPHRD_IEEE802154);
    mutex_lock(&mut (*sdata).sec_mtx);
    res = mac802154_llsec_dev_del(&mut (*sdata).sec, dev_addr);
    mutex_unlock(&mut (*sdata).sec_mtx);
    res
}

pub unsafe fn mac802154_add_devkey(dev: *mut net_device, device_addr: __le64, key: *const ieee802154_llsec_device_key) -> i32 {
    let sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    let res: i32;
    BUG_ON((*dev).type_ != ARPHRD_IEEE802154);
    mutex_lock(&mut (*sdata).sec_mtx);
    res = mac802154_llsec_devkey_add(&mut (*sdata).sec, device_addr, key);
    mutex_unlock(&mut (*sdata).sec_mtx);
    res
}

pub unsafe fn mac802154_del_devkey(dev: *mut net_device, device_addr: __le64, key: *const ieee802154_llsec_device_key) -> i32 {
    let sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    let res: i32;
    BUG_ON((*dev).type_ != ARPHRD_IEEE802154);
    mutex_lock(&mut (*sdata).sec_mtx);
    res = mac802154_llsec_devkey_del(&mut (*sdata).sec, device_addr, key);
    mutex_unlock(&mut (*sdata).sec_mtx);
    res
}

pub unsafe fn mac802154_add_seclevel(dev: *mut net_device, sl: *const ieee802154_llsec_seclevel) -> i32 {
    let sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    let res: i32;
    BUG_ON((*dev).type_ != ARPHRD_IEEE802154);
    mutex_lock(&mut (*sdata).sec_mtx);
    res = mac802154_llsec_seclevel_add(&mut (*sdata).sec, sl);
    mutex_unlock(&mut (*sdata).sec_mtx);
    res
}

pub unsafe fn mac802154_del_seclevel(dev: *mut net_device, sl: *const ieee802154_llsec_seclevel) -> i32 {
    let sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    let res: i32;
    BUG_ON((*dev).type_ != ARPHRD_IEEE802154);
    mutex_lock(&mut (*sdata).sec_mtx);
    res = mac802154_llsec_seclevel_del(&mut (*sdata).sec, sl);
    mutex_unlock(&mut (*sdata).sec_mtx);
    res
}

pub unsafe fn mac802154_lock_table(dev: *mut net_device) {
    let sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    BUG_ON((*dev).type_ != ARPHRD_IEEE802154);
    mutex_lock(&mut (*sdata).sec_mtx);
}

pub unsafe fn mac802154_get_table(dev: *mut net_device, t: *mut *mut ieee802154_llsec_table) {
    let sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    BUG_ON((*dev).type_ != ARPHRD_IEEE802154);
    *t = &mut (*sdata).sec.table;
}

pub unsafe fn mac802154_unlock_table(dev: *mut net_device) {
    let sdata = IEEE802154_DEV_TO_SUB_IF(dev);
    BUG_ON((*dev).type_ != ARPHRD_IEEE802154);
    mutex_unlock(&mut (*sdata).sec_mtx);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
