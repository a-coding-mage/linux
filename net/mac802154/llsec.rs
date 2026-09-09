// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2014 Fraunhofer ITWM
 *
 * Written by:
 * Phoebe Buckheister <phoebe.buckheister@itwm.fraunhofer.de>
 */

// Linux/crypto dependencies and symbols supplied by other translation units

unsafe extern "C" {
    fn llsec_key_put(key: *mut mac802154_llsec_key);
    fn llsec_key_id_equal(a: *const ieee802154_llsec_key_id, b: *const ieee802154_llsec_key_id) -> bool;
}

unsafe fn mac802154_llsec_init(sec: *mut mac802154_llsec) {
    core::ptr::write_bytes(sec, 0, 1);
    core::ptr::write_bytes(core::ptr::addr_of_mut!((*sec).params.default_key_source), 0xff, IEEE802154_ADDR_LEN);
    INIT_LIST_HEAD(core::ptr::addr_of_mut!((*sec).table.security_levels));
    INIT_LIST_HEAD(core::ptr::addr_of_mut!((*sec).table.devices));
    INIT_LIST_HEAD(core::ptr::addr_of_mut!((*sec).table.keys));
    hash_init((*sec).devices_short);
    hash_init((*sec).devices_hw);
    rwlock_init(core::ptr::addr_of_mut!((*sec).lock));
}

unsafe fn mac802154_llsec_destroy(sec: *mut mac802154_llsec) {
    let mut sl = core::ptr::null_mut(); let mut sn = core::ptr::null_mut();
    list_for_each_entry_safe(sl, sn, &(*sec).table.security_levels, list) {
        let msl = container_of(sl, mac802154_llsec_seclevel, level);
        list_del(core::ptr::addr_of_mut!((*sl).list)); kfree_sensitive(msl);
    }
    let mut dev = core::ptr::null_mut(); let mut dn = core::ptr::null_mut();
    list_for_each_entry_safe(dev, dn, &(*sec).table.devices, list) {
        let mdev = container_of(dev, mac802154_llsec_device, dev);
        list_del(core::ptr::addr_of_mut!((*dev).list)); llsec_dev_free(mdev);
    }
    let mut key = core::ptr::null_mut(); let mut kn = core::ptr::null_mut();
    list_for_each_entry_safe(key, kn, &(*sec).table.keys, list) {
        let mkey = container_of((*key).key, mac802154_llsec_key, key);
        list_del(core::ptr::addr_of_mut!((*key).list)); llsec_key_put(mkey); kfree_sensitive(key);
    }
}

unsafe fn mac802154_llsec_get_params(sec: *mut mac802154_llsec, params: *mut ieee802154_llsec_params) -> i32 {
    read_lock_bh(core::ptr::addr_of_mut!((*sec).lock)); *params = (*sec).params; read_unlock_bh(core::ptr::addr_of_mut!((*sec).lock)); 0
}

unsafe fn mac802154_llsec_set_params(sec: *mut mac802154_llsec, params: *const ieee802154_llsec_params, changed: i32) -> i32 {
    write_lock_bh(core::ptr::addr_of_mut!((*sec).lock));
    if changed & IEEE802154_LLSEC_PARAM_ENABLED != 0 { (*sec).params.enabled = (*params).enabled; }
    if changed & IEEE802154_LLSEC_PARAM_FRAME_COUNTER != 0 { (*sec).params.frame_counter = (*params).frame_counter; }
    if changed & IEEE802154_LLSEC_PARAM_OUT_LEVEL != 0 { (*sec).params.out_level = (*params).out_level; }
    if changed & IEEE802154_LLSEC_PARAM_OUT_KEY != 0 { (*sec).params.out_key = (*params).out_key; }
    if changed & IEEE802154_LLSEC_PARAM_KEY_SOURCE != 0 { (*sec).params.default_key_source = (*params).default_key_source; }
    if changed & IEEE802154_LLSEC_PARAM_PAN_ID != 0 { (*sec).params.pan_id = (*params).pan_id; }
    if changed & IEEE802154_LLSEC_PARAM_HWADDR != 0 { (*sec).params.hwaddr = (*params).hwaddr; }
    if changed & IEEE802154_LLSEC_PARAM_COORD_HWADDR != 0 { (*sec).params.coord_hwaddr = (*params).coord_hwaddr; }
    if changed & IEEE802154_LLSEC_PARAM_COORD_SHORTADDR != 0 { (*sec).params.coord_shortaddr = (*params).coord_shortaddr; }
    write_unlock_bh(core::ptr::addr_of_mut!((*sec).lock)); 0
}

unsafe fn llsec_key_alloc(template: *const ieee802154_llsec_key) -> *mut mac802154_llsec_key {
    let authsizes = [4, 8, 16];
    let key = kzalloc_obj::<mac802154_llsec_key>(); if key.is_null() { return core::ptr::null_mut(); }
    kref_init(core::ptr::addr_of_mut!((*key).ref_)); (*key).key = *template;
    for i in 0..(*key).tfm.len() {
        (*key).tfm[i] = crypto_alloc_aead(c"ccm(aes)".as_ptr(), 0, CRYPTO_ALG_ASYNC);
        if IS_ERR((*key).tfm[i]) || crypto_aead_setkey((*key).tfm[i], (*template).key.as_ptr(), IEEE802154_LLSEC_KEY_SIZE) != 0 || crypto_aead_setauthsize((*key).tfm[i], authsizes[i]) != 0 { goto_err_tfm(key, i); return core::ptr::null_mut(); }
    }
    (*key).tfm0 = crypto_alloc_sync_skcipher(c"ctr(aes)".as_ptr(), 0, 0);
    if IS_ERR((*key).tfm0) || crypto_sync_skcipher_setkey((*key).tfm0, (*template).key.as_ptr(), IEEE802154_LLSEC_KEY_SIZE) != 0 { if !(*key).tfm0.is_null() { crypto_free_sync_skcipher((*key).tfm0); } goto_err_tfm(key, (*key).tfm.len()); return core::ptr::null_mut(); }
    key
}

