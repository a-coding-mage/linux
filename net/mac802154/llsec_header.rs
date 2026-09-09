/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2014 Fraunhofer ITWM
 *
 * Written by:
 * Phoebe Buckheister <phoebe.buckheister@itwm.fraunhofer.de>
 */

// Dependencies supplied by the surrounding kernel/Rust translation environment.

#[repr(C)]
pub struct mac802154_llsec_key {
    pub key: ieee802154_llsec_key,

    /* one tfm for each authsize (4/8/16) */
    pub tfm: [*mut crypto_aead; 3],
    pub tfm0: *mut crypto_sync_skcipher,

    pub ref_: kref,
}

#[repr(C)]
pub struct mac802154_llsec_device_key {
    pub devkey: ieee802154_llsec_device_key,

    pub rcu: rcu_head,
}

#[repr(C)]
pub struct mac802154_llsec_device {
    pub dev: ieee802154_llsec_device,

    pub bucket_s: hlist_node,
    pub bucket_hw: hlist_node,

    /* protects dev.frame_counter and the elements of dev.keys */
    pub lock: spinlock_t,

    pub rcu: rcu_head,
}

#[repr(C)]
pub struct mac802154_llsec_seclevel {
    pub level: ieee802154_llsec_seclevel,

    pub rcu: rcu_head,
}

#[repr(C)]
pub struct mac802154_llsec {
    pub params: ieee802154_llsec_params,
    pub table: ieee802154_llsec_table,

    pub devices_short: [hlist_head; 1 << 6],
    pub devices_hw: [hlist_head; 1 << 6],

    /* protects params, all other fields are fine with RCU */
    pub lock: rwlock_t,
}

extern "C" {
    pub fn mac802154_llsec_init(sec: *mut mac802154_llsec);
    pub fn mac802154_llsec_destroy(sec: *mut mac802154_llsec);

    pub fn mac802154_llsec_get_params(
        sec: *mut mac802154_llsec,
        params: *mut ieee802154_llsec_params,
    ) -> core::ffi::c_int;
    pub fn mac802154_llsec_set_params(
        sec: *mut mac802154_llsec,
        params: *const ieee802154_llsec_params,
        changed: core::ffi::c_int,
    ) -> core::ffi::c_int;

    pub fn mac802154_llsec_key_add(
        sec: *mut mac802154_llsec,
        id: *const ieee802154_llsec_key_id,
        key: *const ieee802154_llsec_key,
    ) -> core::ffi::c_int;
    pub fn mac802154_llsec_key_del(
        sec: *mut mac802154_llsec,
        key: *const ieee802154_llsec_key_id,
    ) -> core::ffi::c_int;

    pub fn mac802154_llsec_dev_add(
        sec: *mut mac802154_llsec,
        dev: *const ieee802154_llsec_device,
    ) -> core::ffi::c_int;
    pub fn mac802154_llsec_dev_del(
        sec: *mut mac802154_llsec,
        device_addr: __le64,
    ) -> core::ffi::c_int;

    pub fn mac802154_llsec_devkey_add(
        sec: *mut mac802154_llsec,
        dev_addr: __le64,
        key: *const ieee802154_llsec_device_key,
    ) -> core::ffi::c_int;
    pub fn mac802154_llsec_devkey_del(
        sec: *mut mac802154_llsec,
        dev_addr: __le64,
        key: *const ieee802154_llsec_device_key,
    ) -> core::ffi::c_int;

    pub fn mac802154_llsec_seclevel_add(
        sec: *mut mac802154_llsec,
        sl: *const ieee802154_llsec_seclevel,
    ) -> core::ffi::c_int;
    pub fn mac802154_llsec_seclevel_del(
        sec: *mut mac802154_llsec,
        sl: *const ieee802154_llsec_seclevel,
    ) -> core::ffi::c_int;

    pub fn mac802154_llsec_encrypt(sec: *mut mac802154_llsec, skb: *mut sk_buff)
        -> core::ffi::c_int;
    pub fn mac802154_llsec_decrypt(sec: *mut mac802154_llsec, skb: *mut sk_buff)
        -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