unsafe fn goto_err_tfm(key: *mut mac802154_llsec_key, upto: usize) { for i in 0..upto { if !IS_ERR_OR_NULL((*key).tfm[i]) { crypto_free_aead((*key).tfm[i]); } } kfree_sensitive(key); }

unsafe fn llsec_key_release(ref_: *mut kref) { let key = container_of(ref_, mac802154_llsec_key, ref_); for tfm in (*key).tfm { crypto_free_aead(tfm); } crypto_free_sync_skcipher((*key).tfm0); kfree_sensitive(key); }
unsafe fn llsec_key_get(key: *mut mac802154_llsec_key) -> *mut mac802154_llsec_key { kref_get(core::ptr::addr_of_mut!((*key).ref_)); key }

unsafe fn llsec_dev_free(dev: *mut mac802154_llsec_device) { let mut p=core::ptr::null_mut(); let mut n=core::ptr::null_mut(); list_for_each_entry_safe(p,n,&(*dev).dev.keys,list) { let d=container_of(p,mac802154_llsec_device_key,devkey); list_del(core::ptr::addr_of_mut!((*p).list)); kfree_sensitive(d); } kfree_sensitive(dev); }

unsafe fn llsec_dev_use_shortaddr(short_addr: __le16) -> bool { short_addr != cpu_to_le16(IEEE802154_ADDR_UNDEF) && short_addr != cpu_to_le16(0xffff) }
unsafe fn llsec_dev_hash_short(short_addr: __le16, pan_id: __le16) -> u32 { ((__force_u16(short_addr) as u32)<<16) | __force_u16(pan_id) as u32 }
unsafe fn llsec_dev_hash_long(hwaddr: __le64) -> u64 { __force_u64(hwaddr) }

unsafe fn llsec_key_id_equal(a: *const ieee802154_llsec_key_id, b: *const ieee802154_llsec_key_id) -> bool {
    if (*a).mode != (*b).mode { return false; }
    if (*a).mode == IEEE802154_SCF_KEY_IMPLICIT { return ieee802154_addr_equal(&(*a).device_addr, &(*b).device_addr); }
    if (*a).id != (*b).id { return false; }
    match (*a).mode { IEEE802154_SCF_KEY_INDEX => true, IEEE802154_SCF_KEY_SHORT_INDEX => (*a).short_source == (*b).short_source, IEEE802154_SCF_KEY_HW_INDEX => (*a).extended_source == (*b).extended_source, _ => false }
}

unsafe fn mac802154_llsec_key_add(sec:*mut mac802154_llsec,id:*const ieee802154_llsec_key_id,key:*const ieee802154_llsec_key)->i32 {
    if ((*key).frame_types & (1<<IEEE802154_FC_TYPE_MAC_CMD)) == 0 && (*key).cmd_frame_ids != 0 { return -EINVAL; }
    let mut pos=core::ptr::null_mut(); list_for_each_entry(pos,&(*sec).table.keys,list) { if llsec_key_id_equal(&(*pos).id,id) { return -EEXIST; } }
    let new=kzalloc_obj::<ieee802154_llsec_key_entry>(); if new.is_null(){return -ENOMEM;} let mkey=llsec_key_alloc(key); if mkey.is_null(){kfree_sensitive(new);return -ENOMEM;} (*new).id=*id; (*new).key=&mut (*mkey).key; list_add_rcu(&mut (*new).list,&mut (*sec).table.keys); 0
}
unsafe fn mac802154_llsec_key_del(sec:*mut mac802154_llsec,key:*const ieee802154_llsec_key_id)->i32 { let mut p=core::ptr::null_mut(); list_for_each_entry(p,&(*sec).table.keys,list){if llsec_key_id_equal(&(*p).id,key){list_del_rcu(&mut (*p).list);return 0;}} -ENOENT }

unsafe fn mac802154_llsec_dev_add(_sec:*mut mac802154_llsec,_dev:*const ieee802154_llsec_device)->i32 { -ENOSYS }
unsafe fn mac802154_llsec_dev_del(_sec:*mut mac802154_llsec,_addr:__le64)->i32 { -ENOSYS }
unsafe fn mac802154_llsec_devkey_add(_sec:*mut mac802154_llsec,_addr:__le64,_key:*const ieee802154_llsec_device_key)->i32 { -ENOSYS }
unsafe fn mac802154_llsec_devkey_del(_sec:*mut mac802154_llsec,_addr:__le64,_key:*const ieee802154_llsec_device_key)->i32 { -ENOSYS }
unsafe fn mac802154_llsec_seclevel_add(_sec:*mut mac802154_llsec,_sl:*const ieee802154_llsec_seclevel)->i32 { -ENOSYS }
unsafe fn mac802154_llsec_seclevel_del(_sec:*mut mac802154_llsec,_sl:*const ieee802154_llsec_seclevel)->i32 { -ENOSYS }
unsafe fn mac802154_llsec_encrypt(_sec:*mut mac802154_llsec,_skb:*mut sk_buff)->i32 { -ENOSYS }
unsafe fn mac802154_llsec_decrypt(_sec:*mut mac802154_llsec,_skb:*mut sk_buff)->i32 { -ENOSYS }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
